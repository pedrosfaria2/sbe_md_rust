#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TradeCondition(pub u16);
impl TradeCondition {
    #[inline]
    pub fn new(value: u16) -> Self {
        TradeCondition(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_opening_price(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_opening_price(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_crossed(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_crossed(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }

    #[inline]
    pub fn get_last_trade_at_the_same_price(&self) -> bool {
        0 != self.0 & (1 << 2)
    }

    #[inline]
    pub fn set_last_trade_at_the_same_price(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 2)
        } else {
            self.0 & !(1 << 2)
        };
        self
    }

    #[inline]
    pub fn get_out_of_sequence(&self) -> bool {
        0 != self.0 & (1 << 3)
    }

    #[inline]
    pub fn set_out_of_sequence(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 3)
        } else {
            self.0 & !(1 << 3)
        };
        self
    }

    #[inline]
    pub fn get_trade_on_behalf(&self) -> bool {
        0 != self.0 & (1 << 6)
    }

    #[inline]
    pub fn set_trade_on_behalf(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 6)
        } else {
            self.0 & !(1 << 6)
        };
        self
    }

    #[inline]
    pub fn get_regular_trade(&self) -> bool {
        0 != self.0 & (1 << 13)
    }

    #[inline]
    pub fn set_regular_trade(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 13)
        } else {
            self.0 & !(1 << 13)
        };
        self
    }

    #[inline]
    pub fn get_block_trade(&self) -> bool {
        0 != self.0 & (1 << 14)
    }

    #[inline]
    pub fn set_block_trade(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 14)
        } else {
            self.0 & !(1 << 14)
        };
        self
    }
}
impl core::fmt::Debug for TradeCondition {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "TradeCondition[opening_price(0)={},crossed(1)={},last_trade_at_the_same_price(2)={},out_of_sequence(3)={},trade_on_behalf(6)={},regular_trade(13)={},block_trade(14)={}]",
            self.get_opening_price(),self.get_crossed(),self.get_last_trade_at_the_same_price(),self.get_out_of_sequence(),self.get_trade_on_behalf(),self.get_regular_trade(),self.get_block_trade(),)
    }
}
