#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MultiLegPriceMethod {
    NET_PRICE = 0x0_u8, 
    REVERSED_NET_PRICE = 0x1_u8, 
    YIELD_DIFFERENCE = 0x2_u8, 
    INDIVIDUAL = 0x3_u8, 
    CONTRACT_WEIGHTED_AVERAGE_PRICE = 0x4_u8, 
    MULTIPLIED_PRICE = 0x5_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for MultiLegPriceMethod {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NET_PRICE, 
            0x1_u8 => Self::REVERSED_NET_PRICE, 
            0x2_u8 => Self::YIELD_DIFFERENCE, 
            0x3_u8 => Self::INDIVIDUAL, 
            0x4_u8 => Self::CONTRACT_WEIGHTED_AVERAGE_PRICE, 
            0x5_u8 => Self::MULTIPLIED_PRICE, 
            _ => Self::NullVal,
        }
    }
}
