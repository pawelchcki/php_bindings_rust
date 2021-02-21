use std::ptr;

#[php_test_macros::render]
pub mod module {}

#[no_mangle]
pub extern "C" fn php_module_info() {
    // print_table_start();
    // print_table_row("A demo PHP extension written in Rust", "enabled");
    // print_table_end();
}

#[no_mangle]
pub extern "C" fn get_module() -> *mut std::ffi::c_void {
    // let z = php_56::create_module().unwrap();

    // return Box::into_raw(z) as *mut std::ffi::c_void;
    ptr::null_mut()
}
