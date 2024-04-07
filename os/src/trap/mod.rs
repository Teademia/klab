mod context;
pub use context::TrapContext;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::sbi;
use crate::syscall::syscall;
use crate::task::{current_trap_cx, current_user_token};
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

#[no_mangle]
pub fn trap_return() -> ! {
    //执行第一个任务时
    //现在是内核地址空间
    //下面这个函数是设置stvec的值，设置为TrapoLine跳板的值
    //之后restore
    //restore的过程中做了几件事
    //1:切换地址空间到用户空间
    //2.设置sepc的值返回到程序开始的地方

    //当函数触发trap的时候，会跳转到stvec寄存器指定的值，就是TrapoLine
    //alltraps会保存用户态的信息，跳转到trap_handler
    //trap_handler进行的第一件事情就是将stvec设置为内核所对应的trap_handler,这里我们是用一个panic代替
    //trap_handler最后会调用trap_returu,所以要把trap_handler设置为用户状态的trap_handler(从alltraps)进入
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT; //用户地址空间保存TrapContext的虚拟地址
    let user_satp: usize = current_user_token(); //用户地址空间的satp寄存器地值
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
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
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

pub fn init() {
    set_kernel_trap_entry(); //Set Kernel Trap Entry to panic
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
