#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ApplVerID {
    FIX27 = 0x0_u8, 
    FIX30 = 0x1_u8, 
    FIX40 = 0x2_u8, 
    FIX41 = 0x3_u8, 
    FIX42 = 0x4_u8, 
    FIX43 = 0x5_u8, 
    FIX44 = 0x6_u8, 
    FIX50 = 0x7_u8, 
    FIX50SP1 = 0x8_u8, 
    FIX50SP2 = 0x9_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for ApplVerID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::FIX27, 
            0x1_u8 => Self::FIX30, 
            0x2_u8 => Self::FIX40, 
            0x3_u8 => Self::FIX41, 
            0x4_u8 => Self::FIX42, 
            0x5_u8 => Self::FIX43, 
            0x6_u8 => Self::FIX44, 
            0x7_u8 => Self::FIX50, 
            0x8_u8 => Self::FIX50SP1, 
            0x9_u8 => Self::FIX50SP2, 
            _ => Self::NullVal,
        }
    }
}
