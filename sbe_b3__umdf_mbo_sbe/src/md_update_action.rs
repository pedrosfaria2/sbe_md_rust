#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MDUpdateAction {
    NEW = 0x0_u8, 
    CHANGE = 0x1_u8, 
    DELETE = 0x2_u8, 
    DELETE_THRU = 0x3_u8, 
    DELETE_FROM = 0x4_u8, 
    OVERLAY = 0x5_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for MDUpdateAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NEW, 
            0x1_u8 => Self::CHANGE, 
            0x2_u8 => Self::DELETE, 
            0x3_u8 => Self::DELETE_THRU, 
            0x4_u8 => Self::DELETE_FROM, 
            0x5_u8 => Self::OVERLAY, 
            _ => Self::NullVal,
        }
    }
}
