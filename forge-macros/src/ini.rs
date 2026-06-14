use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, Type};

use crate::forge_crate;

/// Per-field metadata resolved from `#[section]` / `#[alias]` attributes.
struct FieldMeta<'a> {
    ident: &'a syn::Ident,
    ty: &'a Type,
    /// The persistent section this field belongs to (joined with the parent base).
    section: String,
    /// The property key (scalar) or sub-section name (nested struct).
    key: String,
}

/// Walks the struct's named fields, tracking the persistent `#[section(..)]` and
/// resolving each field's `#[alias(..)]` override. Returns the per-field metadata
/// shared by both derives, or a compile-error token stream on a malformed struct.
fn collect_fields<'a>(
    input: &'a DeriveInput,
    derive: &str,
    errors: &mut Vec<TokenStream2>,
) -> Result<Vec<FieldMeta<'a>>, TokenStream2> {
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    format!("#[derive({derive})] requires a struct with named fields"),
                )
                .to_compile_error());
            }
        },
        _ => {
            return Err(
                syn::Error::new_spanned(&input.ident, format!("#[derive({derive})] can only be derived for structs"))
                    .to_compile_error(),
            );
        }
    };

    let mut current_section = String::new();
    let mut metas = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().expect("named field");

        // `alias` renames the field's property key (scalar) or sub-section (nested
        // struct). `section` updates the persistent section for this and all
        // following fields.
        let mut rename: Option<String> = None;
        for attr in &field.attrs {
            if attr.path().is_ident("section") {
                match attr.parse_args::<LitStr>() {
                    Ok(lit) => current_section = lit.value(),
                    Err(err) => errors.push(err.to_compile_error()),
                }
            } else if attr.path().is_ident("alias") {
                match attr.parse_args::<LitStr>() {
                    Ok(lit) => rename = Some(lit.value()),
                    Err(err) => errors.push(err.to_compile_error()),
                }
            }
        }

        metas.push(FieldMeta {
            ident,
            ty: &field.ty,
            section: current_section.clone(),
            key: rename.unwrap_or_else(|| ident.to_string()),
        });
    }

    Ok(metas)
}

pub fn ini_deserialize_derive(input: DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let forge = forge_crate();

    let mut errors = Vec::new();
    let metas = match collect_fields(&input, "IniDeserialize", &mut errors) {
        Ok(metas) => metas,
        Err(err) => return err,
    };

    let field_inits = metas.iter().map(|FieldMeta { ident, ty, section, key }| {
        quote! {
            #ident: {
                let __section = #forge::config::__private::join(__base, #section);
                #[allow(unused_imports)]
                use #forge::config::__private::{IniNestedField as _, IniScalarField as _};
                (&#forge::config::__private::Probe::<#ty>(::core::marker::PhantomData))
                    .__ini_read(__ini, &__section, #key)
            },
        }
    });

    quote! {
        impl #forge::config::IniDeserialize for #name {
            fn from_ini_section(
                __ini: &#forge::config::ini::Ini,
                __base: &str,
            ) -> Self {
                #(#errors)*
                Self { #(#field_inits)* }
            }
        }
    }
}

pub fn ini_serialize_derive(input: DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let forge = forge_crate();

    let mut errors = Vec::new();
    let metas = match collect_fields(&input, "IniSerialize", &mut errors) {
        Ok(metas) => metas,
        Err(err) => return err,
    };

    let field_writes = metas.iter().map(|FieldMeta { ident, ty, section, key }| {
        quote! {
            {
                let __section = #forge::config::__private::join(__base, #section);
                #[allow(unused_imports)]
                use #forge::config::__private::{IniNestedWrite as _, IniScalarWrite as _};
                (&#forge::config::__private::Probe::<#ty>(::core::marker::PhantomData))
                    .__ini_write(&self.#ident, __ini, &__section, #key);
            }
        }
    });

    quote! {
        impl #forge::config::IniSerialize for #name {
            fn to_ini_section(&self, __ini: &mut #forge::config::ini::Ini, __base: &str) {
                #(#errors)*
                #(#field_writes)*
            }
        }
    }
}
