// use std::ffi::CStr;
//
// use php_5x_sys::php56 as sys;

// type Callback = unsafe extern "C" fn(type_: std::os::raw::c_int, module_number: std::os::raw::c_int) -> std::os::raw::c_int;

// macro_rules! mod_make {
//     ( $s:ident, $name:expr, $version:expr) => {
//         extern "C" fn blah(type_: std::os::raw::c_int, module_number: std::os::raw::c_int) -> std::os::raw::c_int {
//             0
//         }

//         const $s: sys::zend_module_entry = sys::zend_module_entry {
//             size: size_of::<sys::zend_module_entry>() as u16,
//             zend_api: sys::ZEND_MODULE_API_NO,
//             zend_debug: sys::DEBUG_ZEND as u8,
//             zts: sys::USING_ZTS as u8,
//             ini_entry: ptr::null(),
//             deps: ptr::null(),
//             name: cstr!($name).as_ptr(),
//             functions: ptr::null(),
//             module_startup_func: None,
//             module_shutdown_func: None,
//             request_startup_func: None,
//             request_shutdown_func: None,
//             info_func: None,
//             version: cstr!($version).as_ptr(),
//             globals_size: 0,
//             globals_ptr: ptr::null_mut(),
//             globals_ctor: None,
//             globals_dtor: None,
//             post_deactivate_func: None,
//             module_started: 0,
//             type_: 0,
//             handle: ptr::null_mut(),
//             module_number: 0,
//             build_id: cstr_u8!(sys::ZEND_MODULE_BUILD_ID_).as_ptr(),
//         };
//     };
// }

// mod_make!(MOD, "best", "0.0.0");
// const fn new(args: &ModArgs) -> sys::zend_module_entry {
//     sys::zend_module_entry {
//         size: size_of::<sys::zend_module_entry>() as u16,
//         zend_api: sys::ZEND_MODULE_API_NO,
//         zend_debug: sys::DEBUG_ZEND as u8,
//         zts: sys::USING_ZTS as u8,
//         ini_entry: ptr::null(),
//         deps: ptr::null(),
//         name: args.name.as_ptr(),
//         functions: ptr::null(),
//         module_startup_func: args.startup_func,
//         module_shutdown_func: None,
//         request_startup_func: None,
//         request_shutdown_func: None,
//         info_func: None,
//         version: args.version.as_ptr(),
//         globals_size: 0,
//         globals_ptr: ptr::null_mut(),
//         globals_ctor: None,
//         globals_dtor: None,
//         post_deactivate_func: None,
//         module_started: 0,
//         type_: 0,
//         handle: ptr::null_mut(),
//         module_number: 0,
//         build_id: BUILD_ID.as_ptr(),
//     }
// }
