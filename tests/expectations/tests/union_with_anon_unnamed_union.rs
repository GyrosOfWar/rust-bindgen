/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy)]
pub union foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy)]
pub union foo__bindgen_ty_1 {
    pub b: ::std::os::raw::c_ushort,
    pub c: ::std::os::raw::c_uchar,
    _bindgen_union_align: u16,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 2usize , concat !
               ( "Size of: " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 2usize , concat
                ! ( "Alignment of " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . b as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( b ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . c as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( c ) ));
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 4usize , concat ! (
               "Size of: " , stringify ! ( foo ) ));
    assert_eq! (::std::mem::align_of::<foo>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( foo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . a as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo ) , "::" ,
                stringify ! ( a ) ));
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
impl Default for foo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
