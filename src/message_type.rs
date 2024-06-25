#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    Sequence = 48_u8, 
    SequenceReset = 52_u8, 
    MarketDataIncrementalRefresh = 88_u8, 
    SecurityStatus = 102_u8, 
    SecurityDefinition = 100_u8, 
    News = 66_u8, 
    MarketDataSnapshotFullRefresh = 87_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for MessageType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::Sequence, 
            52_u8 => Self::SequenceReset, 
            88_u8 => Self::MarketDataIncrementalRefresh, 
            102_u8 => Self::SecurityStatus, 
            100_u8 => Self::SecurityDefinition, 
            66_u8 => Self::News, 
            87_u8 => Self::MarketDataSnapshotFullRefresh, 
            _ => Self::NullVal,
        }
    }
}
