use crate::*;

pub use encoder::SecurityDefinition_4Encoder;
pub use decoder::SecurityDefinition_4Decoder;

pub const SBE_BLOCK_LENGTH: u16 = 230;
pub const SBE_TEMPLATE_ID: u16 = 4;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 9;
pub const SBE_SEMANTIC_VERSION: &str = "1.8.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct SecurityDefinition_4Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SecurityDefinition_4Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SecurityDefinition_4Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityDefinition_4Encoder<'a> {
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

        /// primitive array field 'securityExchange'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: Exchange
        /// - encodedOffset: 8
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn security_exchange(&mut self, value: &[u8; 4]) {
            let offset = self.offset + 8;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_id_source(&mut self, value: security_id_source::SecurityIDSource) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'securityGroup'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 13
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn security_group(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 13;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'symbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn symbol(&mut self, value: &[u8; 20]) {
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_update_action(&mut self, value: security_update_action::SecurityUpdateAction) {
            let offset = self.offset + 36;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_type(&mut self, value: security_type::SecurityType) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'securitySubType'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn security_sub_type(&mut self, value: u16) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'totNoRelatedSym'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 40
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn tot_no_related_sym(&mut self, value: u32) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn min_price_increment_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 44;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn strike_price_encoder(self) -> price_optional_codec::PriceOptionalEncoder<Self> {
            let offset = self.offset + 52;
            price_optional_codec::PriceOptionalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn contract_multiplier_encoder(self) -> fixed_8_codec::Fixed8Encoder<Self> {
            let offset = self.offset + 60;
            fixed_8_codec::Fixed8Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_divisor_encoder(self) -> fixed_8_codec::Fixed8Encoder<Self> {
            let offset = self.offset + 68;
            fixed_8_codec::Fixed8Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn security_validity_timestamp_encoder(self) -> utc_timestamp_seconds_codec::UTCTimestampSecondsEncoder<Self> {
            let offset = self.offset + 76;
            utc_timestamp_seconds_codec::UTCTimestampSecondsEncoder::default().wrap(self, offset)
        }

        /// primitive field 'noSharesIssued'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 84
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn no_shares_issued(&mut self, value: u64) {
            let offset = self.offset + 84;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'clearingHouseID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 92
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn clearing_house_id(&mut self, value: u64) {
            let offset = self.offset + 92;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'minOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 100
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_order_qty(&mut self, value: i64) {
            let offset = self.offset + 100;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'maxOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 108
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn max_order_qty(&mut self, value: i64) {
            let offset = self.offset + 108;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'minLotSize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 116
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_lot_size(&mut self, value: i64) {
            let offset = self.offset + 116;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'minTradeVol'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 124
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn min_trade_vol(&mut self, value: i64) {
            let offset = self.offset + 124;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'corporateActionEventId'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 132
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn corporate_action_event_id(&mut self, value: u32) {
            let offset = self.offset + 132;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'issueDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 136
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn issue_date(&mut self, value: i32) {
            let offset = self.offset + 136;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'maturityDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 140
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn maturity_date(&mut self, value: i32) {
            let offset = self.offset + 140;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive array field 'countryOfIssue'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 144
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn country_of_issue(&mut self, value: &[u8; 2]) {
            let offset = self.offset + 144;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'startDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 146
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn start_date(&mut self, value: i32) {
            let offset = self.offset + 146;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'endDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 150
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn end_date(&mut self, value: i32) {
            let offset = self.offset + 150;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'settlType'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 154
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn settl_type(&mut self, value: u16) {
            let offset = self.offset + 154;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'settlDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 156
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn settl_date(&mut self, value: i32) {
            let offset = self.offset + 156;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'datedDate'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: LocalMktDate
        /// - encodedOffset: 160
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn dated_date(&mut self, value: i32) {
            let offset = self.offset + 160;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive array field 'isinNumber'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 164
        /// - encodedLength: 12
        /// - version: 0
        #[inline]
        pub fn isin_number(&mut self, value: &[u8; 12]) {
            let offset = self.offset + 164;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'asset'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 176
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn asset(&mut self, value: &[u8; 6]) {
            let offset = self.offset + 176;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'cfiCode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 182
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn cfi_code(&mut self, value: &[u8; 6]) {
            let offset = self.offset + 182;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn maturity_month_year_encoder(self) -> maturity_month_year_codec::MaturityMonthYearEncoder<Self> {
            let offset = self.offset + 188;
            maturity_month_year_codec::MaturityMonthYearEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn contract_settl_month_encoder(self) -> maturity_month_year_codec::MaturityMonthYearEncoder<Self> {
            let offset = self.offset + 193;
            maturity_month_year_codec::MaturityMonthYearEncoder::default().wrap(self, offset)
        }

        /// primitive array field 'currency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: Currency
        /// - encodedOffset: 198
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn currency(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 198;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'strikeCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: Currency
        /// - encodedOffset: 201
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn strike_currency(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 201;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'settlCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: Currency
        /// - encodedOffset: 204
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn settl_currency(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 204;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'securityStrategyType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 207
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn security_strategy_type(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 207;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn lot_type(&mut self, value: lot_type::LotType) {
            let offset = self.offset + 210;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'tickSizeDenominator'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 211
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn tick_size_denominator(&mut self, value: u8) {
            let offset = self.offset + 211;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn product(&mut self, value: product::Product) {
            let offset = self.offset + 212;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn exercise_style(&mut self, value: exercise_style::ExerciseStyle) {
            let offset = self.offset + 213;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn put_or_call(&mut self, value: put_or_call::PutOrCall) {
            let offset = self.offset + 214;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_type(&mut self, value: price_type::PriceType) {
            let offset = self.offset + 215;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'marketSegmentID'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 216
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn market_segment_id(&mut self, value: u8) {
            let offset = self.offset + 216;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn governance_indicator(&mut self, value: governance_indicator::GovernanceIndicator) {
            let offset = self.offset + 217;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_match_type(&mut self, value: security_match_type::SecurityMatchType) {
            let offset = self.offset + 218;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn last_fragment(&mut self, value: boolean::Boolean) {
            let offset = self.offset + 219;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn multi_leg_model(&mut self, value: multi_leg_model::MultiLegModel) {
            let offset = self.offset + 220;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn multi_leg_price_method(&mut self, value: multi_leg_price_method::MultiLegPriceMethod) {
            let offset = self.offset + 221;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'minCrossQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: Qty
        /// - encodedOffset: 222
        /// - encodedLength: 8
        /// - version: 6
        #[inline]
        pub fn min_cross_qty(&mut self, value: i64) {
            let offset = self.offset + 222;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// GROUP ENCODER (id=711, description='Underlying instruments.')
        #[inline]
        pub fn no_underlyings_encoder(self, count: u8, no_underlyings_encoder: NoUnderlyingsEncoder<Self>) -> NoUnderlyingsEncoder<Self> {
            no_underlyings_encoder.wrap(self, count)
        }

        /// GROUP ENCODER (id=555, description='Instrument legs.')
        #[inline]
        pub fn no_legs_encoder(self, count: u8, no_legs_encoder: NoLegsEncoder<Self>) -> NoLegsEncoder<Self> {
            no_legs_encoder.wrap(self, count)
        }

        /// GROUP ENCODER (id=870, description='Specifies the number of the application ID occurrences (number of channels).')
        #[inline]
        pub fn no_instr_attribs_encoder(self, count: u8, no_instr_attribs_encoder: NoInstrAttribsEncoder<Self>) -> NoInstrAttribsEncoder<Self> {
            no_instr_attribs_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn security_desc(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value.as_bytes());
        }

    }

    #[derive(Debug, Default)]
    pub struct NoUnderlyingsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoUnderlyingsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoUnderlyingsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoUnderlyingsEncoder<P> where P: Encoder<'a> + Default {
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
            44
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

        /// primitive field 'underlyingSecurityID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn underlying_security_id(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        // skipping CONSTANT enum 'underlyingSecurityIDSource'

        // skipping CONSTANT underlyingSecurityExchange

        /// COMPOSITE ENCODER
        #[inline]
        pub fn index_pct_encoder(self) -> percentage_9_codec::Percentage9Encoder<Self> {
            let offset = self.offset + 8;
            percentage_9_codec::Percentage9Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn index_theoretical_qty_encoder(self) -> fixed_8_codec::Fixed8Encoder<Self> {
            let offset = self.offset + 16;
            fixed_8_codec::Fixed8Encoder::default().wrap(self, offset)
        }

        /// primitive array field 'underlyingSymbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 24
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn underlying_symbol(&mut self, value: &[u8; 20]) {
            let offset = self.offset + 24;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoLegsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoLegsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoLegsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoLegsEncoder<P> where P: Encoder<'a> + Default {
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
            38
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

        /// primitive field 'legSecurityID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: Int
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn leg_security_id(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        // skipping CONSTANT enum 'legSecurityIDSource'

        // skipping CONSTANT legSecurityExchange

        /// COMPOSITE ENCODER
        #[inline]
        pub fn leg_ratio_qty_encoder(self) -> ratio_qty_codec::RatioQtyEncoder<Self> {
            let offset = self.offset + 8;
            ratio_qty_codec::RatioQtyEncoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn leg_security_type(&mut self, value: security_type::SecurityType) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn leg_side(&mut self, value: side::Side) {
            let offset = self.offset + 17;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'legSymbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: String
        /// - encodedOffset: 18
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn leg_symbol(&mut self, value: &[u8; 20]) {
            let offset = self.offset + 18;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoInstrAttribsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoInstrAttribsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoInstrAttribsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoInstrAttribsEncoder<P> where P: Encoder<'a> + Default {
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
            2
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

        /// REQUIRED enum
        #[inline]
        pub fn instr_attrib_type(&mut self, value: instr_attrib_type::InstrAttribType) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn instr_attrib_value(&mut self, value: instr_attrib_value::InstrAttribValue) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SecurityDefinition_4Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for SecurityDefinition_4Decoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for SecurityDefinition_4Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SecurityDefinition_4Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityDefinition_4Decoder<'a> {
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
            message_type::MessageType::SecurityDefinition
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

        #[inline]
        pub fn security_exchange(&self) -> [u8; 4] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_id_source(&self) -> security_id_source::SecurityIDSource {
            self.get_buf().get_u8_at(self.offset + 12).into()
        }

        #[inline]
        pub fn security_group(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 13)
        }

        #[inline]
        pub fn symbol(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 16)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_update_action(&self) -> security_update_action::SecurityUpdateAction {
            self.get_buf().get_u8_at(self.offset + 36).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_type(&self) -> security_type::SecurityType {
            self.get_buf().get_u8_at(self.offset + 37).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn security_sub_type(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 38)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tot_no_related_sym(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 40)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn min_price_increment_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 44;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn strike_price_decoder(self) -> price_optional_codec::PriceOptionalDecoder<Self> {
            let offset = self.offset + 52;
            price_optional_codec::PriceOptionalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn contract_multiplier_decoder(self) -> fixed_8_codec::Fixed8Decoder<Self> {
            let offset = self.offset + 60;
            fixed_8_codec::Fixed8Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_divisor_decoder(self) -> fixed_8_codec::Fixed8Decoder<Self> {
            let offset = self.offset + 68;
            fixed_8_codec::Fixed8Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn security_validity_timestamp_decoder(self) -> utc_timestamp_seconds_codec::UTCTimestampSecondsDecoder<Self> {
            let offset = self.offset + 76;
            utc_timestamp_seconds_codec::UTCTimestampSecondsDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn no_shares_issued(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 84);
            if value == 0x0_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn clearing_house_id(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 92);
            if value == 0x0_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn min_order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 100);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn max_order_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 108);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn min_lot_size(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 116);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn min_trade_vol(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 124);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn corporate_action_event_id(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 132);
            if value == 0x0_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn issue_date(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 136)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn maturity_date(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 140);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn country_of_issue(&self) -> [u8; 2] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 144)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn start_date(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 146);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn end_date(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 150);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '65535' }
        #[inline]
        pub fn settl_type(&self) -> Option<u16> {
            let value = self.get_buf().get_u16_at(self.offset + 154);
            if value == 0xffff_u16 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn settl_date(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 156);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn dated_date(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 160);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn isin_number(&self) -> [u8; 12] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 164)
        }

        #[inline]
        pub fn asset(&self) -> [u8; 6] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 176)
        }

        #[inline]
        pub fn cfi_code(&self) -> [u8; 6] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 182)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn maturity_month_year_decoder(self) -> maturity_month_year_codec::MaturityMonthYearDecoder<Self> {
            let offset = self.offset + 188;
            maturity_month_year_codec::MaturityMonthYearDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn contract_settl_month_decoder(self) -> maturity_month_year_codec::MaturityMonthYearDecoder<Self> {
            let offset = self.offset + 193;
            maturity_month_year_codec::MaturityMonthYearDecoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 198)
        }

        #[inline]
        pub fn strike_currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 201)
        }

        #[inline]
        pub fn settl_currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 204)
        }

        #[inline]
        pub fn security_strategy_type(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 207)
        }

        /// REQUIRED enum
        #[inline]
        pub fn lot_type(&self) -> lot_type::LotType {
            self.get_buf().get_u8_at(self.offset + 210).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '255' }
        #[inline]
        pub fn tick_size_denominator(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 211);
            if value == 0xff_u8 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn product(&self) -> product::Product {
            self.get_buf().get_u8_at(self.offset + 212).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn exercise_style(&self) -> exercise_style::ExerciseStyle {
            self.get_buf().get_u8_at(self.offset + 213).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn put_or_call(&self) -> put_or_call::PutOrCall {
            self.get_buf().get_u8_at(self.offset + 214).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn price_type(&self) -> price_type::PriceType {
            self.get_buf().get_u8_at(self.offset + 215).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn market_segment_id(&self) -> Option<u8> {
            let value = self.get_buf().get_u8_at(self.offset + 216);
            if value == 0x0_u8 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn governance_indicator(&self) -> governance_indicator::GovernanceIndicator {
            self.get_buf().get_u8_at(self.offset + 217).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_match_type(&self) -> security_match_type::SecurityMatchType {
            self.get_buf().get_u8_at(self.offset + 218).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn last_fragment(&self) -> boolean::Boolean {
            self.get_buf().get_u8_at(self.offset + 219).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn multi_leg_model(&self) -> multi_leg_model::MultiLegModel {
            self.get_buf().get_u8_at(self.offset + 220).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn multi_leg_price_method(&self) -> multi_leg_price_method::MultiLegPriceMethod {
            self.get_buf().get_u8_at(self.offset + 221).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn min_cross_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 222);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// GROUP DECODER (id=711, description='Underlying instruments.')
        #[inline]
        pub fn no_underlyings_decoder(self) -> NoUnderlyingsDecoder<Self> {
            NoUnderlyingsDecoder::default().wrap(self)
        }

        /// GROUP DECODER (id=555, description='Instrument legs.')
        #[inline]
        pub fn no_legs_decoder(self) -> NoLegsDecoder<Self> {
            NoLegsDecoder::default().wrap(self)
        }

        /// GROUP DECODER (id=870, description='Specifies the number of the application ID occurrences (number of channels).')
        #[inline]
        pub fn no_instr_attribs_decoder(self) -> NoInstrAttribsDecoder<Self> {
            NoInstrAttribsDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn security_desc_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn security_desc_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoUnderlyingsDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for NoUnderlyingsDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for NoUnderlyingsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoUnderlyingsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoUnderlyingsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
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

        /// group token - Token{signal=BEGIN_GROUP, name='noUnderlyings', referencedName='null', description='Underlying instruments.', packageName='null', id=711, version=0, deprecated=0, encodedLength=44, offset=230, componentTokenCount=33, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn underlying_security_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// CONSTANT enum
        #[inline]
        pub fn underlying_security_id_source(&self) -> security_id_source::SecurityIDSource {
            security_id_source::SecurityIDSource::EXCHANGE_SYMBOL
        }

        /// CONSTANT 
        /// characterEncoding: 'ASCII'
        #[inline]
        pub fn underlying_security_exchange(&self) -> &'static [u8] {
            b"BVMF"
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn index_pct_decoder(self) -> percentage_9_codec::Percentage9Decoder<Self> {
            let offset = self.offset + 8;
            percentage_9_codec::Percentage9Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn index_theoretical_qty_decoder(self) -> fixed_8_codec::Fixed8Decoder<Self> {
            let offset = self.offset + 16;
            fixed_8_codec::Fixed8Decoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn underlying_symbol(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 24)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoLegsDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for NoLegsDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for NoLegsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoLegsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoLegsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
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

        /// group token - Token{signal=BEGIN_GROUP, name='noLegs', referencedName='null', description='Instrument legs.', packageName='null', id=555, version=0, deprecated=0, encodedLength=38, offset=-1, componentTokenCount=54, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn leg_security_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// CONSTANT enum
        #[inline]
        pub fn leg_security_id_source(&self) -> security_id_source::SecurityIDSource {
            security_id_source::SecurityIDSource::EXCHANGE_SYMBOL
        }

        /// CONSTANT 
        /// characterEncoding: 'ASCII'
        #[inline]
        pub fn leg_security_exchange(&self) -> &'static [u8] {
            b"BVMF"
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn leg_ratio_qty_decoder(self) -> ratio_qty_codec::RatioQtyDecoder<Self> {
            let offset = self.offset + 8;
            ratio_qty_codec::RatioQtyDecoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn leg_security_type(&self) -> security_type::SecurityType {
            self.get_buf().get_u8_at(self.offset + 16).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn leg_side(&self) -> side::Side {
            self.get_buf().get_u8_at(self.offset + 17).into()
        }

        #[inline]
        pub fn leg_symbol(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 18)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoInstrAttribsDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for NoInstrAttribsDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for NoInstrAttribsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoInstrAttribsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoInstrAttribsDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
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

        /// group token - Token{signal=BEGIN_GROUP, name='noInstrAttribs', referencedName='null', description='Specifies the number of the application ID occurrences (number of channels).', packageName='null', id=870, version=0, deprecated=0, encodedLength=2, offset=-1, componentTokenCount=21, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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

        /// REQUIRED enum
        #[inline]
        pub fn instr_attrib_type(&self) -> instr_attrib_type::InstrAttribType {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn instr_attrib_value(&self) -> instr_attrib_value::InstrAttribValue {
            self.get_buf().get_u8_at(self.offset + 1).into()
        }

    }

} // end decoder

