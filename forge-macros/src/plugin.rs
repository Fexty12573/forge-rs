use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ImplItem, ItemImpl, parse_quote, spanned::Spanned};

/// Hooks `#[forge::plugin]` exports when the impl defines them: (method, symbol, params, args).
fn hooks() -> Vec<(&'static str, &'static str, TokenStream, TokenStream)> {
    #[allow(unused_mut)]
    let mut hooks = vec![("on_update", "forge_onUpdate", quote! { dt: f32 }, quote! { dt })];

    #[cfg(feature = "nvn")]
    hooks.extend([
        ("on_nvn_ready", "forge_onNvnReady", quote! {}, quote! {}),
        (
            "on_nvn_render",
            "forge_onNvnRender",
            quote! { window: *mut ::forge::sys::nvn::types::NVNwindow, texture_index: i32 },
            quote! { window, texture_index },
        ),
    ]);

    #[cfg(feature = "imgui")]
    hooks.extend([
        ("on_imgui_render", "forge_onImGuiRender", quote! {}, quote! {}),
        ("on_imgui_free_render", "forge_onImGuiFreeRender", quote! {}, quote! {}),
    ]);

    hooks
}

pub fn plugin(mut item: ItemImpl) -> syn::Result<TokenStream> {
    if item.trait_.is_none() {
        return Err(syn::Error::new(
            item.self_ty.span(),
            "#[forge::plugin] must be placed on `impl forge::Plugin for YourType`",
        ));
    }

    if !item.generics.params.is_empty() {
        return Err(syn::Error::new(
            item.generics.span(),
            "#[forge::plugin] does not support generic plugin types",
        ));
    }

    let forge = crate::forge_crate();
    let ty = &item.self_ty;

    let defined: Vec<String> = item
        .items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Fn(f) => Some(f.sig.ident.to_string()),
            _ => None,
        })
        .collect();

    if defined.iter().any(|name| name == "__instance") {
        return Err(syn::Error::new(ty.span(), "`__instance` is implemented by #[forge::plugin]"));
    }

    item.items.push(parse_quote! {
        fn __instance() -> &'static #forge::plugin::__private::Instance<Self> {
            static INSTANCE: #forge::plugin::__private::Instance<#ty> = #forge::plugin::__private::Instance::new();
            &INSTANCE
        }
    });

    let exports = hooks()
        .into_iter()
        .filter(|(method, ..)| defined.iter().any(|d| d == method))
        .map(|(method, symbol, params, args)| {
            let method = format_ident!("{method}");
            let symbol = format_ident!("{symbol}");
            quote! {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #symbol(#params) {
                    unsafe { #forge::plugin::__private::#method::<#ty>(#args) }
                }
            }
        });

    Ok(quote! {
        #item

        const _: () = {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn forge_onLoad(params: *mut #forge::sys::init::PluginInitParams) {
                unsafe { #forge::plugin::__private::on_load::<#ty>(params) }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn forge_onInit() {
                #forge::plugin::__private::on_init::<#ty>()
            }

            #(#exports)*
        };
    })
}
