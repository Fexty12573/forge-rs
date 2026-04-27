use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, FnArg, ItemFn, LitInt, Token, parse::ParseStream, parse_macro_input, visit_mut::VisitMut};

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

struct HookArgs {
    offset: LitInt,
}

impl syn::parse::Parse for HookArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if ident != "offset" {
            return Err(syn::Error::new(ident.span(), "expected `offset`"));
        }
        let _: Token![=] = input.parse()?;
        let offset: LitInt = input.parse()?;
        Ok(HookArgs { offset })
    }
}

/// Transforms `original!(args)` -> `__forge_original(args)` and
/// `context!(T)` -> `&mut *(__forge_context as *mut T)` in hook bodies.
struct HookTransformer;

impl VisitMut for HookTransformer {
    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        if let syn::Stmt::Macro(stmt_mac) = stmt {
            let is_magic = stmt_mac.mac.path.is_ident("original")
                || stmt_mac.mac.path.is_ident("original_function")
                || stmt_mac.mac.path.is_ident("context");
            if is_magic {
                let semi = stmt_mac.semi_token;
                let as_expr = syn::Expr::Macro(syn::ExprMacro {
                    attrs: stmt_mac.attrs.clone(),
                    mac: stmt_mac.mac.clone(),
                });
                *stmt = syn::Stmt::Expr(as_expr, semi);
            }
        }
        syn::visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Macro(expr_mac) = &*expr {
            let mac = &expr_mac.mac;

            if mac.path.is_ident("original") {
                // original!() or original!(args) -> call original
                let tokens = mac.tokens.clone();
                *expr = syn::parse_quote!(__forge_original(#tokens));
                return;
            }

            if mac.path.is_ident("original_function") {
                // original_function!() -> raw function pointer
                *expr = syn::parse_quote!(__forge_original);
                return;
            }

            if mac.path.is_ident("context") {
                let tokens = mac.tokens.clone();
                if tokens.is_empty() {
                    // context!() -> raw *const c_void
                    *expr = syn::parse_quote!(__forge_context);
                } else {
                    // context!(SomeType) -> &mut SomeType
                    *expr = syn::parse_quote!(unsafe { &mut *(__forge_context as *mut #tokens) });
                }
                return;
            }
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }
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
