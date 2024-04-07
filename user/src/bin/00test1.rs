#![no_std]
#![no_main]

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> i32 {
    println!("Hello user!");
    yield_();
    println!("Hello user");
    0
}
