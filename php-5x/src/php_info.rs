use std::ffi::{CStr};

use php_5x_sys::php56;
use crate::cstr;

pub struct InsideTable {

}

impl InsideTable {
    pub fn header(&self, headers: &[&CStr]) {
        //TODO!!!
        unsafe { php56::php_info_print_table_header(headers.len() as i32, headers.as_ptr())  }
    }

    pub fn row(&self, headers: &[&CStr]) {
        //TODO!!!
        unsafe { php56::php_info_print_table_row(headers.len() as i32, headers.as_ptr())  }
    }
}

pub fn start_table<F>(f: F) where F: FnOnce(&InsideTable) {
    unsafe {  php56::php_info_print_table_start() };    
    f(&InsideTable {});
    unsafe {  php56::php_info_print_table_end() };

}

fn _test() {
    start_table(|t| { 
        t.header(&[cstr!("Hello"), cstr!("ehlo")]);
    })
}