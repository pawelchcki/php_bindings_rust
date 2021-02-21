use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, ItemMod};

mod codegen;
mod syntax;

use syntax::TokenStreamParse;

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
        let args = codegen::Args::parse_ts(_attr)?;

        let m: ItemMod = syn::parse2(item)?;
        let inner = codegen::php56::foo(None, args)?;
        render_inside_mod(m, inner)
    }

    match render(attr.into(), item.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
