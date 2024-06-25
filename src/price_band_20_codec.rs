use crate::*;

pub use encoder::PriceBand_20Encoder;
pub use decoder::PriceBand_20Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 48;
pub const SBE_TEMPLATE_ID: u16 = 20;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct PriceBand_20Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for PriceBand_20Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for PriceBand_20Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PriceBand_20Encoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        // skipping CONSTANT enum 'messageType'

        // skipping CONSTANT enum 'applVerID'

        /// primitive field 'securityID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn security_id(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        // skipping CONSTANT enum 'securityIDSource'

        // skipping CONSTANT securityExchange

        #[inline]
        pub fn match_event_indicator(&mut self, value: match_event_indicator::MatchEventIndicator) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

        // skipping CONSTANT enum 'mDUpdateAction'

        // skipping CONSTANT enum 'mDEntryType'

        /// REQUIRED enum
        #[inline]
        pub fn price_band_type(&mut self, value: price_band_type::PriceBandType) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_limit_type(&mut self, value: price_limit_type::PriceLimitType) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_band_midpoint_price_type(&mut self, value: price_band_midpoint_price_type::PriceBandMidpointPriceType) {
            let offset = self.offset + 11;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn low_limit_price_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 12;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn high_limit_price_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 20;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn trading_reference_price_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 28;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_timestamp_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 36;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// primitive field 'rptSeq'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 44
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn rpt_seq(&mut self, value: u32) {
            let offset = self.offset + 44;
            self.get_buf_mut().put_u32_at(offset, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct PriceBand_20Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for PriceBand_20Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for PriceBand_20Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for PriceBand_20Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PriceBand_20Decoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// CONSTANT enum
        #[inline]
        pub fn message_type(&self) -> message_type::MessageType {
            message_type::MessageType::MarketDataIncrementalRefresh
        }

        /// CONSTANT enum
        #[inline]
        pub fn appl_ver_id(&self) -> appl_ver_id::ApplVerID {
            appl_ver_id::ApplVerID::FIX50SP2
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn security_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// CONSTANT enum
        #[inline]
        pub fn security_id_source(&self) -> security_id_source::SecurityIDSource {
            security_id_source::SecurityIDSource::EXCHANGE_SYMBOL
        }

        /// CONSTANT 
        /// characterEncoding: 'ASCII'
        #[inline]
        pub fn security_exchange(&self) -> &'static [u8] {
            b"BVMF"
        }

        /// BIT SET DECODER
        #[inline]
        pub fn match_event_indicator(&self) -> match_event_indicator::MatchEventIndicator {
            match_event_indicator::MatchEventIndicator::new(self.get_buf().get_u8_at(self.offset + 8))
        }

        /// CONSTANT enum
        #[inline]
        pub fn mdu_pdate_action(&self) -> md_update_action::MDUpdateAction {
            md_update_action::MDUpdateAction::NEW
        }

        /// CONSTANT enum
        #[inline]
        pub fn mde_ntry_type(&self) -> md_entry_type::MDEntryType {
            md_entry_type::MDEntryType::PRICE_BAND
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_band_type(&self) -> price_band_type::PriceBandType {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_limit_type(&self) -> price_limit_type::PriceLimitType {
            self.get_buf().get_u8_at(self.offset + 10).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_band_midpoint_price_type(&self) -> price_band_midpoint_price_type::PriceBandMidpointPriceType {
            self.get_buf().get_u8_at(self.offset + 11).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn low_limit_price_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 12;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn high_limit_price_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 20;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn trading_reference_price_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 28;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_timestamp_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 36;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn rpt_seq(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 44);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

    }

} // end decoder

