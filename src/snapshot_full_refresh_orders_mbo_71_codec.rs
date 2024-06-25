use crate::*;

pub use encoder::SnapshotFullRefresh_Orders_MBO_71Encoder;
pub use decoder::SnapshotFullRefresh_Orders_MBO_71Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 8;
pub const SBE_TEMPLATE_ID: u16 = 71;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct SnapshotFullRefresh_Orders_MBO_71Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SnapshotFullRefresh_Orders_MBO_71Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SnapshotFullRefresh_Orders_MBO_71Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SnapshotFullRefresh_Orders_MBO_71Encoder<'a> {
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

        /// GROUP ENCODER (id=268, description='Partial list of orders.')
        #[inline]
        pub fn no_md_entries_encoder(self, count: u8, no_md_entries_encoder: NoMDEntriesEncoder<Self>) -> NoMDEntriesEncoder<Self> {
            no_md_entries_encoder.wrap(self, count)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDEntriesEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoMDEntriesEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoMDEntriesEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDEntriesEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            41
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_px_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// primitive field 'mDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn mde_ntry_size(&mut self, value: i64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'mDEntryPositionNo'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 16
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn mde_ntry_position_no(&mut self, value: u32) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'enteringFirm'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 20
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn entering_firm(&mut self, value: u32) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_insert_timestamp_encoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// primitive field 'secondaryOrderID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 32
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn secondary_order_id(&mut self, value: u64) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn mde_ntry_type(&mut self, value: md_entry_type::MDEntryType) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SnapshotFullRefresh_Orders_MBO_71Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for SnapshotFullRefresh_Orders_MBO_71Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for SnapshotFullRefresh_Orders_MBO_71Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SnapshotFullRefresh_Orders_MBO_71Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SnapshotFullRefresh_Orders_MBO_71Decoder<'a> {
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

        /// GROUP DECODER (id=268, description='Partial list of orders.')
        #[inline]
        pub fn no_md_entries_decoder(self) -> NoMDEntriesDecoder<Self> {
            NoMDEntriesDecoder::default().wrap(self)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDEntriesDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for NoMDEntriesDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for NoMDEntriesDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoMDEntriesDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDEntriesDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='noMDEntries', referencedName='null', description='Partial list of orders.', packageName='null', id=268, version=0, deprecated=0, encodedLength=41, offset=8, componentTokenCount=55, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_px_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mde_ntry_size(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mde_ntry_position_no(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 16)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn entering_firm(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 20);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_insert_timestamp_decoder(self) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 24;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn secondary_order_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 32)
        }

        /// REQUIRED enum
        #[inline]
        pub fn mde_ntry_type(&self) -> md_entry_type::MDEntryType {
            self.get_buf().get_u8_at(self.offset + 40).into()
        }

    }

} // end decoder

