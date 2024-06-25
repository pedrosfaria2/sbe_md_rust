#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PriceLimitType {
    PRICE_UNIT = 0x0_u8, 
    TICKS = 0x1_u8, 
    PERCENTAGE = 0x2_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for PriceLimitType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::PRICE_UNIT, 
            0x1_u8 => Self::TICKS, 
            0x2_u8 => Self::PERCENTAGE, 
            _ => Self::NullVal,
        }
    }
}
