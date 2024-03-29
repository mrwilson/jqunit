use crate::jq::jq::{
    jv_array_get, jv_array_length, jv_copy, jv_dump_string, jv_get_kind, jv_string,
    jv_string_value, Jv, JV_KIND_ARRAY, JV_KIND_INVALID, JV_KIND_STRING,
};
use std::ffi::{c_int, CStr, CString};

impl IntoIterator for Jv {
    type Item = Jv;
    type IntoIter = JvIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        if unsafe { jv_get_kind(self) } == JV_KIND_ARRAY {
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
    value: Jv,
    index: usize,
    size: usize,
}

impl Iterator for JvIntoIterator {
    type Item = Jv;
    fn next(&mut self) -> Option<Jv> {
        if self.index >= self.size {
            return None;
        }

        let result = unsafe { jv_array_get(jv_copy(self.value), self.index as c_int) };
        self.index += 1;

        Some(result)
    }
}

pub fn jv_from_string(input: &str) -> Jv {
    unsafe {
        let as_ptr = CString::new(input).expect("Invalid value passed to jv_string");
        return jv_string(as_ptr.as_ptr());
    }
}

impl ToString for Jv {
    fn to_string(&self) -> String {
        unsafe {
            let value = if jv_get_kind(*self) == JV_KIND_STRING {
                *self
            } else {
                jv_dump_string(*self, 0)
            };

            String::from(CStr::from_ptr(jv_string_value(value)).to_str().expect("a"))
        }
    }
}

pub fn remove_arity(name: String) -> String {
    let mut function_name = name.clone();
    function_name.truncate(function_name.find("/").expect("foo"));
    function_name
}

pub fn jv_to_result(value: Jv) -> Result<Jv, Jv> {
    if unsafe { jv_get_kind(value) } == JV_KIND_INVALID {
        Err(value)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::jq::jq::jv_array;
    use crate::jq::utils::{jv_from_string, remove_arity};

    #[test]
    fn string_values_are_reversible() {
        assert_eq!(jv_from_string("Hello, World!").to_string(), "Hello, World!")
    }

    #[test]
    fn non_string_values_are_serialised() {
        unsafe { assert_eq!(jv_array().to_string(), "[]") }
    }

    #[test]
    fn arities_are_removed_from_function_names() {
        assert_eq!(
            remove_arity(String::from("test_function/0")),
            "test_function"
        )
    }
}
