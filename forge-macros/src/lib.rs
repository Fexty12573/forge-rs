use crate::{
    hook::{HookArgs, HookTransformer},
    pure_virtual::{PureVirtualFn, VirtualArgs},
};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, FnArg, ItemFn, parse_macro_input, visit_mut::VisitMut};

fn forge_crate() -> TokenStream2 {
    match crate_name("mhgu-forge") {
        Ok(FoundCrate::Itself) => quote! { crate },
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { ::#ident }
        }
        Err(_) => quote! { ::forge },
    }
}

mod hook;
mod pure_virtual;

/// Marks a function as the plugins entry point.
///
/// ### Example
/// ```ignore
/// #[forge::entry]
/// fn my_main_function() {
///     // Your code here
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let inner = parse_macro_input!(item as ItemFn);
    let inner_name = &inner.sig.ident;

    let expanded = quote! {
        #inner

        #[unsafe(no_mangle)]
        pub extern "C" fn forge_onLoad(params: *mut ::forge::sys::init::PluginInitParams) {
            unsafe {
                (*params).required_version = ::forge::REQUIRED_VERSION;
            }
            #inner_name();
        }
    };

    expanded.into()
}

/// Defines a function hook at a fixed offset from a base address.
///
/// The annotated function becomes a module of the same name that holds the
/// hook's static state and can be installed via [`forge::install_hook!`].
///
/// Inside the body the following pseudo-macros are available:
/// - `original!(args)` — call the original function (zero or more args).
/// - `original!()` — call the original function with no arguments.
/// - `original_function!()` — obtain the raw function pointer without calling it.
/// - `context!(T)` — borrow the context as `&mut T` (requires context variant of install).
/// - `context!()` — obtain the raw `*const c_void` context pointer.
///
/// ### Example
/// ```ignore
/// #[forge::hook(offset = 0x1234)]
/// fn my_hook(param: u32) -> u32 {
///     let result = original!(param);
///     result * 2
/// }
///
/// // Context must outlive the hook (static or Box::leak with "allocator" feature).
/// static mut MULTIPLIER: u32 = 2;
///
/// #[forge::hook(offset = 0x5678)]
/// fn ctx_hook(value: u32) -> u32 {
///     let m = context!(u32);
///     original!(value) * *m
/// }
///
/// #[forge::entry]
/// fn main() {
///     let base = forge::mem::text_addr();
///     forge::install_hook!(base, my_hook);
///     forge::install_hook!(base, ctx_hook, unsafe { &raw mut MULTIPLIER });
/// }
/// ```
#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as HookArgs);
    let func = parse_macro_input!(item as ItemFn);

    let offset = &args.offset;
    let func_name = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let output = &func.sig.output;

    let param_types: Vec<TokenStream2> = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let ty = &pat_type.ty;
                quote! { #ty }
            }
            FnArg::Receiver(_) => {
                panic!("#[forge::hook] does not support `self` parameters")
            }
        })
        .collect();

    let ret_type = match output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let fn_ptr_type = quote! { unsafe extern "C" fn(#(#param_types),*) -> #ret_type };

    let mut body = func.block.clone();
    HookTransformer.visit_block_mut(&mut body);

    let expanded = quote! {
        pub mod #func_name {
            #[allow(unused_imports)]
            use super::*;

            /// Offset of the hook target from the base address supplied to `install_hook!`.
            pub const OFFSET: u32 = #offset as u32;

            static mut __HOOK: ::core::mem::MaybeUninit<::forge::sys::hook::Hook> =
                ::core::mem::MaybeUninit::uninit();
            static mut __ORIGINAL: *const ::core::ffi::c_void = ::core::ptr::null();

            pub unsafe extern "C" fn __detour(#inputs) #output {
                let __forge_original: #fn_ptr_type = unsafe {
                    ::core::mem::transmute(__ORIGINAL)
                };
                let __forge_context = unsafe { ::forge::sys::hook::forge_hook_getContext() };
                #body
            }

            pub unsafe fn __install(base: u32) {
                unsafe {
                    __HOOK.write(::forge::sys::hook::forge_hook_create(
                        (base + OFFSET) as *const ::core::ffi::c_void,
                        __detour as *const ::core::ffi::c_void,
                        ::core::ptr::addr_of_mut!(__ORIGINAL),
                    ));
                }
            }

            pub unsafe fn __install_with_ctx(base: u32, ctx: *const ::core::ffi::c_void) {
                unsafe {
                    __HOOK.write(::forge::sys::hook::forge_hook_createWithContext(
                        (base + OFFSET) as *const ::core::ffi::c_void,
                        __detour as *const ::core::ffi::c_void,
                        ::core::ptr::addr_of_mut!(__ORIGINAL),
                        ctx,
                    ));
                }
            }

            /// Update the context pointer for an already-installed hook.
            pub unsafe fn __update_ctx(ctx: *const ::core::ffi::c_void) {
                unsafe {
                    let result = ::forge::sys::hook::forge_hook_updateContext(
                        __HOOK.as_mut_ptr(),
                        ctx,
                    );
                    debug_assert_eq!(result, 0, "forge_hook_updateContext failed");
                }
            }
        }
    };

    expanded.into()
}

