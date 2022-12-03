use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn day(attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!(attr);
    dbg!(item);
    quote::quote! {
    use aoc_lib::input;

    fn main() {
        println!("hello world");
    }
    }.into()
}
