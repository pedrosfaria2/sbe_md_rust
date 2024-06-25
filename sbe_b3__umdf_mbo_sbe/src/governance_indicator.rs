#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GovernanceIndicator {
    No = 0x0_u8, 
    N1 = 0x1_u8, 
    N2 = 0x2_u8, 
    NM = 0x4_u8, 
    MA = 0x5_u8, 
    MB = 0x6_u8, 
    M2 = 0x7_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for GovernanceIndicator {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::No, 
            0x1_u8 => Self::N1, 
            0x2_u8 => Self::N2, 
            0x4_u8 => Self::NM, 
            0x5_u8 => Self::MA, 
            0x6_u8 => Self::MB, 
            0x7_u8 => Self::M2, 
            _ => Self::NullVal,
        }
    }
}
