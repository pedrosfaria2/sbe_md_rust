#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExerciseStyle {
    EUROPEAN = 0x0_u8, 
    AMERICAN = 0x1_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for ExerciseStyle {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::EUROPEAN, 
            0x1_u8 => Self::AMERICAN, 
            _ => Self::NullVal,
        }
    }
}
