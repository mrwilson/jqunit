mod jqunit {
    use crate::jq::jq::{
        jq_compile, jq_get_lib_dirs, jq_init, jq_next, jq_set_attr, jq_start, jq_state, jv,
        jv_array, jv_array_append, jv_array_get, jv_array_length, jv_copy, jv_dump_string,
        jv_get_kind, jv_kind_JV_KIND_NULL, jv_null, jv_string, jv_string_value,
    };
    use std::ffi::{CStr, CString};

    pub struct Runner {
        state: *mut jq_state,
    }

    pub fn jv_to_string(jv: jv) -> String {
        unsafe {
            String::from(
                CStr::from_ptr(jv_string_value(jv_dump_string(jv, 0)))
                    .to_str()
                    .expect("a"),
            )
        }
    }

    pub fn jv_from_string(input: &str) -> jv {
        unsafe {
            return jv_string(
                CString::new(input)
                    .expect("Invalid value passed to jv_string")
                    .as_ptr(),
            );
        }
    }

    impl Runner {
        pub fn start() -> Runner {
            Runner {
                state: unsafe { jq_init() },
            }
        }

        pub fn add_library(&self, path: &str) {
            unsafe {
                let libs = jv_array_append(jq_get_lib_dirs(self.state), jv_from_string(path));
                jq_set_attr(self.state, jv_from_string("JQ_LIBRARY_PATH"), libs);
            }
        }

        pub fn execute_code_with_no_input(&self, code: &str) -> Option<String> {
            let code_as_cstring = CString::new(code).expect("failure");

            unsafe {
                jq_compile(self.state, code_as_cstring.as_ptr());
                jq_start(self.state, jv_null(), 0);

                Some(self.state)
                    .map(|value| jq_next(value))
                    .map(|value| jv_string_value(value))
                    .map(|value| CStr::from_ptr(value))
                    .and_then(|cstr| cstr.to_str().ok())
                    .map(String::from)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::runner::jqunit::Runner;
    use std::fs;

    #[test]
    fn should_execute_code_with_no_input() {
        assert_eq!(
            Runner::start().execute_code_with_no_input("\"hello\""),
            Some(String::from("hello"))
        );
    }

    #[test]
    fn should_load_library_and_execute_code() {
        let runner = Runner::start();
        runner.add_library(
            fs::canonicalize("./fixtures")
                .expect("path exists")
                .as_path()
                .to_str()
                .expect("path"),
        );

        assert_eq!(
            runner.execute_code_with_no_input("include \"simple_function\"; simple_function"),
            Some(String::from("2"))
        );
    }
}
