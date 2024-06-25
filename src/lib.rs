#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]

#![allow(ambiguous_glob_reexports)]

use ::core::{convert::TryInto};

pub mod aggressor_side;
pub mod appl_ver_id;
pub mod auction_imbalance_19_codec;
pub mod boolean;
pub mod channel_reset_11_codec;
pub mod closing_price_17_codec;
pub mod delete_order_mbo_51_codec;
pub mod empty_book_9_codec;
pub mod execution_statistics_56_codec;
pub mod execution_summary_55_codec;
pub mod exercise_style;
pub mod fixed_8_codec;
pub mod forward_trade_54_codec;
pub mod framing_header_codec;
pub mod governance_indicator;
pub mod group_size_encoding_codec;
pub mod header_message_0_codec;
pub mod high_price_24_codec;
pub mod imbalance_condition;
pub mod instr_attrib_type;
pub mod instr_attrib_value;
pub mod last_trade_price_27_codec;
pub mod lot_type;
pub mod low_price_25_codec;
pub mod mass_delete_orders_mbo_52_codec;
pub mod match_event_indicator;
pub mod maturity_month_year_codec;
pub mod md_entry_type;
pub mod md_update_action;
pub mod message_header_codec;
pub mod message_type;
pub mod multi_leg_model;
pub mod multi_leg_price_method;
pub mod news_5_codec;
pub mod news_source;
pub mod opening_price_15_codec;
pub mod open_close_settl_flag;
pub mod open_interest_29_codec;
pub mod order_mbo_50_codec;
pub mod packet_header_codec;
pub mod percentage_9_codec;
pub mod percentage_codec;
pub mod price_8_codec;
pub mod price_band_20_codec;
pub mod price_band_22_codec;
pub mod price_band_midpoint_price_type;
pub mod price_band_type;
pub mod price_codec;
pub mod price_limit_type;
pub mod price_offset_8_optional_codec;
pub mod price_optional_codec;
pub mod price_type;
pub mod product;
pub mod put_or_call;
pub mod quantity_band_21_codec;
pub mod ratio_qty_codec;
pub mod security_definition_12_codec;
pub mod security_definition_4_codec;
pub mod security_group_phase_10_codec;
pub mod security_id_source;
pub mod security_match_type;
pub mod security_status_3_codec;
pub mod security_trading_event;
pub mod security_trading_status;
pub mod security_type;
pub mod security_update_action;
pub mod sequence_2_codec;
pub mod sequence_reset_1_codec;
pub mod settlement_price_28_codec;
pub mod settl_price_type;
pub mod side;
pub mod snapshot_full_refresh_header_30_codec;
pub mod snapshot_full_refresh_orders_mbo_71_codec;
pub mod text_encoding_codec;
pub mod theoretical_opening_price_16_codec;
pub mod trade_53_codec;
pub mod trade_bust_57_codec;
pub mod trade_condition;
pub mod trading_session_id;
pub mod trading_session_sub_id;
pub mod trd_sub_type;
pub mod utc_timestamp_nanos_codec;
pub mod utc_timestamp_seconds_codec;
pub mod var_string_codec;

pub type SbeResult<T> = core::result::Result<T, SbeErr>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SbeErr {
    ParentNotSet,
}
impl core::fmt::Display for SbeErr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SbeErr {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait Writer<'a>: Sized {
    fn get_buf_mut(&mut self) -> &mut WriteBuf<'a>;
}

pub trait Encoder<'a>: Writer<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

pub trait ActingVersion {
    fn acting_version(&self) -> u16;
}

pub trait Reader<'a>: Sized {
    fn get_buf(&self) -> &ReadBuf<'a>;
}

pub trait Decoder<'a>: Reader<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadBuf<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> for ReadBuf<'a> {
    #[inline]
    fn get_buf(&self) -> &ReadBuf<'a> {
        self
    }
}
#[allow(dead_code)]
impl<'a> ReadBuf<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub(crate) fn get_bytes_at<const N: usize>(slice: &[u8], index: usize) -> [u8; N] {
        slice[index..index+N].try_into().expect("slice with incorrect length")
    }

    #[inline]
    pub fn get_u8_at(&self, index: usize) -> u8 {
        self.data[index]
    }

    #[inline]
    pub fn get_i8_at(&self, index: usize) -> i8 {
        i8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i16_at(&self, index: usize) -> i16 {
        i16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i32_at(&self, index: usize) -> i32 {
        i32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i64_at(&self, index: usize) -> i64 {
        i64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u16_at(&self, index: usize) -> u16 {
        u16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u32_at(&self, index: usize) -> u32 {
        u32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u64_at(&self, index: usize) -> u64 {
        u64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f32_at(&self, index: usize) -> f32 {
        f32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f64_at(&self, index: usize) -> f64 {
        f64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_slice_at(&self, index: usize, len: usize) -> &[u8] {
        &self.data[index..index+len]
    }

}

#[derive(Debug, Default)]
pub struct WriteBuf<'a> {
    data: &'a mut [u8],
}
impl<'a> WriteBuf<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn put_bytes_at<const COUNT: usize>(&mut self, index: usize, bytes: &[u8; COUNT]) -> usize {
        self.data[index..index + COUNT].copy_from_slice(bytes);
        COUNT
    }

    #[inline]
    pub fn put_u8_at(&mut self, index: usize, value: u8) {
        self.data[index] = value;
    }

    #[inline]
    pub fn put_i8_at(&mut self, index: usize, value: i8) {
        self.put_bytes_at(index, &i8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i16_at(&mut self, index: usize, value: i16) {
        self.put_bytes_at(index, &i16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i32_at(&mut self, index: usize, value: i32) {
        self.put_bytes_at(index, &i32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i64_at(&mut self, index: usize, value: i64) {
        self.put_bytes_at(index, &i64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u16_at(&mut self, index: usize, value: u16) {
        self.put_bytes_at(index, &u16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u32_at(&mut self, index: usize, value: u32) {
        self.put_bytes_at(index, &u32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u64_at(&mut self, index: usize, value: u64) {
        self.put_bytes_at(index, &u64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f32_at(&mut self, index: usize, value: f32) {
        self.put_bytes_at(index, &f32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f64_at(&mut self, index: usize, value: f64) {
        self.put_bytes_at(index, &f64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_slice_at(&mut self, index: usize, src: &[u8]) -> usize {
        let len = src.len();
        let dest = self.data.split_at_mut(index).1.split_at_mut(len).0;
        dest.clone_from_slice(src);
        len
    }
}

