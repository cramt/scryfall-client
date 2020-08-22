use proc_macro::TokenStream;

#[proc_macro]
pub fn is_struct2(items: TokenStream) -> TokenStream {
    "pub fn answer() -> u32 { 42 }".parse().unwrap()
}
