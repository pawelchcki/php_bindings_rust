use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse, spanned::Spanned, FnArg, ItemFn, PatType, Path, Signature};

use crate::syntax::module::InnerMacros;

use super::Args;

pub fn render_init_fn(original_function: &ItemFn) -> syn::Result<TokenStream2> {
    let ItemFn {
        attrs: _,
        vis: _,
        sig: signature,
        block,
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
        inputs,
        variadic: _,
        output,
    } = signature.clone();

    // TODO add validation that parametr is integer type, or can be .into() it
    let fn_call = match inputs.len() {
        0 => {
            quote! (
                #ident()
            )
        }
        1 => {
            quote! (
                #ident(module_number)
            )
        }
        _ => {
            return Err(parse::Error::new(
                signature.span(),
                "module function has too many arguments",
            ))
        }
    };

    let fn_call_with_result = match output {
        syn::ReturnType::Default => {
            quote! {
                #fn_call;
                0 // Always a success
            }
        }
        syn::ReturnType::Type(_, _) => {
            // TODO add validation of output type is really a basic ? some ? Result type :D
            quote! {
                match #fn_call {
                    Ok(_) => { 0 }
                    Err(_) => { -1 }
                }
            }
        }
    };

    Ok(
        quote!( //TODO: consider not marking this entry funciton as unsafe as it could propagate ??
            unsafe extern "C" fn #ident(_: ::std::os::raw::c_int, module_number: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
                // wrap function
                #signature #block

                #fn_call_with_result
            }
        ),
    )
}

fn module_startup_function_ptr(inner_macros: &Vec<InnerMacros>) -> Option<&syn::Ident> {
    let f = inner_macros.iter().find_map(|f| match f {
        InnerMacros::ModuleInit(f) => Some(f),
        _ => None,
    });

    f.map(|f| &f.sig).map(|s| &s.ident)
}

fn module_info_function_ptr(inner_macros: &Vec<InnerMacros>) -> Option<&syn::Ident> {
    let f = inner_macros.iter().find_map(|f| match f {
        InnerMacros::ModuleInfo(f) => Some(f),
        _ => None,
    });

    f.map(|f| &f.sig).map(|s| &s.ident)
}

pub fn render_mod(inner_macros: &Vec<InnerMacros>, args: Args) -> syn::Result<TokenStream2> {
    let sys = quote!(php_5x_sys::php56);
    let none: syn::Expr = syn::parse2(quote!(None))?;
    let module_startup_function = module_startup_function_ptr(inner_macros)
        .map(|f| syn::parse2::<syn::Expr>(quote! (Some(#f))))
        .unwrap_or(Ok(none.clone()))?;
    
    let module_info_function = module_info_function_ptr(inner_macros)
        .map(|f| syn::parse2::<syn::Expr>(quote! (Some(#f))))
        .unwrap_or(Ok(none))?;

    let version = args
        .version
        .unwrap_or(syn::parse2(quote!(env!("CARGO_PKG_VERSION")))?);
    let name = args
        .name
        .unwrap_or(syn::parse2(quote!(env!("CARGO_PKG_NAME")))?);

    Ok(quote! (
         pub static mut MOD: #sys::zend_module_entry = #sys::zend_module_entry {
            size: ::std::mem::size_of::<#sys::zend_module_entry>() as u16,
            zend_api: #sys::ZEND_MODULE_API_NO,
            zend_debug: #sys::DEBUG_ZEND as u8,
            zts: #sys::USING_ZTS as u8,
            ini_entry: ::std::ptr::null(),
            deps: ::std::ptr::null(),
            name: ::php_5x::cstr!(#name).as_ptr(),
            functions: ::std::ptr::null(),
            module_startup_func: #module_startup_function,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: #module_info_function,
            version: ::php_5x::cstr!(#version).as_ptr(),
            globals_size: 0,
            globals_ptr: ::std::ptr::null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: ::std::ptr::null_mut(),
            module_number: 0,
            build_id: ::php_5x::cstr_u8!(#sys::ZEND_MODULE_BUILD_ID_).as_ptr(),
        };
    ))
}
