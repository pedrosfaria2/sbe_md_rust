#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SecurityTradingEvent {
    TRADING_SESSION_CHANGE = 0x4_u8, 
    SECURITY_STATUS_CHANGE = 0x65_u8, 
    SECURITY_REJOINS_SECURITY_GROUP_STATUS = 0x66_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for SecurityTradingEvent {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x4_u8 => Self::TRADING_SESSION_CHANGE, 
            0x65_u8 => Self::SECURITY_STATUS_CHANGE, 
            0x66_u8 => Self::SECURITY_REJOINS_SECURITY_GROUP_STATUS, 
            _ => Self::NullVal,
        }
    }
}
