use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};

struct Args {
    day: LitInt,
    parser: Option<Ident>,
    part1: Option<Ident>,
    part2: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let day = input.parse()?;
        let mut args = HashMap::new();

        input.parse::<Token![,]>()?;
        while let Some(id) = input.parse::<Ident>().ok() {
            match id.to_string().as_str() {
                "parser" => {
                    input.parse::<Token![=]>()?;
                    let parser: Ident = input.parse()?;
                    args.insert("parser", parser);
                }
                "part1" => {
                    input.parse::<Token![=]>()?;
                    let part1: Ident = input.parse()?;
                    args.insert("part1", part1);
                }

                "part2" => {
                    input.parse::<Token![=]>()?;
                    let part2: Ident = input.parse()?;
                    args.insert("part2", part2);
                }
                _ => return Err(syn::Error::new(id.span(), "unknown argument")),
            }

            if !input.peek(Token![,]) {
                break;
            } else {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Args {
            day,
            parser: args.get("parser").cloned(),
            part1: args.get("part1").cloned(),
            part2: args.get("part2").cloned(),
        })
    }
}

#[proc_macro_attribute]
pub fn day(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);
    let day: u8 = args.day.base10_parse().unwrap();
    let parser = args.parser;
    let part1 = args.part1;
    let part2 = args.part2;

    quote! {
        fn main() {
            println!("Running Advent of Code Day {}", #day);
            _ = aoc_lib::input::apply(&format!("input-day-{:02}.txt", #day), |input| {
                aoc_lib::timed(|| {
                    let input = #parser(&input);
                    aoc_lib::timed(|| println!("Part 1: {}", #part1(&input)));
                    aoc_lib::timed(|| println!("Part 2: {}", #part2(&input)));
                });
            });
        }
    }
    .into()
}
