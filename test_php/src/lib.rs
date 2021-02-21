use std::{borrow::Borrow, ptr};

use libc;

#[no_mangle]
pub extern fn php_module_info() {
    // print_table_start();
    // print_table_row("A demo PHP extension written in Rust", "enabled");
    // print_table_end();

}

// const MOD: () = {
//     #[mod_init]
//     fn mod_init() {
        
//     }

// };

#[no_mangle]
pub extern fn get_module() -> *mut std::ffi::c_void {
    // let z = php_56::create_module().unwrap();

    // return Box::into_raw(z) as *mut std::ffi::c_void;
    ptr::null_mut()
}


// #[no_mangle]
// pub extern fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
//     let mut name_zval = Zval::new_as_null();
//     php_parse_parameters!(&mut name_zval);
//     let name = String::try_from(name_zval).ok().unwrap();
//     let hello = format!("Hello {}", name);
//     php_return!(retval, hello);
// }
