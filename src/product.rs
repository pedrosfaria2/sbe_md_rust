#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Product {
    COMMODITY = 0x2_u8, 
    CORPORATE = 0x3_u8, 
    CURRENCY = 0x4_u8, 
    EQUITY = 0x5_u8, 
    GOVERNMENT = 0x6_u8, 
    INDEX = 0x7_u8, 
    ECONOMIC_INDICATOR = 0xf_u8, 
    MULTILEG = 0x10_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for Product {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x2_u8 => Self::COMMODITY, 
            0x3_u8 => Self::CORPORATE, 
            0x4_u8 => Self::CURRENCY, 
            0x5_u8 => Self::EQUITY, 
            0x6_u8 => Self::GOVERNMENT, 
            0x7_u8 => Self::INDEX, 
            0xf_u8 => Self::ECONOMIC_INDICATOR, 
            0x10_u8 => Self::MULTILEG, 
            _ => Self::NullVal,
        }
    }
}
