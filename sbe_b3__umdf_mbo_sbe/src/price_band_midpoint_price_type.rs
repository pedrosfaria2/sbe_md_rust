#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PriceBandMidpointPriceType {
    LAST_TRADED_PRICE = 0x0_u8, 
    COMPLEMENTARY_LAST_PRICE = 0x1_u8, 
    THEORETICAL_PRICE = 0x2_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for PriceBandMidpointPriceType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::LAST_TRADED_PRICE, 
            0x1_u8 => Self::COMPLEMENTARY_LAST_PRICE, 
            0x2_u8 => Self::THEORETICAL_PRICE, 
            _ => Self::NullVal,
        }
    }
}
