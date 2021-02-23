use std::{os::raw::c_char, ptr};

use php_5x::cstr;
use php_5x_sys::php56::{zend_extension, zend_module_entry };
use php_test_macros::php_module;

#[php_module(
    name = "best_module_ever",
    version = "both version and module are automatically specified when not configured"
)]
pub mod module {
    #[module_init]
    fn init() {
        println!("Module initiated");
    }
}

// #[no_mangle]
// pub static mut extension_version_info: zend_extension_version_info = zend_extension_version_info {

// };

// #[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut zend_extension_entry: zend_extension = zend_extension {
    name: unsafe { cstr!("ehlo").as_ptr() as *mut c_char },
    version: unsafe { cstr!("ehlo").as_ptr() as *mut c_char },
    author: unsafe { cstr!("ehlo").as_ptr() as *mut c_char },
    URL: unsafe { cstr!("ehlo").as_ptr() as *mut c_char },
    copyright: unsafe { cstr!("ehlo").as_ptr() as *mut c_char },
    startup: None,
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
    api_no_check: None,
    build_id_check: None,
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

    unsafe { &module::MOD as * const zend_module_entry as *const std::ffi::c_void as *mut std::ffi::c_void }
}
