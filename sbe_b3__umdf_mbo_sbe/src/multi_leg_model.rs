#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MultiLegModel {
    PREDEFINED = 0x0_u8, 
    USER_DEFINED = 0x1_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for MultiLegModel {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::PREDEFINED, 
            0x1_u8 => Self::USER_DEFINED, 
            _ => Self::NullVal,
        }
    }
}
