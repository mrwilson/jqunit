mod jqunit {
    use std::ffi::{CStr, CString};
    use crate::jq::jq::{jq_compile, jq_init, jq_next, jq_start, jq_state, jv_null, jv_string_value};

    pub struct Runner {
        state: *mut jq_state
    }

    impl Runner {
        pub fn start() -> Runner {
            Runner {
                state: unsafe { jq_init() }
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

    #[test]
    fn should_execute_code_with_no_input() {
        assert_eq!(
            Runner::start().execute_code_with_no_input("\"hello\""),
            Some(String::from("hello"))
        );
    }

}