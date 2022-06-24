use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{fold::Fold, Token};

use crate::utils::{delocal_ident, local_ident, macro_ident_expr};

pub fn forward_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);

    let mut visitor = FVisitor::new();
    let block = visitor.fold_block(input);

    visitor.delocal_check();

    let mut output = TokenStream::new();

    let brace = syn::token::Brace::default();

    brace.surround(&mut output, |output| block.to_tokens(output));

    proc_macro::TokenStream::from(output)
}

struct FVisitor {
    pub delocal_list: Vec<syn::Ident>,
    level: u8,
}

impl FVisitor {
    fn new() -> Self {
        FVisitor {
            delocal_list: Vec::default(),
            level: 0,
        }
    }

    fn fwd_stmt(&mut self, node: syn::Stmt) -> syn::Stmt {
        match node {
            syn::Stmt::Local(l) => self.local(l),
            syn::Stmt::Item(_) => todo!(),
            syn::Stmt::Expr(e) => self.expr(e),
            syn::Stmt::Semi(e, s) => self.semi(e, s),
        }
    }

    fn local(&mut self, local: syn::Local) -> syn::Stmt {
        let i = local_ident(&local);
        self.delocal_list.push(i);
        syn::Stmt::Local(local)
    }

    fn expr(&mut self, expr: syn::Expr) -> syn::Stmt {
        self.delocal(&expr);
        syn::Stmt::Expr(fwd_expr(self.fold_expr(expr)))
    }

    fn semi(&mut self, expr: syn::Expr, semi: Token![;]) -> syn::Stmt {
        self.delocal(&expr);
        syn::Stmt::Semi(fwd_expr(self.fold_expr(expr)), semi)
    }

    fn delocal(&mut self, expr: &syn::Expr) {
        if let Some(i) = macro_ident_expr(expr) {
            let delocal: syn::Ident = syn::parse_quote!{ delocal };
            if i == delocal {
                let di = delocal_ident(expr).unwrap();
                if let Some(index) = self.delocal_list.iter().position(|l| *l == di) {
                    self.delocal_list.remove(index);
                } else {
                    panic!("Attempt to delocal a non local variable: {}", di);
                }
            }
        }
    }

    fn delocal_check(&self) {
        if !self.delocal_list.is_empty() {
            let ident_list = self
                .delocal_list
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>();
            panic!(
                "The following local(s) need to be consumed by delocal!: {:?}",
                ident_list
            );
        }
    }
}

fn fwd_expr(expr: syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::AssignOp(syn::ExprAssignOp {
            attrs,
            left,
            op,
            right,
        }) => {
            let cmp: syn::Stmt = syn::parse_quote! {
                if core::ptr::eq(&(#left), &(#right)) {
                    panic!("{}:{}: Lefthand and righthand are aliases of each other", file!(), line!());
                }
            };

            let aop = syn::Expr::AssignOp(syn::ExprAssignOp {
                attrs,
                left: left.clone(),
                op,
                right: right.clone(),
            });

            let block: syn::ExprBlock = syn::parse_quote! {
                {
                    stringify!(#left, #op, #right);
                    #cmp
                    #aop
                }
            };
            syn::Expr::Block(block)
        }
        _ => expr,
    }
}

impl syn::fold::Fold for FVisitor {
    fn fold_stmt(&mut self, node: syn::Stmt) -> syn::Stmt {
        self.fwd_stmt(node)
    }

    fn fold_block(&mut self, mut block: syn::Block) -> syn::Block {
        let mut block_visitor = FVisitor::new();

        block_visitor.level = self.level + 1;
        block.stmts.iter_mut().for_each(|n| {
            *n = block_visitor.fold_stmt(n.clone());
        });

        block_visitor.delocal_check();

        block
    }
}
