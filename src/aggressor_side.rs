#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AggressorSide {
    NO_AGGRESSOR = 0x0_u8, 
    BUY = 0x1_u8, 
    SELL = 0x2_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for AggressorSide {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NO_AGGRESSOR, 
            0x1_u8 => Self::BUY, 
            0x2_u8 => Self::SELL, 
            _ => Self::NullVal,
        }
    }
}
