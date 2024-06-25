#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SecurityIDSource {
    ISIN = 52_u8, 
    EXCHANGE_SYMBOL = 56_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for SecurityIDSource {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            52_u8 => Self::ISIN, 
            56_u8 => Self::EXCHANGE_SYMBOL, 
            _ => Self::NullVal,
        }
    }
}
