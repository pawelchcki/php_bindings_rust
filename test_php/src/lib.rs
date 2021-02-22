use php_5x_sys::php56::zend_module_entry;
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zme {
    pub globals_ptr: *mut ::std::os::raw::c_void,
    pub module_number: ::std::os::raw::c_int,
}

static mut C: &zme = &zme { 
    globals_ptr: std::ptr::null_mut(),
    module_number: 1,
};

#[no_mangle]
pub extern "C" fn get_module() -> *mut std::ffi::c_void {
    "asf".as_ptr();
    // unsafe { std::mem::transmute::<_, * mut std::ffi::c_void>(module::MOD) }

    unsafe { &module::MOD as * const zend_module_entry as *const std::ffi::c_void as *mut std::ffi::c_void }
}
