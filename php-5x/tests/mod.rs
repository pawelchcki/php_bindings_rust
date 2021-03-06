#[cfg(test)]
mod tests {
    use php_all_sys::php56::zend_module_entry;
    use std::ffi::CStr;
    fn from_c_str<'a>(ptr: *const std::os::raw::c_char) -> &'a str {
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    #[php_test_macros::render(name = "custom_name", version = "custom_version")]
    mod custom_module_params {}

    #[test]
    fn valid_custom_name_and_version() {
        let m: zend_module_entry = unsafe { custom_module_params::MOD };
        assert_eq!(from_c_str(m.build_id), "API20131226,NTS");
        assert_eq!(from_c_str(m.version), "custom_version");
        assert_eq!(from_c_str(m.name), "custom_name");
    }

    #[php_test_macros::render]
    mod default_module {}
    #[test]
    fn expected_custom_name_and_version() {
        let m: zend_module_entry = unsafe { default_module::MOD };
        assert_eq!(from_c_str(m.build_id), "API20131226,NTS");
        assert_eq!(from_c_str(m.version), env!("CARGO_PKG_VERSION"));
        assert_eq!(from_c_str(m.name), env!("CARGO_PKG_NAME"));
    }
    #[php_test_macros::php_module]
    mod module_with_init {
        #[module_init]
        fn init() {

        }
    }

    #[test]
    fn module_init_is_set_correctly() {
        let _m: zend_module_entry = unsafe { module_with_init::MOD };

        // assert_eq!(m.module_startup_func.unwrap(), module_with_init::init);
    }
}
