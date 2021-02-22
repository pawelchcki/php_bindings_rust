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

#[no_mangle]
pub extern "C" fn get_module() -> *mut std::ffi::c_void {
    unsafe { &module::MOD as * const zend_module_entry as *const std::ffi::c_void as *mut std::ffi::c_void }
}
