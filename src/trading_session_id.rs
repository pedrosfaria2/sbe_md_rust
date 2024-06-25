#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TradingSessionID {
    REGULAR_TRADING_SESSION = 0x1_u8, 
    NON_REGULAR_TRADING_SESSION = 0x6_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for TradingSessionID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::REGULAR_TRADING_SESSION, 
            0x6_u8 => Self::NON_REGULAR_TRADING_SESSION, 
            _ => Self::NullVal,
        }
    }
}
