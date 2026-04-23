use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

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
        pub extern "C" fn forge_onLoad() {
            #inner_name();
        }
    };

    expanded.into()
}
