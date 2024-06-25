#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SettlPriceType {
    FINAL = 0x1_u8, 
    THEORETICAL = 0x2_u8, 
    UPDATED = 0x3_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for SettlPriceType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::FINAL, 
            0x2_u8 => Self::THEORETICAL, 
            0x3_u8 => Self::UPDATED, 
            _ => Self::NullVal,
        }
    }
}
