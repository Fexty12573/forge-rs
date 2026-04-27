use syn::{Expr, LitInt, Token, parse::ParseStream, visit_mut::VisitMut};

pub struct HookArgs {
    pub(crate) offset: LitInt,
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
pub struct HookTransformer;

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
