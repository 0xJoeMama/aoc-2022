use proc_macro::{Ident, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Fn,
};

struct Args {
    t: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[proc_macro_attribute]
pub fn day(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(item as Fn);
    let args = parse_macro_input!(attr as Args);
    quote::quote! {
    use aoc_lib::input;

    fn main() {
    },
    }
    .into()
}
