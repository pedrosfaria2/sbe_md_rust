use crate::*;

pub use encoder::News_5Encoder;
pub use decoder::News_5Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 36;
pub const SBE_TEMPLATE_ID: u16 = 5;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct News_5Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for News_5Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for News_5Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> News_5Encoder<'a> {
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
        /// - null value: 0
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
        pub fn news_source(&mut self, value: news_source::NewsSource) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'languageCode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 10
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn language_code(&mut self, value: &[u8; 2]) {
            let offset = self.offset + 10;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'partCount'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn part_count(&mut self, value: u16) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'partNumber'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 14
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn part_number(&mut self, value: u16) {
            let offset = self.offset + 14;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'newsID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 16
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn news_id(&mut self, value: u64) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn orig_time_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// primitive field 'totalTextLength'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 32
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn total_text_length(&mut self, value: u32) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn headline(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut().put_slice_at(limit + 2, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn text(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut().put_slice_at(limit + 2, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn urll_ink(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut().put_slice_at(limit + 2, value.as_bytes());
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct News_5Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for News_5Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for News_5Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for News_5Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> News_5Decoder<'a> {
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
            message_type::MessageType::News
        }

        /// CONSTANT enum
        #[inline]
        pub fn appl_ver_id(&self) -> appl_ver_id::ApplVerID {
            appl_ver_id::ApplVerID::FIX50SP2
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn security_id(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset);
            if value == 0x0_u64 {
                None
            } else {
                Some(value)
            }
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
        pub fn news_source(&self) -> news_source::NewsSource {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        #[inline]
        pub fn language_code(&self) -> [u8; 2] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 10)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn part_count(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 12)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn part_number(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 14)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn news_id(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 16);
            if value == 0x0_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn orig_time_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn total_text_length(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 32)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn headline_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn headline_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn text_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn text_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn urll_ink_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn urll_ink_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

} // end decoder

