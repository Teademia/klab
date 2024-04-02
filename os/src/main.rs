//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate bitflags;

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod mm;
mod sync;
mod config;

global_asm!(include_str!("entry.asm"));

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    lang_items::clear_bss();
    logging::init();
    debug!("Hello, world!");
    mm::init();
    sbi::shutdown(false)
}
