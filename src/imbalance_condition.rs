#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImbalanceCondition(pub u16);
impl ImbalanceCondition {
    #[inline]
    pub fn new(value: u16) -> Self {
        ImbalanceCondition(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_imbalance_more_buyers(&self) -> bool {
        0 != self.0 & (1 << 8)
    }

    #[inline]
    pub fn set_imbalance_more_buyers(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 8)
        } else {
            self.0 & !(1 << 8)
        };
        self
    }

    #[inline]
    pub fn get_imbalance_more_sellers(&self) -> bool {
        0 != self.0 & (1 << 9)
    }

    #[inline]
    pub fn set_imbalance_more_sellers(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 9)
        } else {
            self.0 & !(1 << 9)
        };
        self
    }
}
impl core::fmt::Debug for ImbalanceCondition {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "ImbalanceCondition[imbalance_more_buyers(8)={},imbalance_more_sellers(9)={}]",
            self.get_imbalance_more_buyers(),self.get_imbalance_more_sellers(),)
    }
}
