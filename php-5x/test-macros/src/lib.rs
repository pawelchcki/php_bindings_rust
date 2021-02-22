use codegen::php56::render_init_fn;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, ItemMod};

mod codegen;
mod syntax;

use syntax::{TokenStreamParse, module::parse_inner_macros};

fn render_inside_mod(item: syn::ItemMod, inner: TokenStream2) -> parse::Result<TokenStream2> {
    let syn::ItemMod {
        attrs: _,
        vis,
        mod_token: _,
        ident,
        content: _,
        semi,
    } = item;
    // content.map_or(Ok(()), |(_, items)| {
    //     if items.len() == 0 {
    //         Ok(())
    //     } else {
    //         Err(parse::Error::new(
    //             ident.span(),
    //             "module must be empty, it will be filled by the macro",
    //         ))
    //     }
    // })?;

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
        let inner = codegen::php56::render_mod(&Vec::new(), args)?;
        render_inside_mod(m, inner)
    }

    match render(attr.into(), item.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn php_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    pub fn render(_attr: TokenStream2, item: TokenStream2) -> parse::Result<TokenStream2> {
        let args = codegen::Args::parse_ts(_attr)?;

        let m: ItemMod = syn::parse2(item)?;
        let inner_macros = parse_inner_macros(&m)?;

        let inner_mod = codegen::php56::render_mod(&inner_macros, args)?;

        let init_fn = inner_macros.iter().find_map(|f| match f {
            syntax::module::InnerMacros::ModuleInit(f) => Some(f),
            _ => None,
        });

        let z = match init_fn {
            Some(item_fn) => {
                Some(render_init_fn(item_fn)?)
            }
            None => { None }
        };

        // let function = render_init_fn() 
        render_inside_mod(m, quote! (
            #z

            #inner_mod
        ))
    }

    match render(attr.into(), item.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
