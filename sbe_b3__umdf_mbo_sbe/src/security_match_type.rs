#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SecurityMatchType {
    ISSUING_BUY_BACK_AUCTION = 0x8_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for SecurityMatchType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x8_u8 => Self::ISSUING_BUY_BACK_AUCTION, 
            _ => Self::NullVal,
        }
    }
}
