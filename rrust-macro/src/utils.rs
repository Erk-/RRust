use syn::parse::Parser;

pub fn local_ident(local: &syn::Local) -> syn::Ident {
    match &local.pat {
        syn::Pat::Ident(pi) => pi.ident.clone(),
        _ => panic!("get_ident: Not implemented: {:?}", local.pat),
    }
}

pub fn macro_ident_expr(expr: &syn::Expr) -> Option<syn::Ident> {
    match expr {
        syn::Expr::Macro(syn::ExprMacro { attrs: _, mac }) => mac.path.get_ident().cloned(),
        _ => None,
    }
}

pub fn delocal_ident(expr: &syn::Expr) -> Option<syn::Ident> {
    let punct: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> = match expr {
        syn::Expr::Macro(syn::ExprMacro { attrs: _, mac }) => {
            (|input: &syn::parse::ParseBuffer| syn::punctuated::Punctuated::parse_terminated(input))
                .parse2(mac.tokens.clone())
                .unwrap()
        }
        _ => panic!(),
    };

    let name = punct.first().unwrap();
    let ident = match name {
        syn::Expr::Path(syn::ExprPath {
            attrs: _,
            qself: _,
            path,
        }) => path.get_ident().cloned(),
        _ => None,
    };
    ident
}

