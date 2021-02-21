use std::collections::HashSet;

use php_codegen::Args;
use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream, Parser},
    Expr, ItemMod,
};

trait TokenStreamParse: Sized {
    fn parse_ts(input: TokenStream2) -> parse::Result<Self> {
        |stream: ParseStream| -> syn::Result<Self> { Self::parse(stream) }.parse2(input)
    }
    fn parse(input: ParseStream) -> syn::Result<Self>;
}

impl TokenStreamParse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Args {
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

fn render_inside_mod(item: syn::ItemMod, inner: TokenStream2) -> parse::Result<TokenStream2> {
    let syn::ItemMod {
        attrs: _,
        vis,
        mod_token: _,
        ident,
        content,
        semi,
    } = item;
    content.map_or(Ok(()), |(_, items)| {
        if items.len() == 0 {
            Ok(())
        } else {
            Err(parse::Error::new(
                ident.span(),
                "module must be empty, it will be filled by the macro",
            ))
        }
    })?;

    Ok(quote!(
        #vis mod #ident {
            #inner
        }#semi
    ))
}

#[proc_macro_attribute]
pub fn render(attr: TokenStream, item: TokenStream) -> TokenStream {
    pub fn render(_attr: TokenStream2, item: TokenStream2) -> parse::Result<TokenStream2> {
        let args = Args::parse_ts(_attr)?;

        let m: ItemMod = syn::parse2(item)?;
        let inner = php_codegen::php56::foo(None, args)?;
        render_inside_mod(m, inner)
    }
    concat!("1", "2");

    match render(attr.into(), item.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
