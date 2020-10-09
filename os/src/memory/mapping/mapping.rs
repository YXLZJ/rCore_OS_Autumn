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
    pub fn find_entry(&mut self , vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry>
    {
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..]{
            if entry.is_empty(){
                let new_table = page_tableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                *entry = PageTableEntry::new(Some(new_ppn),Flags::VALID);
                self.page_tables.push(new_table);
            }
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        Ok(entry) 
    }
    fn map_one(
        &mut self,
        vpn:VirtualPageNumber,
        ppn:Option<PhysicalPageNumber>,
        flags:Flags
    )-> MemoryResult<()>{
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(),"virtual address is already mapped");
        *entry = PageTableEntry::new(ppn,flags);
        Ok(())
    }
    
    pub fn map(&mut self,segment: &Segment,init_data:Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            MapType::Linear =>{
                for vpn in segment.page_range().iter(){
                    self.map_one(vpn,Some(vpn.into()), segment.flags)?;
                }
                if let Some(data) = init_data {
                    unsafe{
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(),data.len()))
                            .copy_from_slice(data)
                    }
                }
            }
            MapType::Framed => {
                for vpn in segment.page_range().iter(){
                    let mut page_data = [0u8;PAGE_SIZE];
                    if let Some(init_data) = init_data {
                        if !init_data.is_empty(){
                            let page_address = VirtualPageNumber::from(vpn);
                            let start = if segment.range.start >page_address {
                                segment.range.start - page_address
                            } else {
                                0
                            };
                            let stop = min(PAGE_SIZE,segment.range.end - page_address);
                            let dst_slice = &mut page_data[start..stop];
                            let src_slice = &init_data[(page_address + start - segment.range.start)
                                ..(page_address + stop -segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    };

                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    (*frame).copy_from_slice(&page_data);
                    self.mapped_pairs.push_back((vpn,frame));
                }
            }
        }
        Ok(())
    }
}