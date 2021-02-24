use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use syn::Path;

use super::Args;

pub fn foo(init_fn: Option<Path>, args: Args) -> syn::Result<TokenStream2> {
    let sys = quote!(php_all_sys::php54);
    let none: syn::Path = syn::parse2(quote!(None))?;
    let module_startup_function = init_fn.unwrap_or(none);
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
            info_func: None,
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
