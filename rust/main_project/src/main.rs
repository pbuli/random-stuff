/*
  Application using calls to rust static library and C static library
*/

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]

//our's rust lib
extern crate staticlib;
//crates.io libc wrapper
extern crate libc;


//repr(C) keeps C-style data layout
#[repr(C)]
// redeclaration of c struct, without definition
pub struct c_opaque_obj;

//redefinition of used c struct
#[repr(C)]
pub struct c_point {
    pub x: i32,
    pub y: i32,
}

//redefinition of used c enum
#[repr(C)]
enum c_enum {
    RED,
    GREEN,
    BLUE
}

// declarations of c library function
// link library by name, linking path provided in build.rs file
#[link(name = "testlibc", kind = "static")]
extern {
    fn c_multiply_four(input: libc::c_int) -> libc::c_int;
    fn c_add_point_values(input: c_point) -> libc::c_int;
    fn c_get_enum_string(input: c_enum) -> *const libc::c_char;

    fn c_create_obj(input: i32) -> c_opaque_obj;
    fn c_get_obj_val(input: c_opaque_obj) -> i32;

    fn c_malloc_obj(input: i32) -> *mut c_opaque_obj;
    fn c_get_obj_ptr_val(input: *mut c_opaque_obj) -> i32;

    fn c_malloc_void_obj(input: i32) -> *mut libc::c_void;
    fn c_get_obj_ptr_void_val(input: *mut libc::c_void) -> i32;
}

fn main() {
    println!("calling rust library function: {}", staticlib::increment(0));
    println!("calling rust library function with lib structure {}",
             staticlib::add_point_values(staticlib::Point { x: 1, y: 1 }));

    unsafe {
        println!("calling c library function: {}", c_multiply_four(1)-1);

        let p = c_point { x: 2, y: 2 };
        println!("calling c library function using redeclared struct: {}",
                 c_add_point_values(p));

        let obj1 = c_create_obj(5);
        println!("calling c library function using not-defined struct: {}",
                 c_get_obj_val(obj1));

        let obj2 = c_malloc_obj(6);
        println!("calling c library function using not-defined struct pointer: {}",
                 c_get_obj_ptr_val(obj2));

        let obj3 = c_malloc_void_obj(7);
        println!("calling c library function using c-style void ptr: {}",
                 c_get_obj_ptr_void_val(obj3));

        let c_raw_str = c_get_enum_string(c_enum::RED);
        // Causes SIGSEGV signal, for some reason does not crash running process...
        let c_str = std::ffi::CString::from_raw(c_raw_str as *mut libc::c_char);
        println!("calling c library function using redeclared enum, reciving c-style char pointer: {:?}",
                 c_str);

    }
}
