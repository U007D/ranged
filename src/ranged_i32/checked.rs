use super::RangedI32;
use arith_traits::{Check, Overflow};

impl<const START: i32, const END: i32> RangedI32<START, END> {
    #[must_use]
    #[inline]
    pub fn checked_abs(self) -> Option<Self> {
        let (value, overflow) = self.overflowing_abs();
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_add(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_add(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_div(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_div_euclid(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_div_euclid(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_mul(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_neg(self) -> Option<Self> {
        let (value, overflow) = self.overflowing_neg();
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_pow(self, rhs: u32) -> Option<Self> {
        let (value, overflow) = self.overflowing_pow(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_rem(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_rem(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_rem_euclid(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_rem_euclid(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        let (value, overflow) = self.overflowing_shl(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        let (value, overflow) = self.overflowing_shr(rhs);
        overflow.then_some(value)
    }

    #[must_use]
    #[inline]
    pub fn checked_sub(self, rhs: i32) -> Option<Self> {
        let (value, overflow) = self.overflowing_sub(rhs);
        overflow.then_some(value)
    }
}

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> Check<i32> for RangedI32<START, END> {
    type Output = Option<Self>;

    #[inline]
    fn checked_abs(self) -> Self::Output {
        self.checked_abs()
    }

    #[inline]
    fn checked_add(self, rhs: i32) -> Self::Output {
        self.checked_add(rhs)
    }

    #[inline]
    fn checked_div(self, rhs: i32) -> Self::Output {
        self.checked_div(rhs)
    }

    #[inline]
    fn checked_div_euclid(self, rhs: i32) -> Self::Output {
        self.checked_div_euclid(rhs)
    }

    #[inline]
    fn checked_mul(self, rhs: i32) -> Self::Output {
        self.checked_mul(rhs)
    }

    #[inline]
    fn checked_neg(self) -> Self::Output {
        self.checked_neg()
    }

    #[inline]
    fn checked_pow(self, rhs: u32) -> Self::Output {
        self.checked_pow(rhs)
    }

    #[inline]
    fn checked_rem(self, rhs: i32) -> Self::Output {
        self.checked_rem(rhs)
    }

    #[inline]
    fn checked_rem_euclid(self, rhs: i32) -> Self::Output {
        self.checked_rem_euclid(rhs)
    }

    #[inline]
    fn checked_shl(self, rhs: u32) -> Self::Output {
        self.checked_shl(rhs)
    }

    #[inline]
    fn checked_shr(self, rhs: u32) -> Self::Output {
        self.checked_shr(rhs)
    }

    #[inline]
    fn checked_sub(self, rhs: i32) -> Self::Output {
        self.checked_sub(rhs)
    }
}
