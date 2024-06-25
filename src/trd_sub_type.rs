#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TrdSubType {
    MULTI_ASSET_TRADE = 0x65_u8, 
    LEG_TRADE = 0x66_u8, 
    MIDPOINT_TRADE = 0x67_u8, 
    BLOCK_BOOK_TRADE = 0x68_u8, 
    RF_TRADE = 0x69_u8, 
    RLP_TRADE = 0x6a_u8, 
    TAC_TRADE = 0x6b_u8, 
    TAA_TRADE = 0x6c_u8, 
    SWEEP_TRADE = 0x6d_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for TrdSubType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x65_u8 => Self::MULTI_ASSET_TRADE, 
            0x66_u8 => Self::LEG_TRADE, 
            0x67_u8 => Self::MIDPOINT_TRADE, 
            0x68_u8 => Self::BLOCK_BOOK_TRADE, 
            0x69_u8 => Self::RF_TRADE, 
            0x6a_u8 => Self::RLP_TRADE, 
            0x6b_u8 => Self::TAC_TRADE, 
            0x6c_u8 => Self::TAA_TRADE, 
            0x6d_u8 => Self::SWEEP_TRADE, 
            _ => Self::NullVal,
        }
    }
}
