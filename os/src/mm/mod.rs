mod address;
mod frame;
mod heap;
mod memory_set;
mod page_table;

use address::{PhysPageNum, SimpleRange, VirtAddr, VirtPageNum};
pub use frame::frame_allocm::frame_alloc;
pub use frame::frame_trackerm::FrameTracker;
use page_table::{PTEFlags, PageTable, PageTableEntry};

use memory_set::KERNEL_SPACE;
pub fn init() {
    heap::init_heap();
    heap::heap_test();
    frame::frame_allocm::init_frame_allocator();
    memory_set::KERNEL_SPACE.exclusive_access().activate();
}
