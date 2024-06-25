#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TradingSessionSubID {
    PAUSE = 0x2_u8, 
    CLOSE = 0x4_u8, 
    OPEN = 0x11_u8, 
    FORBIDDEN = 0x12_u8, 
    UNKNOWN_OR_INVALID = 0x14_u8, 
    RESERVED = 0x15_u8, 
    FINAL_CLOSING_CALL = 0x65_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for TradingSessionSubID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x2_u8 => Self::PAUSE, 
            0x4_u8 => Self::CLOSE, 
            0x11_u8 => Self::OPEN, 
            0x12_u8 => Self::FORBIDDEN, 
            0x14_u8 => Self::UNKNOWN_OR_INVALID, 
            0x15_u8 => Self::RESERVED, 
            0x65_u8 => Self::FINAL_CLOSING_CALL, 
            _ => Self::NullVal,
        }
    }
}
