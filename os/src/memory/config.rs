use super::address::*;
use lazy_static::*;

pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
pub const PAGE_SIZE: usize = 4096;

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = VirtualAddress(kernel_end as usize); 
}

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;