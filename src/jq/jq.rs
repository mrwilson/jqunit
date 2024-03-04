use std::mem::{align_of, size_of, MaybeUninit};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort};
use std::ptr;

pub const JV_KIND_INVALID: JvKind = 0;
pub const JV_KIND_STRING: JvKind = 5;
pub const JV_KIND_ARRAY: JvKind = 6;
pub type JvKind = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JvRefcnt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Jv {
    pub kind_flags: c_uchar,
    pub pad_: c_uchar,
    pub offset: c_ushort,
    pub size: c_int,
    pub u: jv__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union jv__bindgen_ty_1 {
    pub ptr: *mut JvRefcnt,
    pub number: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JqState {
    _unused: [u8; 0],
}

extern "C" {
    pub fn jq_init() -> *mut JqState;
    pub fn jq_compile(arg1: *mut JqState, arg2: *const c_char) -> c_int;
    pub fn jq_start(arg1: *mut JqState, value: Jv, arg2: c_int);
    pub fn jq_next(arg1: *mut JqState) -> Jv;
    pub fn jq_get_lib_dirs(arg1: *mut JqState) -> Jv;
    pub fn jq_set_attr(arg1: *mut JqState, arg2: Jv, arg3: Jv);
    pub fn jv_dump_string(arg1: Jv, flags: c_int) -> Jv;
    pub fn jv_string_value(arg1: Jv) -> *const c_char;
    pub fn jv_string(arg1: *const c_char) -> Jv;
    pub fn jv_array_append(arg1: Jv, arg2: Jv) -> Jv;
    pub fn jv_array_length(arg1: Jv) -> c_int;
    pub fn jv_array_get(arg1: Jv, arg2: c_int) -> Jv;
    pub fn jv_null() -> Jv;
    pub fn jv_array() -> Jv;
    pub fn jv_invalid_get_msg(arg1: Jv) -> Jv;
    pub fn jv_copy(arg1: Jv) -> Jv;
    pub fn jv_get_kind(arg1: Jv) -> JvKind;
}

#[test]
fn bindgen_test_layout_jv_bindgen_ty_1() {
    const UNINIT: MaybeUninit<jv__bindgen_ty_1> = MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        size_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        align_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv__bindgen_ty_1),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).number) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv__bindgen_ty_1),
            "::",
            stringify!(number)
        )
    );
}
#[test]
fn bindgen_test_layout_jv() {
    const UNINIT: MaybeUninit<Jv> = MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        size_of::<Jv>(),
        16usize,
        concat!("Size of: ", stringify!(jv))
    );
    assert_eq!(
        align_of::<Jv>(),
        8usize,
        concat!("Alignment of ", stringify!(jv))
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).kind_flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(kind_flags)
        )
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).pad_) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(pad_))
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(size))
    );
    assert_eq!(
        unsafe { ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(u))
    );
}
