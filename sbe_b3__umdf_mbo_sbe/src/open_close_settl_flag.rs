#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OpenCloseSettlFlag {
    DAILY = 0x0_u8, 
    SESSION = 0x1_u8, 
    EXPECTED_ENTRY = 0x3_u8, 
    ENTRY_FROM_PREVIOUS_BUSINESS_DAY = 0x4_u8, 
    THEORETICAL_PRICE = 0x5_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for OpenCloseSettlFlag {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::DAILY, 
            0x1_u8 => Self::SESSION, 
            0x3_u8 => Self::EXPECTED_ENTRY, 
            0x4_u8 => Self::ENTRY_FROM_PREVIOUS_BUSINESS_DAY, 
            0x5_u8 => Self::THEORETICAL_PRICE, 
            _ => Self::NullVal,
        }
    }
}
