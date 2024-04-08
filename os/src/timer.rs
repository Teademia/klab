//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

pub fn get_time() -> usize {
    time::read()
}

/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn set_next_trigger() {
    //mtime寄存器是计时器
    //mtimecmp当mtime=mtimecmp时就会触发中断

    //首先读取mtime的值，get_time()
    //Clock_Frequency是一个时钟频率单位是hz,是一秒内mtime的增量
    //我们想10ms触发一个时钟中断，就要每次触发中断后给mtimecmp增加相应的值
    //10ms一次，就是一秒触发100次，就是把CLOCK_FREQ分成一百次加给mtimercmp
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
