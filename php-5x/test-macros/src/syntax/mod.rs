use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;

use syn::{
    parse::{self, ParseStream, Parser},
    Expr,
};

use crate::codegen;

pub trait TokenStreamParse: parse::Parse {
    fn parse_ts(input: TokenStream2) -> parse::Result<Self> {
        |stream: ParseStream| -> syn::Result<Self> { Self::parse(stream) }.parse2(input)
    }
}
impl TokenStreamParse for codegen::Args {}

impl parse::Parse for codegen::Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = codegen::Args {
            version: None,
            name: None,
        };

        let mut check_duplicates: HashSet<syn::Ident> = HashSet::new();

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            if check_duplicates.contains(&ident) {
                return Err(parse::Error::new(ident.span(), "duplicate arg keyword"));
            }
            check_duplicates.insert(ident.clone());

            let _eq_token: syn::Token![=] = input.parse()?;
            let ks = ident.to_string();

            match ks.as_str() {
                "name" => {
                    let expr: Expr = input.parse()?;
                    args.name = Some(expr);
                }
                "version" => {
                    let expr: Expr = input.parse()?;
                    args.version = Some(expr);
                }
                _ => return Err(parse::Error::new(ident.span(), "unknown arg keyword")),
            }

            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }
        Ok(args)
    }
}
