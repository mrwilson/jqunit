pub mod runner {
    use std::error::Error;
    use crate::jq::jq::{jq_compile, jq_get_lib_dirs, jq_init, jq_next, jq_set_attr, jq_start, jq_state, jv, jv_array_append, jv_array_get, jv_array_length, jv_copy, jv_dump, jv_dump_string, jv_get_kind, jv_invalid_get_msg, jv_kind_JV_KIND_ARRAY, jv_kind_JV_KIND_INVALID, jv_null, jv_string, jv_string_value};

    use std::ffi::{CStr, CString};

    pub struct Runner {
        state: *mut jq_state,
    }

    pub fn jv_to_string(jv: jv) -> String {
        unsafe { String::from(CStr::from_ptr(jv_string_value(jv)).to_str().expect("a")) }
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

        pub fn execute_code_with_no_input(&self, code: &str) -> Result<String, String> {
            let code_as_cstring = CString::new(code).expect("failure");

            unsafe {
                jq_compile(self.state, code_as_cstring.as_ptr());
                jq_start(self.state, jv_null(), 0);

                Ok(self.state)
                    .map(|value| jq_next(value))
                    .and_then(|value| {
                        if jv_get_kind(value) == jv_kind_JV_KIND_INVALID {
                            Err(value)
                        } else {
                            Ok(value)
                        }
                    })
                    .map(|value| jv_to_string(value))
                    .map_err(|err| jv_invalid_get_msg(err))
                    .map_err(|err| jv_to_string(err))
            }
        }

        pub fn get_functions_for_module(&self, module: &str) -> Vec<String> {
            let code_as_cstring = CString::new("modulemeta | .defs").expect("failure");

            unsafe {
                jq_compile(self.state, code_as_cstring.as_ptr());
                jq_start(self.state, jv_from_string(module), 0);

                let defined_functions = jq_next(self.state);

                if jv_get_kind(defined_functions) == jv_kind_JV_KIND_ARRAY {
                    let mut functions = vec![];

                    for i in 0..jv_array_length(jv_copy(defined_functions)) {
                        let mut function =
                            jv_to_string(jv_array_get(jv_copy(defined_functions), i));

                        function.truncate(function.find("/").expect("foo"));
                        functions.push(function)
                    }

                    functions
                } else {
                    vec![]
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::runner::runner::Runner;

    fn fixtures() -> String {
        fs::canonicalize("./fixtures")
            .expect("path exists")
            .as_path()
            .to_str()
            .map(String::from)
            .expect("as a string")
    }

    #[test]
    fn should_execute_code_with_no_input() {
        assert_eq!(
            Runner::start().execute_code_with_no_input("\"hello\""),
            Ok(String::from("hello"))
        );
    }

    #[test]
    fn should_load_library_and_execute_code() {
        let runner = Runner::start();
        runner.add_library(&fixtures());

        assert_eq!(
            runner
                .execute_code_with_no_input("import \"simple_function\" as s; s::simple_function"),
            Ok(String::from("2"))
        );
    }

    #[test]
    fn should_return_error_if_exits_with_error() {
        let runner = Runner::start();
        runner.add_library(&fixtures());

        assert_eq!(
            runner
                .execute_code_with_no_input("error(\"Failed to run\")"),
            Err(String::from("Failed to run"))
        );
    }

    #[test]
    fn should_load_list_of_functions_from_module() {
        let runner = Runner::start();
        runner.add_library(&fixtures());

        assert_eq!(
            runner.get_functions_for_module("simple_function"),
            vec!["simple_function", "other_simple_function"]
        );
    }
}
