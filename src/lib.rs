use proc_macro::TokenStream;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    use syn::{Ident, parse::Parse, Lit, Token, parse_macro_input};
    use quote::quote;

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

    let DefineLayout { name, value } = parse_macro_input!(input as DefineLayout);

    TokenStream::from(quote! {
        macro_rules! #name {
            () => {
                #value
            }
        }
    })
}

#[proc_macro]
pub fn c_loop(input: TokenStream) -> TokenStream {
    use syn::{Expr, parse::Parse, Token, ExprAssign};
    use quote::quote;
    use syn::parse_macro_input;

    struct CLoop {
        init: ExprAssign,
        condinition: Expr,
        update_loop: Expr,
        update: Expr
    }

    impl Parse for CLoop {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let init: ExprAssign = input.parse()?;
            
            input.parse::<Token![;]>()?;

            let condinition: Expr = input.parse()?;

            input.parse::<Token![;]>()?;

            let update_loop: Expr = input.parse()?;

            input.parse::<Token![;]>()?;

            let update: Expr = input.parse()?;
            
            Ok(CLoop {
                init,
                condinition,
                update_loop,
                update
            })
        }
    }

    let CLoop { init, condinition, update_loop, update } = parse_macro_input!(input as CLoop);


    TokenStream::from(quote! {
        let mut #init;
        '_c_loop_: loop {
            if #condinition {
                break '_c_loop_;
            }

            #update

            #update_loop;
        }
    })
}