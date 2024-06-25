use crate::*;

pub use encoder::Price8Encoder;
pub use decoder::Price8Decoder;

pub const ENCODED_LENGTH: usize = 8;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Price8Encoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for Price8Encoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Price8Encoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'mantissa'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn mantissa(&mut self, value: i64) {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        // skipping CONSTANT exponent

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Price8Decoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for Price8Decoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for Price8Decoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Price8Decoder<P> where P: Reader<'a> + Default {
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
        pub fn mantissa(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// CONSTANT 
        #[inline]
        pub fn exponent(&self) -> i8 {
            -8
        }

    }
} // end decoder mod 