/// Marks a method as a virtual function at a given index in the vtable.
/// Methods marked with this must be part of a type that implements `HasVtable`, and must take `&self` or `&mut self` as the first parameter.
/// The method body is replaced with a call to the function pointer at the specified index in the vtable
///
/// ### Example
/// ```ignore
/// #[derive(forge::HasVtable)]
/// pub struct MyStruct;
///
/// impl MyStruct {
///     #[forge::pure_virtual(3)]
///     pub fn my_virtual_func(&self) -> i32 {}
/// }
/// ```
/// Note the lack of an actual implementation of the function.
#[proc_macro_attribute]
pub fn pure_virtual(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as VirtualArgs);
    let func = parse_macro_input!(item as PureVirtualFn);

    let func_name = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let output = &func.sig.output;

    let has_self = !inputs.is_empty()
        && match &inputs[0] {
            FnArg::Receiver(receiver) => receiver.reference.is_some(),
            _ => false,
        };

    if !has_self {
        panic!("Functions marked with #[forge::pure_virtual] must have `&self` as their first parameter");
    }

    let param_types: Vec<TokenStream2> = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(receiver) => {
                let ty = &receiver.ty;
                quote! { #ty }
            }
            FnArg::Typed(pat_type) => {
                let ty = &pat_type.ty;
                quote! { #ty }
            }
        })
        .collect();

    let ret_type = match output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let fn_ptr_type = quote! { unsafe extern "C" fn(#(#param_types),*) -> #ret_type };

    let index = &args.index;
    let param_names: Vec<TokenStream2> = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => quote! { self },
            FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                quote! { #pat }
            }
        })
        .collect();

    let forge = forge_crate();
    let expanded = quote! {
        pub fn #func_name(#inputs) #output {
            let func: #fn_ptr_type = unsafe {
                ::core::mem::transmute(#forge::sys::cpp::HasVtable::vtable_ptr(self).add(#index))
            };
            unsafe { func(#(#param_names),*) }
        }
    };

    expanded.into()
}

#[proc_macro_derive(HasVtable)]
pub fn has_vtable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;
    let forge = forge_crate();

    let expanded = quote! {
        impl #forge::sys::cpp::HasVtable for #type_name {
            fn vtable_ptr(&self) -> *const *const ::core::ffi::c_void {
                unsafe {
                    let ptr = self as *const Self as *const *const *const ::core::ffi::c_void;
                    *ptr
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Object)]
pub fn mt_object_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;
    let forge = forge_crate();

    let expanded = quote! {
        impl #forge::sys::cpp::HasVtable for #type_name {
            fn vtable_ptr(&self) -> *const *const ::core::ffi::c_void {
                unsafe {
                    let ptr = self as *const Self as *const *const *const ::core::ffi::c_void;
                    *ptr
                }
            }
        }

        impl #forge::mt::object::Object for #type_name {}
    };

    expanded.into()
}
