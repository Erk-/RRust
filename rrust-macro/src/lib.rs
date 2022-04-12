mod utils;
mod forward;
mod reverse;

#[proc_macro]
pub fn forward(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    forward::forward_impl(input)
}

#[proc_macro]
pub fn reverse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    reverse::reverse_impl(input)
}
