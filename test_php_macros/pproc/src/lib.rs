use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, spanned::Spanned, Ident, ItemConst, ItemFn, ItemMod, Signature};

use syn::{
    parse::{self, Parser},
    Item,
};

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    format!("fn answer() -> String {{ \"{}\".into() }}", item)
        .parse()
        .unwrap()
}
fn php56_callback_fn(name: &str, original_function: &ItemFn) -> TokenStream2 {
    let ItemFn {
        attrs: _,
        vis: _,
        sig,
        block: _,
    } = original_function;

    let Signature {
        constness: _,
        asyncness: _,
        unsafety: _,
        abi: _,
        fn_token: _,
        ident,
        generics: _,
        paren_token: _,
        inputs: _,
        variadic: _,
        output: _,
    } = sig;
    let fn_name: Ident = Ident::new(name, Span::call_site());

    eprintln!(
        "{:?}",
        quote!(
            #ident("hello", Empty{})
        )
    );
    // let x = ExprCall {
    //     attrs: Vec::new(),
    //     func: Box::new(Expr::Call("123")),
    //     paren_token: (),
    //     args: (),

    // };

    quote!(
        unsafe extern fn #fn_name(type_: ::std::os::raw::c_int, module_number: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
            #ident("hello", Empty{});
            0
        }
    )
}

#[proc_macro_attribute]
pub fn php_callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_function: ItemFn = syn::parse(item).unwrap();

    let x = php56_callback_fn("mhh", &original_function);

    quote::quote!(
        #original_function

        #x
    )
    .into()
}

#[proc_macro_attribute]
pub fn php_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mod_declaration: ItemMod = syn::parse(item).unwrap();
    let ItemMod {
        attrs,
        vis,
        mod_token,
        ident,
        content,
        semi,
    } = mod_declaration;

    let (_, items) = content.unwrap();
    for item in items {
        match item {
            Item::Macro(mut _m) => {
                // t("implement macro support for referencing a path to function")
                return parse::Error::new(
                    _m.span(),
                    " macro support for referencing a path to function not yet implemented",
                )
                .into_compile_error()
                .into();
            }
            Item::Fn(mut m) => {
                eprintln!("Item::Fn {:?}", m);
            }
            _ => {
                return parse::Error::new(
                    item.span(),
                    "item not yet implemented in #[php_module] macro ",
                )
                .into_compile_error()
                .into();
            }
        }
    }

    quote::quote!(
        //generated_php_module
        #vis mod #ident {

        }#semi      
    ).into()
}
