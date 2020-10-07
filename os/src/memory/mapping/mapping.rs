#[derive(Default)]
pub struct Mapping {
    page_table:Vec<page_tableTracker>,
    root_ppn: PhysicalPageNumber,
    mapped_pairs: VecDeque<(VirtualPageNumber,FrameTracker)>,
}

impl Mapping {
    pub fn new() -> MemoryResult<Mapping>{
        let root_table = page_tableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }
}