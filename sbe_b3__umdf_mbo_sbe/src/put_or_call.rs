#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PutOrCall {
    PUT = 0x0_u8, 
    CALL = 0x1_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for PutOrCall {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::PUT, 
            0x1_u8 => Self::CALL, 
            _ => Self::NullVal,
        }
    }
}
