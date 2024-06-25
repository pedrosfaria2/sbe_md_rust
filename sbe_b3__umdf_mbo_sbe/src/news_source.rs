#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NewsSource {
    OTHER = 0x0_u8, 
    DCM = 0x1_u8, 
    BBMNET = 0x2_u8, 
    MARKET_SURVEILLANCE = 0x3_u8, 
    INTERNET = 0x4_u8, 
    DPR_VE = 0x5_u8, 
    MKT_OPS_FX_AGENCY = 0x13_u8, 
    MKT_OPS_DERIVATIVES_AGENCY = 0x14_u8, 
    OVER_THE_COUNTER_NEWS_AGENCY = 0xb_u8, 
    ELECTRONIC_PURCHASE_EXCHANGE = 0xd_u8, 
    CBLC_NEWS_AGENCY = 0xe_u8, 
    BOVESPA_INDEX_AGENCY = 0xf_u8, 
    BOVESPA_INSTITUTIONAL_AGENCY = 0x10_u8, 
    MKT_OPS_EQUITIES_AGENCY = 0x11_u8, 
    BOVESPA_COMPANIES_AGENCY = 0x12_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for NewsSource {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::OTHER, 
            0x1_u8 => Self::DCM, 
            0x2_u8 => Self::BBMNET, 
            0x3_u8 => Self::MARKET_SURVEILLANCE, 
            0x4_u8 => Self::INTERNET, 
            0x5_u8 => Self::DPR_VE, 
            0x13_u8 => Self::MKT_OPS_FX_AGENCY, 
            0x14_u8 => Self::MKT_OPS_DERIVATIVES_AGENCY, 
            0xb_u8 => Self::OVER_THE_COUNTER_NEWS_AGENCY, 
            0xd_u8 => Self::ELECTRONIC_PURCHASE_EXCHANGE, 
            0xe_u8 => Self::CBLC_NEWS_AGENCY, 
            0xf_u8 => Self::BOVESPA_INDEX_AGENCY, 
            0x10_u8 => Self::BOVESPA_INSTITUTIONAL_AGENCY, 
            0x11_u8 => Self::MKT_OPS_EQUITIES_AGENCY, 
            0x12_u8 => Self::BOVESPA_COMPANIES_AGENCY, 
            _ => Self::NullVal,
        }
    }
}
