#[derive(Debug)]
pub enum MapType{
    Liner,
    Frame,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Segment {
    pub map_type:MapType,
    pub range:Range<<VirtualAddress>,
    pub flags:Flags, 
}

impl Segment {
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber> > {
        match self.map_type {
            MapType::Liner = Some(self.page_range().into().iter()),
            MapType::Frame => None,
        }
    }
}