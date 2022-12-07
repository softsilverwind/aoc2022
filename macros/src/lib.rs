use syn::{parse_macro_input, parse::{Parse, ParseStream}, LitInt, LitStr, Token};
use quote::quote;
use proc_macro2::{TokenStream, Ident, Span};

mod kw
{
    syn::custom_keyword!(days);
}

struct Config
{
    days: i32
}

impl Parse for Config
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        input.parse::<kw::days>()?;
        input.parse::<Token![=]>()?;
        let days_lit: LitInt = input.parse()?;

        Ok(Self {
            days: days_lit.base10_parse()?
        })
    }
}

#[proc_macro]
pub fn gen_aoc_dispatch(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let config: Config = parse_macro_input!(input);

    let mut mods = TokenStream::new();
    let mut cases = TokenStream::new();

    for i in 1..=config.days {
        let modname = Ident::new(&format!("d{}", i), Span::call_site());
        mods.extend(
            quote! {
                mod #modname;
            }
        );

        let simple = LitStr::new(&format!("{}a", i), Span::call_site());
        let complex = LitStr::new(&format!("{}b", i), Span::call_site());
        cases.extend(
            quote! {
                #simple => #modname::simple(),
                #complex => #modname::complex(),
            }
        )
    }

// Expands to:
//
// mod d1;
// mod d2;
// ...
// fn aoc_dispatch(arg: &str)
// {
//     match arg {
//         "1a" => d1::simple(),
//         "1b" => d1::complex(),
//         ...
//     }
// }
    quote! {
        #mods

        fn aoc_dispatch(arg: &str)
        {
            match arg {
                #cases
                x => panic!("Unsolved problem {}, try 1a or 1b!", x)
            }
        }
    }.into()
}

