mod context;
pub use context::TrapContext;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::sbi;
use crate::syscall::syscall;
use crate::task::current_user_token;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

#[no_mangle]
pub fn trap_return() -> ! {
    info!("Reach TrapReturn");
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp: usize = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    debug!("Address of TRAPOLINE {:x}", TRAMPOLINE);
    debug!("Address of _alltraps {:x}", __alltraps as usize);
    debug!("Address of _restore {:x}", __restore as usize);
    debug!("Address of _restore_va {:x}", restore_va);
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}
#[no_mangle]
pub fn trap_handler() {
    debug!("Find trap_handler");
    sbi::shutdown(true);
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

pub fn init() {
    debug!("Set Kernel TrapHandler");
    set_kernel_trap_entry();
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}
fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}
