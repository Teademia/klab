#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> i32 {
    println!("Hello user! from 3");
    let mut a: usize = 1;
    loop {
        a = a + 1;
        println!("Timer from app3 {}", a);
        if a == 1000 {
            break;
        }
    }
    0
}
