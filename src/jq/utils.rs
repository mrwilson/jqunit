use crate::jq::jq::{
    jv, jv_array_get, jv_array_length, jv_copy, jv_dump_string, jv_get_kind, jv_kind_JV_KIND_ARRAY,
    jv_kind_JV_KIND_INVALID, jv_kind_JV_KIND_STRING, jv_string, jv_string_value,
};
use std::ffi::{c_int, CStr, CString};

impl IntoIterator for jv {
    type Item = jv;
    type IntoIter = JvIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        if unsafe { jv_get_kind(self) } == jv_kind_JV_KIND_ARRAY {
            JvIntoIterator {
                value: self,
                index: 0,
                size: unsafe { jv_array_length(jv_copy(self)) } as usize,
            }
        } else {
            JvIntoIterator {
                value: self,
                index: 0,
                size: 0,
            }
        }
    }
}

pub struct JvIntoIterator {
    value: jv,
    index: usize,
    size: usize,
}

impl Iterator for JvIntoIterator {
    type Item = jv;
    fn next(&mut self) -> Option<jv> {
        if self.index >= self.size {
            return None;
        }

        let result = unsafe { jv_array_get(jv_copy(self.value), self.index as c_int) };
        self.index += 1;

        Some(result)
    }
}

pub fn jv_from_string(input: &str) -> jv {
    unsafe {
        let as_ptr = CString::new(input).expect("Invalid value passed to jv_string");
        return jv_string(as_ptr.as_ptr());
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

pub fn remove_arity(name: String) -> String {
    let mut function_name = name.clone();
    function_name.truncate(function_name.find("/").expect("foo"));
    function_name
}

pub fn jv_to_result(value: jv) -> Result<jv, jv> {
    if unsafe { jv_get_kind(value) } == jv_kind_JV_KIND_INVALID {
        Err(value)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::jq::jq::jv_array;
    use crate::jq::utils::{jv_from_string, jv_to_string, remove_arity};

    #[test]
    fn string_values_are_reversible() {
        assert_eq!(
            jv_to_string(jv_from_string("Hello, World!")),
            "Hello, World!"
        )
    }

    #[test]
    fn non_string_values_are_serialised() {
        unsafe { assert_eq!(jv_to_string(jv_array()), "[]") }
    }

    #[test]
    fn arities_are_removed_from_function_names() {
        assert_eq!(
            remove_arity(String::from("test_function/0")),
            "test_function"
        )
    }
}
