use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{fold::Fold, parse::Parser};

use crate::utils::{delocal_ident, local_ident, macro_ident_expr};

pub fn reverse_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);

    let mut visitor = RVisitor::new();
    let block = visitor.fold_block(input);

    visitor.delocal_check();

    let mut output = TokenStream::new();

    let brace = syn::token::Brace::default();

    brace.surround(&mut output, |output| block.to_tokens(output));

    proc_macro::TokenStream::from(output)
}

#[derive(Default)]
struct RVisitor {
    pub delocal_list: Vec<syn::Ident>,
}

impl RVisitor {
    pub fn new() -> Self {
        RVisitor::default()
    }

    fn reverse_stmt(&mut self, node: syn::Stmt) -> syn::Stmt {
        match node {
            syn::Stmt::Local(l) => self.local(l),
            syn::Stmt::Item(_) => panic!("Not yet implemented: Stmt::Item"),
            syn::Stmt::Expr(e) => self.expr(e),
            syn::Stmt::Semi(e, s) => self.semi(e, s),
        }
    }

    fn local(&mut self, local: syn::Local) -> syn::Stmt {
        let i = local_ident(&local);
        let expr = local.init.unwrap().1;
        self.delocal_list.push(i.clone());
        let m: syn::Stmt = syn::parse_quote! {
            ::rrust::delocal!(#i, #expr);
        };
        m
    }

    fn expr(&mut self, expr: syn::Expr) -> syn::Stmt {
        let (b, expr) = self.delocal(expr);
        if b {
            syn::Stmt::Expr(expr)
        } else {
            syn::Stmt::Expr(reverse_expr(self.fold_expr(expr)))
        }
    }

    fn semi(&mut self, expr: syn::Expr, semi: syn::Token![;]) -> syn::Stmt {
        let (b, expr) = self.delocal(expr);
        if b {
            syn::Stmt::Semi(expr, semi)
        } else {
            syn::Stmt::Semi(reverse_expr(self.fold_expr(expr)), semi)
        }
    }

    fn delocal(&mut self, expr: syn::Expr) -> (bool, syn::Expr) {
        if let Some(i) = macro_ident_expr(&expr) {
            let delocal: syn::Ident = syn::parse_str("delocal").unwrap();
            if i == delocal {
                let di = delocal_ident(&expr).unwrap();
                if let Some(index) = self.delocal_list.iter().position(|l| *l == di) {
                    self.delocal_list.remove(index);
                    return (true, delocal_val(expr));
                } else {
                    panic!("Attempt to delocal a non local variable: {}", di);
                }
            }
        }
        (false, expr)
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

impl Fold for RVisitor {
    fn fold_stmt(&mut self, node: syn::Stmt) -> syn::Stmt {
        self.reverse_stmt(node)
    }

    fn fold_block(&mut self, mut block: syn::Block) -> syn::Block {
        let mut block_visitor = RVisitor::new();

        block.stmts.iter_mut().for_each(|n| {
            *n = block_visitor.fold_stmt(n.clone());
        });
        block.stmts.reverse();

        block_visitor.delocal_check();

        block
    }
}

pub fn delocal_val(expr: syn::Expr) -> syn::Expr {
    let punct: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> = match expr {
        syn::Expr::Macro(syn::ExprMacro { attrs: _, mac }) => {
            (|input: &syn::parse::ParseBuffer| syn::punctuated::Punctuated::parse_terminated(input))
                .parse2(mac.tokens)
                .unwrap()
        }
        _ => panic!(),
    };
    let name = punct.first().unwrap();
    let val = punct.last().unwrap();
    syn::parse_quote! {
        let mut #name = #val
    }
}

use syn::{BinOp, Expr, ExprAssignOp, ExprMacro};

fn reverse_bin_op(bin_op: BinOp) -> BinOp {
    match bin_op {
        BinOp::Add(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Sub(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Mul(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Div(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Rem(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::And(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Or(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::BitXor(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::BitAnd(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::BitOr(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Shl(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Shr(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Eq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Lt(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Le(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Ne(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Ge(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::Gt(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::AddEq(_) => BinOp::SubEq(syn::token::SubEq::default()),
        BinOp::SubEq(_) => BinOp::AddEq(syn::token::AddEq::default()),
        BinOp::MulEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::DivEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::RemEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::BitXorEq(x) => BinOp::BitXorEq(x),
        BinOp::BitAndEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::BitOrEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::ShlEq(_) => panic!("disallowed binary operator. {}", line!()),
        BinOp::ShrEq(_) => panic!("disallowed binary operator. {}", line!()),
    }
}

fn reverse_expr(e: Expr) -> Expr {
    match e {
        Expr::Array(_) => panic!("Not yet implemented {}", line!()),
        Expr::Assign(_) => panic!("Not yet implemented {}", line!()),
        Expr::AssignOp(ExprAssignOp {
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

            let aop = Expr::AssignOp(ExprAssignOp {
                attrs,
                left,
                op: reverse_bin_op(op),
                right,
            });

            let block: syn::ExprBlock = syn::parse_quote! {
                {
                    #cmp
                    #aop
                }
            };
            Expr::Block(block)
        }
        Expr::Async(_) => panic!("Not yet implemented {}", line!()),
        Expr::Await(_) => panic!("Not yet implemented {}", line!()),
        Expr::Binary(_) => panic!("Not yet implemented {}", line!()),
        Expr::Block(b) => syn::Expr::Block(b),
        Expr::Box(_) => panic!("Not yet implemented {}", line!()),
        Expr::Break(_) => panic!("Not yet implemented {}", line!()),
        Expr::Call(mut c) => {
            let func = *c.func.clone();
            if let Expr::Path(mut f) = func {
                if let Some(last) = f.path.segments.pop() {
                    let forward: syn::PathSegment = syn::parse_str("forward").unwrap();
                    if last.value().clone() == forward {
                        let backwards: syn::PathSegment = syn::parse_str("backwards").unwrap();
                        f.path.segments.push(backwards);
                        c.func = Box::new(Expr::Path(f));
                    }
                }
            }
            Expr::Call(c)
        }
        Expr::Cast(_) => panic!("Not yet implemented {}", line!()),
        Expr::Closure(_) => panic!("Not yet implemented {}", line!()),
        Expr::Continue(_) => panic!("Not yet implemented {}", line!()),
        Expr::Field(_) => panic!("Not yet implemented {}", line!()),
        Expr::ForLoop(_) => panic!("Not yet implemented {}", line!()),
        Expr::Group(_) => panic!("Not yet implemented {}", line!()),
        Expr::If(_) => panic!("Not yet implemented {}", line!()),
        Expr::Index(_) => panic!("Not yet implemented {}", line!()),
        Expr::Let(_) => panic!("Not yet implemented {}", line!()),
        Expr::Lit(_) => panic!("Not yet implemented {}", line!()),
        Expr::Loop(_) => panic!("Not yet implemented {}", line!()),
        Expr::Macro(ExprMacro { attrs, mac }) => {
            let mut cmac = mac.clone();
            if let Some(i) = mac.path.get_ident() {
                let rif: syn::Ident = syn::parse_str("rif").unwrap();
                let rloop: syn::Ident = syn::parse_str("rloop").unwrap();
                let ic = i.clone();
                if ic == rif {
                    let t: syn::Path = syn::parse_str("::rrust::_reverse_rif").unwrap();
                    cmac.path = t;
                } else if ic == rloop {
                    let t: syn::Path = syn::parse_str("::rrust::_reverse_rloop").unwrap();
                    cmac.path = t;
                }
                Expr::Macro(ExprMacro { attrs, mac: cmac })
            } else {
                Expr::Macro(ExprMacro { attrs, mac: cmac })
            }
        }
        Expr::Match(_) => panic!("Not yet implemented {}", line!()),
        Expr::MethodCall(_) => panic!("Not yet implemented {}", line!()),
        Expr::Paren(_) => panic!("Not yet implemented {}", line!()),
        Expr::Path(_) => panic!("Not yet implemented {}", line!()),
        Expr::Range(_) => panic!("Not yet implemented {}", line!()),
        Expr::Reference(_) => panic!("Not yet implemented {}", line!()),
        Expr::Repeat(_) => panic!("Not yet implemented {}", line!()),
        Expr::Return(_) => panic!("Not yet implemented {}", line!()),
        Expr::Struct(_) => panic!("Not yet implemented {}", line!()),
        Expr::Try(_) => panic!("Not yet implemented {}", line!()),
        Expr::TryBlock(_) => panic!("Not yet implemented {}", line!()),
        Expr::Tuple(_) => panic!("Not yet implemented {}", line!()),
        Expr::Type(_) => panic!("Not yet implemented {}", line!()),
        Expr::Unary(_) => panic!("Not yet implemented {}", line!()),
        Expr::Unsafe(_) => panic!("Not yet implemented {}", line!()),
        Expr::Verbatim(_) => panic!("Not yet implemented {}", line!()),
        Expr::While(_) => panic!("Not yet implemented {}", line!()),
        Expr::Yield(_) => panic!("Not yet implemented {}", line!()),
        _ => panic!("Not yet implemented {}", line!()),
    }
}
