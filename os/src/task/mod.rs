mod context;
mod switch;
mod task;

use core::iter::Take;

use crate::loader::{get_app_data, get_num_app};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::collections::btree_map::Range;
use alloc::vec::Vec;
pub use context::TaskContext;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        debug!("init TASK_MANAGER");
        let num_app = get_num_app();
        debug!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            let a = get_app_data(i);
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }
        warn!("Finish Initialize TASK_MANAGER");
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let cur = inner.current_task;
        inner.tasks[cur].task_status = TaskStatus::Exited;
    }
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            info!("Going to execute task {}", next);
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_cx as *mut TaskContext;
            drop(inner);
            unsafe { __switch(current_task_cx_ptr, next_task_cx_ptr) }
        } else {
            info!("All applications completed!");
            shutdown(true);
        }
    }
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }
    fn show_task_status(&self) {
        let all = self.inner.exclusive_access().tasks.len();
        for i in 0..all {
            info!(
                "Task {} is {}",
                i,
                self.inner.exclusive_access().tasks[i].task_status
            );
        }
    }
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let next_task = &mut inner.tasks[0];
        next_task.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &next_task.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut _, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_user_token()
    }
    fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_trap_cx()
    }
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn run_first_task() {
    debug!("Ready To Go first Task");
    TASK_MANAGER.run_first_task();
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn show_task_status() {
    TASK_MANAGER.show_task_status();
}

pub fn exit_current_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}
