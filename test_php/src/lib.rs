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
    unsafe { php_5x_sys::php56::php_info_print_table_row(1, "2", "3"); };
    // Couldn't find a way to make the struct static without rewriting what bindgen generated
    // Probably with bindgen zend_module rewritten manually this will not require allocaitions
    Box::into_raw(Box::new(module::MOD)) as *mut std::ffi::c_void
}
