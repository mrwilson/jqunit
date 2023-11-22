pub mod runner {
    use crate::jq::jq::{jq_compile, jq_get_lib_dirs, jq_init, jq_next, jq_set_attr, jq_start, jq_state, jv_array_append, jv_invalid_get_msg, jv_null};
    use crate::jq::utils::{jv_from_string, jv_to_result, jv_to_string, remove_arity};
    use std::ffi::CString;
    use std::path::PathBuf;

    pub struct Runner {
        state: *mut jq_state,
    }

    impl Runner {
        pub fn start() -> Runner {
            Runner {
                state: unsafe { jq_init() },
            }
        }

        pub fn add_library(&self, path: PathBuf) {
            unsafe {
                let libs = jv_array_append(jq_get_lib_dirs(self.state), jv_from_string(path.to_str().expect("a")));
                jq_set_attr(self.state, jv_from_string("JQ_LIBRARY_PATH"), libs);
            }
        }

        fn execute_code_with_no_input(&self, code: &str) -> Result<String, String> {
            let code_as_cstring = CString::new(code).expect("failure");

            unsafe {
                jq_compile(self.state, code_as_cstring.as_ptr());
                jq_start(self.state, jv_null(), 0);

                Ok(self.state)
                    .map(|value| jq_next(value))
                    .and_then(jv_to_result)
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

                jq_next(self.state)
                    .into_iter()
                    .map(jv_to_string)
                    .map(remove_arity)
                    .collect::<Vec<String>>()
            }
        }

        pub fn execute_test(&self, module: &str, test_name: &str) -> TestResult {
            let code = format!("include \"{}\"; {}", module, test_name);

            match self.execute_code_with_no_input(&code) {
                Ok(output) => TestResult {
                    module: String::from(module),
                    name: String::from(test_name),
                    pass: true,
                    output,
                },
                Err(output) => TestResult {
                    module: String::from(module),
                    name: String::from(test_name),
                    pass: false,
                    output,
                },
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct TestResult {
        pub module: String,
        pub name: String,
        pub pass: bool,
        pub output: String,
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
        runner.add_library(std::fs::canonicalize("./fixtures").expect("loaded fixtures"));

        assert_eq!(
            runner
                .execute_code_with_no_input("import \"simple_function\" as s; s::simple_function"),
            Ok(String::from("2"))
        );
    }

    #[test]
    fn should_return_error_if_exits_with_error() {
        let runner = Runner::start();
        runner.add_library(std::fs::canonicalize("./fixtures").expect("loaded fixtures"));

        assert_eq!(
            runner.execute_code_with_no_input("error(\"Failed to run\")"),
            Err(String::from("Failed to run"))
        );
    }

    #[test]
    fn should_load_list_of_functions_from_module() {
        let runner = Runner::start();
        runner.add_library(std::fs::canonicalize("./fixtures").expect("loaded fixtures"));

        assert_eq!(
            runner.get_functions_for_module("simple_function"),
            vec!["simple_function", "other_simple_function"]
        );
    }

    #[test]
    fn should_run_test() {
        let runner = Runner::start();
        runner.add_library(std::fs::canonicalize("./fixtures").expect("loaded fixtures"));

        assert_eq!(
            runner.execute_test("simple_function", "simple_function"),
            TestResult {
                module: String::from("simple_function"),
                name: String::from("simple_function"),
                pass: true,
                output: String::from("2")
            }
        );
    }

    #[test]
    fn should_run_failing_test() {
        let runner = Runner::start();
        runner.add_library(std::fs::canonicalize("./fixtures").expect("loaded fixtures"));

        assert_eq!(
            runner.execute_test("bad_module", "function_with_error"),
            TestResult {
                module: String::from("bad_module"),
                name: String::from("function_with_error"),
                pass: false,
                output: String::from("This is a bad function")
            }
        );
    }
}
