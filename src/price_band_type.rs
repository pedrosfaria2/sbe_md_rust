#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PriceBandType {
    HARD_LIMIT = 0x1_u8, 
    AUCTION_LIMITS = 0x2_u8, 
    REJECTION_BAND = 0x3_u8, 
    STATIC_LIMITS = 0x4_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for PriceBandType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::HARD_LIMIT, 
            0x2_u8 => Self::AUCTION_LIMITS, 
            0x3_u8 => Self::REJECTION_BAND, 
            0x4_u8 => Self::STATIC_LIMITS, 
            _ => Self::NullVal,
        }
    }
}
