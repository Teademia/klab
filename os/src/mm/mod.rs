mod address;
mod frame;
mod heap;
mod memory_set;
mod page_table;

pub use address::*;
pub use frame::frame_allocm::frame_alloc;
pub use frame::frame_trackerm::FrameTracker;
pub use memory_set::*;
pub use page_table::*;
pub fn init() {
    heap::init_heap();
    heap::heap_test();
    frame::frame_allocm::init_frame_allocator();
    memory_set::KERNEL_SPACE.exclusive_access().activate();
    memory_set::remap_test();
}
