use super::super::address::PhysPageNum;
use super::frame_alloc::*;
use core::fmt::{self, Debug, Formatter};
use log::*;
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}


impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {

    fn drop(&mut self) {
        debug!("FrameTracker Drop: {:x}", self.ppn.0);
        frame_dealloc(self.ppn);
    }
}