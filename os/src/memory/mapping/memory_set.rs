pub struct MemorySet {
    pub mapping: Mapping,
    pub segment: Vec<segment>,
}

impl MemorySet {
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        extern "c" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        let segment = vec![
            Segment {
                map_type:   MapType::Liner,
                range:      Range::from((text_start as usize)..(rodata_start as usize))
                flags:      Flags::READABLE | Flags::EXECUTABLE,
            },
            Segment {
                map_type:   MapType::Liner,
                range:      Range::from((rodata_start as usize)..(data_start as usize)),
                flags:      Flags::READABLE
            },
            Segment {
                map_type:   MapType::Liner,
                range:      Range::from(VirtualAddress::from(bss_start as usize)..*KERBEL_END_ADDRESS),
                flags:      Flags::READABLE | Flags::WRITABLE,
            },
            Segment {
                map_type:   MapType::Liner,
                range:      Range::from(*KERBEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags:      Flags::READABLE | Flags::WRITABLE,
            },
            Segment {
                map_type:   MapType::Linear,
                range:      Range::from(*KERBEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags:      Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;

        for segment in segments.iter() {
            mapping.map(segment,None)?;
        }
        Ok(MemorySet{mapping,segemts})
    }

    pub fn activate(&self) {
        self.mapping.activate()
    }
}