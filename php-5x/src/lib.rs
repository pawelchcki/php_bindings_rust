use std::{mem::size_of, option, ptr};
use std::{
    ffi::{CStr, CString},
};

pub mod php56;
pub mod cstr;

use php_5x_sys::php56 as sys;
use sys::zend_module_entry;

use php_5x_sys as php_sys;

//    pub static mut zend_execute_ex:
// ::std::option::Option<unsafe extern "C" fn(execute_data: *mut zend_execute_data)>;

type Exec = unsafe extern "C" fn(execute_data: *mut sys::zend_execute_data);
static mut BACKUP: option::Option<Exec> = None;

unsafe extern fn execute(execute_data: *mut sys::zend_execute_data) {
    match BACKUP {
        None => (),
        Some(f) => {
            f(execute_data);
        },
    }
}

unsafe extern fn print_me(type_: std::os::raw::c_int, module_number: std::os::raw::c_int) -> std::os::raw::c_int {
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
        build_id: CStr::from_bytes_with_nul(sys::ZEND_MODULE_BUILD_ID_).unwrap().as_ptr(),
    };

    let y = Box::new(x);

    // let z//: *mut sys::zend_module_entry = unsafe { std::mem::transmute(x) };
    Ok(y)
}

pub struct ModArgs<'a> {
    name: &'a CStr,
    version: &'a CStr,
}


// pub const MOD: sys::zend_module_entry = 
//     sys::zend_module_entry {
//         size: size_of::<sys::zend_module_entry>() as u16,
//         zend_api: sys::ZEND_MODULE_API_NO,
//         zend_debug: sys::DEBUG_ZEND as u8,
//         zts: sys::USING_ZTS as u8,
//         ini_entry: ptr::null(),
//         deps: ptr::null(),
//         name: cstr!("Hello").as_ptr(),
//         functions: ptr::null(),
//         module_startup_func: Some(print_me),
//         module_shutdown_func: None,
//         request_startup_func: None,
//         request_shutdown_func: None,
//         info_func: None,
//         version: cstr!("ptr").as_ptr(),
//         globals_size: 0,
//         globals_ptr: ptr::null_mut(),
//         // globals_id_ptr: ptr::null_mut(),
//         globals_ctor: None,
//         globals_dtor: None,
//         post_deactivate_func: None,
//         module_started: 0,
//         type_: 0,
//         handle: ptr::null_mut(),
//         module_number: 0,
//         build_id: BUILD_ID.as_ptr(),  
// };
