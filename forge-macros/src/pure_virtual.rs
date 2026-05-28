use syn::{Ident, LitInt, Token, parse::ParseStream};

pub struct PureVirtualFn {
    pub(crate) vis: syn::Visibility,
    pub(crate) sig: syn::Signature,
}

impl syn::parse::Parse for PureVirtualFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Attribute::parse_outer(input)?;
        let vis: syn::Visibility = input.parse()?;
        let sig: syn::Signature = input.parse()?;
        if input.peek(Token![;]) {
            let _: Token![;] = input.parse()?;
        } else {
            let _body: syn::Block = input.parse()?;
        }
        Ok(PureVirtualFn { vis, sig })
    }
}

pub struct VirtualArgs {
    pub(crate) index: LitInt,
}

impl syn::parse::Parse for VirtualArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            if ident != "index" {
                return Err(syn::Error::new(ident.span(), "expected `index`"));
            }

            let _: Token![=] = input.parse()?;
            let index: LitInt = input.parse()?;
            Ok(VirtualArgs { index })
        } else if lookahead.peek(LitInt) {
            let index: LitInt = input.parse()?;
            Ok(VirtualArgs { index })
        } else {
            Err(syn::Error::new(input.span(), "expected `index` or literal"))
        }
    }
}
