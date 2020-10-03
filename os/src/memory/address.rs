use bit_field;
use bitflags;
impl VirtualPageNumber{
    //获取页号
    pub fn levels(self) -> [usize;3] {
        pub fn level(self) -> [usize;3]{
            [
                self.0.get_bits(18..27),
                self.0.get_bits(9..18),
                self.0.get_bits(0..9),
            ]
        }
    }
}