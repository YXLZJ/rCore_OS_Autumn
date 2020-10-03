use core;
#[derive(Copy,Clone,Default)]
pub struct PageTableEntry(usize);

const FLAG_GANGE: core::ops::Range<usize> = 0..8;

const PAGE_NUMBER_RANGE: core::ops::Range<usize>  = 10..54;

impl  PageTableEntry {
    pub fn new(page_number: Option<PhysicalPageNumber>,mut flags:flags) -> Self{
        flags.set(Flags::VALID,page_number.is_some());
        Self(
            *0size
                .set_bits(FLAG_RANGE,flags.bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE,page_number.unwrap_or_default().into()),
        )
    }
    pub fn undate_page_number(&mut self,ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_GANGE,(self.flags()|Flags::))
        }
    }
}