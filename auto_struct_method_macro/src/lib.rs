use proc_macro::TokenStream;

#[proc_macro]
pub fn auto_struct_method_macro(_input: TokenStream) -> TokenStream {
    "println!(\"Hello, world!\");".parse().unwrap()
}
