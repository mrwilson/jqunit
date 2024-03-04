pub const jv_kind_JV_KIND_INVALID: jv_kind = 0;
pub const jv_kind_JV_KIND_STRING: jv_kind = 5;
pub const jv_kind_JV_KIND_ARRAY: jv_kind = 6;
pub type jv_kind = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jv_refcnt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jv {
    pub kind_flags: ::std::os::raw::c_uchar,
    pub pad_: ::std::os::raw::c_uchar,
    pub offset: ::std::os::raw::c_ushort,
    pub size: ::std::os::raw::c_int,
    pub u: jv__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jv__bindgen_ty_1 {
    pub ptr: *mut jv_refcnt,
    pub number: f64,
}
#[test]
fn bindgen_test_layout_jv__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<jv__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv__bindgen_ty_1),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).number) as usize - ptr as usize },
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
    const UNINIT: ::std::mem::MaybeUninit<jv> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<jv>(),
        16usize,
        concat!("Size of: ", stringify!(jv))
    );
    assert_eq!(
        ::std::mem::align_of::<jv>(),
        8usize,
        concat!("Alignment of ", stringify!(jv))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).kind_flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(kind_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad_) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(pad_))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(size))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(u))
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jq_state {
    _unused: [u8; 0],
}
extern "C" {
    pub fn jq_init() -> *mut jq_state;
    pub fn jq_compile(
        arg1: *mut jq_state,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn jq_start(arg1: *mut jq_state, value: jv, arg2: ::std::os::raw::c_int);
    pub fn jq_next(arg1: *mut jq_state) -> jv;
    pub fn jq_get_lib_dirs(arg1: *mut jq_state) -> jv;
    pub fn jq_set_attr(arg1: *mut jq_state, arg2: jv, arg3: jv);
    pub fn jv_dump_string(arg1: jv, flags: ::std::os::raw::c_int) -> jv;
    pub fn jv_string_value(arg1: jv) -> *const ::std::os::raw::c_char;
    pub fn jv_string(arg1: *const ::std::os::raw::c_char) -> jv;
    pub fn jv_array_append(arg1: jv, arg2: jv) -> jv;
    pub fn jv_array_length(arg1: jv) -> ::std::os::raw::c_int;
    pub fn jv_array_get(arg1: jv, arg2: ::std::os::raw::c_int) -> jv;
    pub fn jv_null() -> jv;
    pub fn jv_array() -> jv;
    pub fn jv_invalid_get_msg(arg1: jv) -> jv;
    pub fn jv_copy(arg1: jv) -> jv;
    pub fn jv_get_kind(arg1: jv) -> jv_kind;
}
