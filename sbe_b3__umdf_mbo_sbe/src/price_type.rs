#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PriceType {
    PERCENTAGE = 0x1_u8, 
    PU = 0x2_u8, 
    FIXED_AMOUNT = 0x3_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for PriceType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::PERCENTAGE, 
            0x2_u8 => Self::PU, 
            0x3_u8 => Self::FIXED_AMOUNT, 
            _ => Self::NullVal,
        }
    }
}
