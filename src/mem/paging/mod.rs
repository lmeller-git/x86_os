mod pagetable;
use bootloader::bootinfo::MemoryMap;
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Size4KiB},
    VirtAddr,
};

use crate::Result;
pub mod allocator;

pub unsafe fn init(
    phys_mem_offset: u64,
    mmap: &'static MemoryMap,
) -> Result<(impl Mapper<Size4KiB>, impl FrameAllocator<Size4KiB>)> {
    let offset = VirtAddr::new(phys_mem_offset);
    let mapper = pagetable::init(offset);
    let allocator = allocator::BootInfoFrameAllocator::init(mmap);
    Ok((mapper, allocator))
}
