#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SecurityType {
    CASH = 0x1_u8, 
    CORP = 0x2_u8, 
    CS = 0x3_u8, 
    DTERM = 0x4_u8, 
    ETF = 0x5_u8, 
    FOPT = 0x6_u8, 
    FORWARD = 0x7_u8, 
    FUT = 0x8_u8, 
    INDEX = 0x9_u8, 
    INDEXOPT = 0xa_u8, 
    MLEG = 0xb_u8, 
    OPT = 0xc_u8, 
    OPTEXER = 0xd_u8, 
    PS = 0xe_u8, 
    SECLOAN = 0xf_u8, 
    SOPT = 0x10_u8, 
    SPOT = 0x11_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for SecurityType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::CASH, 
            0x2_u8 => Self::CORP, 
            0x3_u8 => Self::CS, 
            0x4_u8 => Self::DTERM, 
            0x5_u8 => Self::ETF, 
            0x6_u8 => Self::FOPT, 
            0x7_u8 => Self::FORWARD, 
            0x8_u8 => Self::FUT, 
            0x9_u8 => Self::INDEX, 
            0xa_u8 => Self::INDEXOPT, 
            0xb_u8 => Self::MLEG, 
            0xc_u8 => Self::OPT, 
            0xd_u8 => Self::OPTEXER, 
            0xe_u8 => Self::PS, 
            0xf_u8 => Self::SECLOAN, 
            0x10_u8 => Self::SOPT, 
            0x11_u8 => Self::SPOT, 
            _ => Self::NullVal,
        }
    }
}
