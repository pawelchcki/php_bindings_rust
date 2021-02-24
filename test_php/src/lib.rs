use std::{
    os::raw::{c_char, c_int},
    ptr,
};

use php_5x::cstr;
use php_all_sys::php56::{
    zend_extension, zend_extension_version_info, zend_module_entry, zend_register_extension,
    zend_register_module_ex, zend_startup_module_ex,
};
use php_test_macros::php_module;

#[php_module()]
pub mod module {
    #[module_init]
    fn init() {
        println!("Module initiated");
    }
}

#[no_mangle]
pub static mut extension_version_info: zend_extension_version_info = zend_extension_version_info {
    zend_extension_api_no: 300000000_i32,
    build_id: unsafe { cstr!("123").as_ptr() as *mut c_char },
};

pub extern "C" fn api_no_check(a: c_int) -> c_int {
    0
}

pub extern "C" fn build_id_check(v: *const c_char) -> c_int {
    0
}

pub extern "C" fn startup(extension: *mut zend_extension) -> ::std::os::raw::c_int {
    unsafe {
        zend_register_module_ex(get_module() as *mut zend_module_entry);
        // zend_startup_module_ex(get_module() as *mut zend_module_entry)
        0
    }
}

// #[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut zend_extension_entry: zend_extension = zend_extension {
    name: unsafe { cstr!(env!("CARGO_PKG_NAME")).as_ptr() as *mut c_char },
    version: unsafe { cstr!(env!("CARGO_PKG_VERSION")).as_ptr() as *mut c_char },
    author: unsafe { cstr!(env!("CARGO_PKG_AUTHORS")).as_ptr() as *mut c_char },
    URL: unsafe { cstr!(env!("CARGO_PKG_HOMEPAGE")).as_ptr() as *mut c_char },
    copyright: unsafe { cstr!("Copyright (c) 1999-2030").as_ptr() as *mut c_char },
    startup: Some(startup),
    shutdown: None,
    activate: None,
    deactivate: None,
    message_handler: None,
    op_array_handler: None,
    statement_handler: None,
    fcall_begin_handler: None,
    fcall_end_handler: None,
    op_array_ctor: None,
    op_array_dtor: None,
    api_no_check: Some(api_no_check),
    build_id_check: Some(build_id_check),
    reserved3: ptr::null_mut(),
    reserved4: ptr::null_mut(),
    reserved5: ptr::null_mut(),
    reserved6: ptr::null_mut(),
    reserved7: ptr::null_mut(),
    reserved8: ptr::null_mut(),
    handle: ptr::null_mut(),
    resource_number: 1,
};

#[no_mangle]
pub extern "C" fn get_module() -> *mut std::ffi::c_void {
    unsafe {
        &module::MOD as *const zend_module_entry as *const std::ffi::c_void as *mut std::ffi::c_void
    }
}
