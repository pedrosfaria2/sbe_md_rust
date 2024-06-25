#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrAttribValue {
    ELECTRONIC_MATCH_OR_GTD_GTC_ELIGIBLE = 0x1_u8, 
    ORDER_CROSS_ELIGIBLE = 0x2_u8, 
    BLOCK_TRADE_ELIGIBLE = 0x3_u8, 
    FLAG_RFQ_FOR_CROSS_ELIGIBLE = 0xe_u8, 
    NEGOTIATED_QUOTE_ELIGIBLE = 0x11_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for InstrAttribValue {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::ELECTRONIC_MATCH_OR_GTD_GTC_ELIGIBLE, 
            0x2_u8 => Self::ORDER_CROSS_ELIGIBLE, 
            0x3_u8 => Self::BLOCK_TRADE_ELIGIBLE, 
            0xe_u8 => Self::FLAG_RFQ_FOR_CROSS_ELIGIBLE, 
            0x11_u8 => Self::NEGOTIATED_QUOTE_ELIGIBLE, 
            _ => Self::NullVal,
        }
    }
}
