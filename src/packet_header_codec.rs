use crate::*;

pub use encoder::PacketHeaderEncoder;
pub use decoder::PacketHeaderDecoder;

pub const ENCODED_LENGTH: usize = 16;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PacketHeaderEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for PacketHeaderEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> PacketHeaderEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'channelNumber'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn channel_number(&mut self, value: u8) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'reserved'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn reserved(&mut self, value: u8) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'sequenceVersion'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn sequence_version(&mut self, value: u16) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'sequenceNumber'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn sequence_number(&mut self, value: u32) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'sendingTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn sending_time(&mut self, value: u64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PacketHeaderDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for PacketHeaderDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for PacketHeaderDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> PacketHeaderDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn channel_number(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn reserved(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 1)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn sequence_version(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 2)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn sequence_number(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 4)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn sending_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 8)
        }

    }
} // end decoder mod 
