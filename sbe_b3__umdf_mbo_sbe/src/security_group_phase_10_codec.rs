use crate::*;

pub use encoder::SecurityGroupPhase_10Encoder;
pub use decoder::SecurityGroupPhase_10Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 32;
pub const SBE_TEMPLATE_ID: u16 = 10;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct SecurityGroupPhase_10Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SecurityGroupPhase_10Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SecurityGroupPhase_10Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityGroupPhase_10Encoder<'a> {
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

        /// primitive array field 'securityGroup'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 0
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn security_group(&mut self, value: &[u8; 3]) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        #[inline]
        pub fn match_event_indicator(&mut self, value: match_event_indicator::MatchEventIndicator) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&mut self, value: trading_session_id::TradingSessionID) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_sub_id(&mut self, value: trading_session_sub_id::TradingSessionSubID) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_event(&mut self, value: security_trading_event::SecurityTradingEvent) {
            let offset = self.offset + 11;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'tradeDate'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 12
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn trade_date(&mut self, value: u16) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn trad_ses_open_time_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 16;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn transact_time_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SecurityGroupPhase_10Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for SecurityGroupPhase_10Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for SecurityGroupPhase_10Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SecurityGroupPhase_10Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityGroupPhase_10Decoder<'a> {
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
            message_type::MessageType::SecurityStatus
        }

        /// CONSTANT enum
        #[inline]
        pub fn appl_ver_id(&self) -> appl_ver_id::ApplVerID {
            appl_ver_id::ApplVerID::FIX50SP2
        }

        #[inline]
        pub fn security_group(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset)
        }

        /// BIT SET DECODER
        #[inline]
        pub fn match_event_indicator(&self) -> match_event_indicator::MatchEventIndicator {
            match_event_indicator::MatchEventIndicator::new(self.get_buf().get_u8_at(self.offset + 8))
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&self) -> trading_session_id::TradingSessionID {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_sub_id(&self) -> trading_session_sub_id::TradingSessionSubID {
            self.get_buf().get_u8_at(self.offset + 10).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_event(&self) -> security_trading_event::SecurityTradingEvent {
            self.get_buf().get_u8_at(self.offset + 11).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn trade_date(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 12)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn trad_ses_open_time_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 16;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn transact_time_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

    }

} // end decoder

