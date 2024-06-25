#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrAttribType {
    TRADE_TYPE_ELIGIBILITY = 0x18_u8, 
    GTD_GTC_ELIGIBILITY = 0x22_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for InstrAttribType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x18_u8 => Self::TRADE_TYPE_ELIGIBILITY, 
            0x22_u8 => Self::GTD_GTC_ELIGIBILITY, 
            _ => Self::NullVal,
        }
    }
}
