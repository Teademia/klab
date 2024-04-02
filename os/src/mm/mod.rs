mod heap;
mod frame;
mod address;

pub fn init() {
    heap::init_heap();
    heap::heap_test();
    frame::init_frame_allocator();
    frame::frame_allocator_test();
}