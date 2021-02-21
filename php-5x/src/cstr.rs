
#[allow(unconditional_panic)]
const fn const_panic() {
    [][0]
}

#[doc(hidden)]
pub const fn validate_cstr_contents(bytes: &[u8], nul: bool) {
    let mut i = 0;
    let check_len = if nul && bytes[bytes.len()-1] == b'\0' {
        bytes.len()-1usize
    } else {
        bytes.len()
    };

    while i < check_len {
        if bytes[i] == b'\0' {
            const_panic();
        }
        i += 1;
    }
}

#[doc(hidden)]
pub const fn ensure_is_null_terminated(bytes: &[u8]) {
    if bytes[bytes.len()-1usize] != b'\0' {
        const_panic()
    }
}

#[macro_export]
macro_rules! cstr {
    ( $s:literal ) => {{
        $crate::cstr::validate_cstr_contents($s.as_bytes(), false);
        unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!($s, "\0")) }
    }};
}

#[macro_export]
macro_rules! cstr_with_nul {
    ( $s:literal ) => {{
        $crate::cstr::validate_cstr_contents($s.as_bytes(), true);
        $crate::cstr::ensure_is_null_terminated($s.as_bytes());
        unsafe { std::mem::transmute::<_, &std::ffi::CStr>($s) }
    }};
}

#[macro_export]
macro_rules! cstr_u8 {
    ( $s:path ) => {{
        $crate::cstr::validate_cstr_contents($s, true);
        $crate::cstr::ensure_is_null_terminated($s);
        unsafe { std::mem::transmute::<_, &std::ffi::CStr>($s as &[u8]) }
    }};
}

