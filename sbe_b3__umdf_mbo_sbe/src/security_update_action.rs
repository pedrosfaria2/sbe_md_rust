#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SecurityUpdateAction {
    ADD = 65_u8, 
    DELETE = 68_u8, 
    MODIFY = 77_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for SecurityUpdateAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            65_u8 => Self::ADD, 
            68_u8 => Self::DELETE, 
            77_u8 => Self::MODIFY, 
            _ => Self::NullVal,
        }
    }
}
