#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> usize {
    println!("Hello user! from 2");
    let mut a: usize = 1;
    loop {
        a = a + 1;
        println!("Timer from app2 {}", a);
        if a == 1000 {
            break;
        }
    }
    0
}
