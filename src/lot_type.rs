#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LotType {
    ODD_LOT = 0x1_u8, 
    ROUND_LOT = 0x2_u8, 
    BLOCK_LOT = 0x3_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for LotType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::ODD_LOT, 
            0x2_u8 => Self::ROUND_LOT, 
            0x3_u8 => Self::BLOCK_LOT, 
            _ => Self::NullVal,
        }
    }
}
