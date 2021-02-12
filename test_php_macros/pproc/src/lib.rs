
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Expr, ExprCall, ExprMethodCall, Ident, ItemFn, Signature};

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    format!("fn answer() -> String {{ \"{}\".into() }}", item)
        .parse()
        .unwrap()
}
fn php56_callback_fn(name: &str, original_function: &ItemFn) -> TokenStream2 {
    let ItemFn { attrs, vis: _, sig, block:_  } = original_function;

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
    let fnName = Ident::new(name, Span::call_site());


    eprintln!("{:?}", quote!(
        #ident("hello", Empty{})
    ));
    // let x = ExprCall {
    //     attrs: Vec::new(),
    //     func: Box::new(Expr::Call("123")),
    //     paren_token: (),
    //     args: (),

    // };

    quote!(
        unsafe extern fn #fnName(type_: ::std::os::raw::c_int, module_number: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
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
    ).into()
}
