use std::ffi::{CStr, CString};
use std::{mem::size_of, option, ptr};

pub mod cstr;
pub mod php56;

pub mod php_info;

use php_all_sys::php56 as sys;

type Exec = unsafe extern "C" fn(execute_data: *mut sys::zend_execute_data);
static mut BACKUP: option::Option<Exec> = None;

unsafe extern "C" fn execute(execute_data: *mut sys::zend_execute_data) {
    match BACKUP {
        None => (),
        Some(f) => {
            f(execute_data);
        }
    }
}

unsafe extern "C" fn print_me(
    _type_: std::os::raw::c_int,
    _module_number: std::os::raw::c_int,
) -> std::os::raw::c_int {
    BACKUP = sys::zend_execute_ex;
    sys::zend_execute_ex = Some(execute);
    0
}

pub fn create_module() -> anyhow::Result<Box<sys::zend_module_entry>> {
    let x: sys::zend_module_entry = sys::zend_module_entry {
        size: size_of::<sys::zend_module_entry>() as u16,
        zend_api: sys::ZEND_MODULE_API_NO,
        zend_debug: sys::DEBUG_ZEND as u8,
        zts: sys::USING_ZTS as u8,
        ini_entry: ptr::null(),
        deps: ptr::null(),
        name: CString::new("ahello")?.into_raw(),
        functions: ptr::null(),
        module_startup_func: Some(print_me),
        module_shutdown_func: None,
        request_startup_func: None,
        request_shutdown_func: None,
        info_func: None,
        version: CString::new("0.1.0")?.into_raw(),
        globals_size: 0,
        globals_ptr: ptr::null_mut(),
        // globals_id_ptr: ptr::null_mut(),
        globals_ctor: None,
        globals_dtor: None,
        post_deactivate_func: None,
        // STANDARD_MODULE_PROPERTIES_EX 0, 0, NULL, 0, ZEND_MODULE_BUILD_ID
        module_started: 0,
        type_: 0,
        handle: ptr::null_mut(),
        module_number: 0,
        build_id: CStr::from_bytes_with_nul(sys::ZEND_MODULE_BUILD_ID_)
            .unwrap()
            .as_ptr(),
    };

    let y = Box::new(x);

    // let z//: *mut sys::zend_module_entry = unsafe { std::mem::transmute(x) };
    Ok(y)
}
