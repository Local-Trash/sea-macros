pub use proc_macro::TokenStream;
pub use syn::{Ident, parse::Parse, Lit, Token, parse_macro_input};

struct DefineLayout {
    name: Ident,
    value: Lit
}

impl Parse for DefineLayout {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        input.parse::<Token![,]>()?;

        let value: Lit = input.parse()?;
        Ok(DefineLayout {
            name,
            value
        })
    }
}