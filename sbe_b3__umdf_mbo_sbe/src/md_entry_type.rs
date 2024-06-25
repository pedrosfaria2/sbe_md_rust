#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MDEntryType {
    BID = 48_u8, 
    OFFER = 49_u8, 
    TRADE = 50_u8, 
    INDEX_VALUE = 51_u8, 
    OPENING_PRICE = 52_u8, 
    CLOSING_PRICE = 53_u8, 
    SETTLEMENT_PRICE = 54_u8, 
    SESSION_HIGH_PRICE = 55_u8, 
    SESSION_LOW_PRICE = 56_u8, 
    EXECUTION_STATISTICS = 57_u8, 
    IMBALANCE = 65_u8, 
    TRADE_VOLUME = 66_u8, 
    OPEN_INTEREST = 67_u8, 
    EMPTY_BOOK = 74_u8, 
    SECURITY_TRADING_STATE_PHASE = 99_u8, 
    PRICE_BAND = 103_u8, 
    QUANTITY_BAND = 104_u8, 
    COMPOSITE_UNDERLYING_PRICE = 68_u8, 
    EXECUTION_SUMMARY = 115_u8, 
    VOLATILITY_PRICE = 118_u8, 
    TRADE_BUST = 117_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for MDEntryType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::BID, 
            49_u8 => Self::OFFER, 
            50_u8 => Self::TRADE, 
            51_u8 => Self::INDEX_VALUE, 
            52_u8 => Self::OPENING_PRICE, 
            53_u8 => Self::CLOSING_PRICE, 
            54_u8 => Self::SETTLEMENT_PRICE, 
            55_u8 => Self::SESSION_HIGH_PRICE, 
            56_u8 => Self::SESSION_LOW_PRICE, 
            57_u8 => Self::EXECUTION_STATISTICS, 
            65_u8 => Self::IMBALANCE, 
            66_u8 => Self::TRADE_VOLUME, 
            67_u8 => Self::OPEN_INTEREST, 
            74_u8 => Self::EMPTY_BOOK, 
            99_u8 => Self::SECURITY_TRADING_STATE_PHASE, 
            103_u8 => Self::PRICE_BAND, 
            104_u8 => Self::QUANTITY_BAND, 
            68_u8 => Self::COMPOSITE_UNDERLYING_PRICE, 
            115_u8 => Self::EXECUTION_SUMMARY, 
            118_u8 => Self::VOLATILITY_PRICE, 
            117_u8 => Self::TRADE_BUST, 
            _ => Self::NullVal,
        }
    }
}
