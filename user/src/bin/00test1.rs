#![no_std]
#![no_main]

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> i32 {
    println!("Hello user!");
    yield_();
    let mut a: usize = 1;
    loop {
        a = a + 1;
        println!("Timer from app1 {}", a);
        if a == 1000 {
            break;
        }
    }
    println!("Hello user");
    0
}
