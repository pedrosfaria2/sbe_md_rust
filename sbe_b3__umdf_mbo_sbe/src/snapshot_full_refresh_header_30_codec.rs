use crate::*;

pub use encoder::SnapshotFullRefresh_Header_30Encoder;
pub use decoder::SnapshotFullRefresh_Header_30Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 32;
pub const SBE_TEMPLATE_ID: u16 = 30;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct SnapshotFullRefresh_Header_30Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SnapshotFullRefresh_Header_30Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SnapshotFullRefresh_Header_30Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SnapshotFullRefresh_Header_30Encoder<'a> {
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

        /// primitive field 'lastMsgSeqNumProcessed'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 8
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn last_msg_seq_num_processed(&mut self, value: u32) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'totNumReports'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn tot_num_reports(&mut self, value: u32) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'totNumBids'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn tot_num_bids(&mut self, value: u32) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'totNumOffers'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 20
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn tot_num_offers(&mut self, value: u32) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'totNumStats'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 24
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn tot_num_stats(&mut self, value: u16) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'lastRptSeq'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 28
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn last_rpt_seq(&mut self, value: u32) {
            let offset = self.offset + 28;
            self.get_buf_mut().put_u32_at(offset, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SnapshotFullRefresh_Header_30Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for SnapshotFullRefresh_Header_30Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for SnapshotFullRefresh_Header_30Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SnapshotFullRefresh_Header_30Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SnapshotFullRefresh_Header_30Decoder<'a> {
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
            message_type::MessageType::MarketDataSnapshotFullRefresh
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_msg_seq_num_processed(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tot_num_reports(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 12)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tot_num_bids(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tot_num_offers(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 20)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tot_num_stats(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 24)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn last_rpt_seq(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 28);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

    }

} // end decoder

