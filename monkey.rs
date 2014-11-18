#![crate_type = "dylib"]

extern crate libc;
use std::mem;

#[repr(C)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern fn dubble(value: i32) -> i32 {
    value * 2
}

extern {
    fn get_x(vec: Vec2) -> i32;
    fn get_ry(vec: *const Vec2) -> i32;
    fn set_x(vec: *mut Vec2, x: i32) -> ();
}

#[no_mangle]
pub extern fn mathy(vec: Vec2) -> i32 {
    unsafe {
        get_x(vec) * get_ry(&vec)
    }
}

#[no_mangle]
pub extern fn refy(vec: *mut Vec2) -> () {
    unsafe {
        (*vec).y = 110;
        set_x(vec, 9);
    }
}

#[no_mangle]
pub extern fn clone_vec(vec: *const Vec2) -> Vec2 {
    unsafe {
        let out = Vec2 {
            x: (*vec).x,
            y: (*vec).y,
        };

        out
    }
}

#[no_mangle]
pub extern fn new_vec(x: i32, y: i32) -> *mut Vec2 {
    unsafe {
        let vec: *mut Vec2 = libc::malloc(mem::size_of::<Vec2>() as libc::size_t) as *mut Vec2;
        if vec.is_null() {
            panic!("memory goin down!");
        }
        (*vec).x = x;
        (*vec).y = y;
        vec
    }
}
