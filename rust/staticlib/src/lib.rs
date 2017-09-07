/*
  rust static library, will be used by rust application
  due to toml specification will also create c staticlib, used by c app
*/

mod file;
use file::another_file_function;

extern crate libc;

pub fn addition() {
    println!("Library function!");
    println!("Calling function from another file.");
    println!("4 + {:3.3} = {}", another_file_function(), 4 + another_file_function());
}

#[no_mangle]
pub extern "C" fn increment(var: i32) -> i32 {
    var + 1
}

#[no_mangle]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[no_mangle]
pub extern "C" fn add_point_values(p: Point) -> i32 {
    p.x + p.y
}

pub enum Person {
    // An `enum` may either be `unit-like`,
    Engineer,
    Scientist,
    // like tuple structs,
    Height(i32),
    Weight(i32),
    // or like structures.
    Info { name: String, height: i32 }
}

fn inspect(p: &Person) {
    // Usage of an `enum` must cover all cases (irrefutable)
    // so a `match` is used to branch over it.
    match p {
        &Person::Engineer  => println!("Is an engineer!"),
        &Person::Scientist => println!("Is a scientist!"),
        // Destructure `i` from inside the `enum`.
        &Person::Height(i) => println!("Has a height of {}.", i),
        &Person::Weight(i) => println!("Has a weight of {}.", i),
        // Destructure `Info` into `name` and `height`.
        &Person::Info { ref name, ref height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

#[no_mangle]
pub struct complicated_rust_struct {
    pub x: Person,
    pub y: Person,
    pub z: Person,
}

impl complicated_rust_struct {
    pub fn use_struct_fields_method(&self) {
        inspect(&self.x);
        inspect(&self.y);
        inspect(&self.z);
    }
}

#[no_mangle]
pub extern "C" fn use_struct_fields_function(strc: *mut complicated_rust_struct) {
    unsafe {
        println!("Library function!");
        (*strc).use_struct_fields_method();
    }
}

#[no_mangle]
pub extern "C" fn create_struct_ptr_function() -> *mut complicated_rust_struct {
    unsafe {
        let ptr: *mut complicated_rust_struct =
            libc::malloc(std::mem::size_of::<complicated_rust_struct>()) as *mut complicated_rust_struct;
        (*ptr).y = Person::Scientist;
        (*ptr).z = Person::Info {name: "Status".to_owned(), height: 184};
        return ptr;
    }
    // free pointer somewhere later
}

#[no_mangle]
pub extern "C" fn free_struct_ptr_function(ptr: *mut complicated_rust_struct) {
    unsafe {
        libc::free(ptr as *mut libc::c_void);
    }
}
