use crate::*;

pub use encoder::MaturityMonthYearEncoder;
pub use decoder::MaturityMonthYearDecoder;

pub const ENCODED_LENGTH: usize = 5;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MaturityMonthYearEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for MaturityMonthYearEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> MaturityMonthYearEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'year'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn year(&mut self, value: u16) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'month'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn month(&mut self, value: u8) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'day'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 3
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn day(&mut self, value: u8) {
            let offset = self.offset + 3;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'week'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn week(&mut self, value: u8) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u8_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MaturityMonthYearDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for MaturityMonthYearDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for MaturityMonthYearDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> MaturityMonthYearDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn year(&self) -> Option<u16> {
            let value = self.get_buf().get_u16_at(self.offset);
            if value == 0x0_u16 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn month(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 2);
            if value == 0x0_u8 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn day(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 3);
            if value == 0x0_u8 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn week(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 4);
            if value == 0x0_u8 {
                None
            } else {
                Some(value)
            }
        }

    }
} // end decoder mod 
