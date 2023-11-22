use std::ffi::{CStr, CString};
use crate::jq::jq::{jv, jv_dump_string, jv_get_kind, jv_kind_JV_KIND_STRING, jv_string, jv_string_value};

pub fn jv_from_string(input: &str) -> jv {
    unsafe {
        return jv_string(
            CString::new(input)
                .expect("Invalid value passed to jv_string")
                .as_ptr(),
        );
    }
}

pub fn jv_to_string(jv: jv) -> String {
    unsafe {
        let value = if jv_get_kind(jv) == jv_kind_JV_KIND_STRING {
            jv
        } else {
            jv_dump_string(jv, 0)
        };

        String::from(CStr::from_ptr(jv_string_value(value)).to_str().expect("a"))
    }
}

#[cfg(test)]
mod test {
    use crate::jq::jq::jv_array;
    use crate::jq::utils::{jv_from_string, jv_to_string};

    #[test]
    fn string_values_are_reversible() {
        assert_eq!(jv_to_string(jv_from_string("Hello, World!")), "Hello, World!")
    }

    #[test]
    fn non_string_values_are_serialised() {
        unsafe { assert_eq!(jv_to_string(jv_array()), "[]") }
    }
}