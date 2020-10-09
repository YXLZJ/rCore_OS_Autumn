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
                range:      Range::from()
            }
        ]
    }
}