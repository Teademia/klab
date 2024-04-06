//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.
use crate::task::exit_current_and_run_next;
use crate::{
    sbi::{self, shutdown},
    task,
};
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_SBRK: usize = 214;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => {
            debug!("System write");
            0
        }
        SYSCALL_EXIT => {
            task::show_task_status();
            info!("Going to run next");
            task::exit_current_and_run_next();
            0
        }
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
