extern crate libc;

mod jq;
mod runner;

use std::ffi::CStr;
use std::ffi::CString;

use crate::jq::jq::{jq_compile, jq_init, jq_next, jq_start, jq_teardown, jv_string, jv_string_value};

fn main() {

    unsafe {

        let mut jq = jq_init();

        let hello_world_string = jv_string(CString::new("Hello, world!").expect("whee").as_ptr());

        jq_compile(jq, CString::new(".").expect("whee").as_ptr());
        jq_start(jq, hello_world_string, 0);

        println!("{}", CStr::from_ptr(jv_string_value(jq_next(jq))).to_str().expect("whee"));
        jq_teardown(&mut jq);
    }

}
