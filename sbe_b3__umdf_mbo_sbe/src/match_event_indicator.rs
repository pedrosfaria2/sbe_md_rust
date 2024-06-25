#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchEventIndicator(pub u8);
impl MatchEventIndicator {
    #[inline]
    pub fn new(value: u8) -> Self {
        MatchEventIndicator(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_last_trade_msg(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_last_trade_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_last_volume_msg(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_last_volume_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }

    #[inline]
    pub fn get_last_quote_msg(&self) -> bool {
        0 != self.0 & (1 << 2)
    }

    #[inline]
    pub fn set_last_quote_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 2)
        } else {
            self.0 & !(1 << 2)
        };
        self
    }

    #[inline]
    pub fn get_last_stats_msg(&self) -> bool {
        0 != self.0 & (1 << 3)
    }

    #[inline]
    pub fn set_last_stats_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 3)
        } else {
            self.0 & !(1 << 3)
        };
        self
    }

    #[inline]
    pub fn get_last_implied_msg(&self) -> bool {
        0 != self.0 & (1 << 4)
    }

    #[inline]
    pub fn set_last_implied_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 4)
        } else {
            self.0 & !(1 << 4)
        };
        self
    }

    #[inline]
    pub fn get_recovery_msg(&self) -> bool {
        0 != self.0 & (1 << 5)
    }

    #[inline]
    pub fn set_recovery_msg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 5)
        } else {
            self.0 & !(1 << 5)
        };
        self
    }

    #[inline]
    pub fn get_reserved(&self) -> bool {
        0 != self.0 & (1 << 6)
    }

    #[inline]
    pub fn set_reserved(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 6)
        } else {
            self.0 & !(1 << 6)
        };
        self
    }

    #[inline]
    pub fn get_end_of_event(&self) -> bool {
        0 != self.0 & (1 << 7)
    }

    #[inline]
    pub fn set_end_of_event(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 7)
        } else {
            self.0 & !(1 << 7)
        };
        self
    }
}
impl core::fmt::Debug for MatchEventIndicator {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "MatchEventIndicator[last_trade_msg(0)={},last_volume_msg(1)={},last_quote_msg(2)={},last_stats_msg(3)={},last_implied_msg(4)={},recovery_msg(5)={},reserved(6)={},end_of_event(7)={}]",
            self.get_last_trade_msg(),self.get_last_volume_msg(),self.get_last_quote_msg(),self.get_last_stats_msg(),self.get_last_implied_msg(),self.get_recovery_msg(),self.get_reserved(),self.get_end_of_event(),)
    }
}
