use crate::*;

pub use encoder::Order_MBO_50Encoder;
pub use decoder::Order_MBO_50Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 64;
pub const SBE_TEMPLATE_ID: u16 = 50;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct Order_MBO_50Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for Order_MBO_50Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for Order_MBO_50Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> Order_MBO_50Encoder<'a> {
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

        /// REQUIRED enum
        #[inline]
        pub fn mdu_pdate_action(&mut self, value: md_update_action::MDUpdateAction) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn mde_ntry_type(&mut self, value: md_entry_type::MDEntryType) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_px_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 12;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// primitive field 'mDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 20
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn mde_ntry_size(&mut self, value: i64) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'mDEntryPositionNo'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 28
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn mde_ntry_position_no(&mut self, value: u32) {
            let offset = self.offset + 28;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'enteringFirm'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 32
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn entering_firm(&mut self, value: u32) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_insert_timestamp_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 36;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// primitive field 'secondaryOrderID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 44
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn secondary_order_id(&mut self, value: u64) {
            let offset = self.offset + 44;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'rptSeq'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 52
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn rpt_seq(&mut self, value: u32) {
            let offset = self.offset + 52;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_timestamp_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 56;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Order_MBO_50Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for Order_MBO_50Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for Order_MBO_50Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for Order_MBO_50Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> Order_MBO_50Decoder<'a> {
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

        /// REQUIRED enum
        #[inline]
        pub fn mdu_pdate_action(&self) -> md_update_action::MDUpdateAction {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn mde_ntry_type(&self) -> md_entry_type::MDEntryType {
            self.get_buf().get_u8_at(self.offset + 10).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_px_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 12;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mde_ntry_size(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 20)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mde_ntry_position_no(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 28)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn entering_firm(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 32);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_insert_timestamp_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 36;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn secondary_order_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 44)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn rpt_seq(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 52);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_timestamp_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 56;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

    }

} // end decoder

