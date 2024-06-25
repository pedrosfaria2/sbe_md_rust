#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Boolean {
    FALSE_VALUE = 0x0_u8, 
    TRUE_VALUE = 0x1_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for Boolean {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::FALSE_VALUE, 
            0x1_u8 => Self::TRUE_VALUE, 
            _ => Self::NullVal,
        }
    }
}
