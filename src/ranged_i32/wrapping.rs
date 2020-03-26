use super::RangedI32;
use arith_traits::{Overflow, Wrap};

impl<const START: i32, const END: i32> RangedI32<START, END> {
    #[must_use]
    #[inline]
    pub fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_add(self, rhs: i32) -> Self {
        self.overflowing_add(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_div(self, rhs: i32) -> Self {
        self.overflowing_div(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_div_euclid(self, rhs: i32) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_mul(self, rhs: i32) -> Self {
        self.overflowing_mul(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_pow(self, rhs: u32) -> Self {
        self.overflowing_pow(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_rem(self, rhs: i32) -> Self {
        self.overflowing_rem(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_rem_euclid(self, rhs: i32) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_shl(self, rhs: u32) -> Self {
        self.overflowing_shl(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_shr(self, rhs: u32) -> Self {
        self.overflowing_shr(rhs).0
    }

    #[must_use]
    #[inline]
    pub fn wrapping_sub(self, rhs: i32) -> Self {
        self.overflowing_sub(rhs).0
    }
}

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> Wrap<i32> for RangedI32<START, END> {
    type Output = Self;

    #[inline]
    fn wrapping_abs(self) -> Self::Output {
        self.wrapping_abs()
    }

    #[inline]
    fn wrapping_add(self, rhs: i32) -> Self::Output {
        self.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_div(self, rhs: i32) -> Self::Output {
        self.wrapping_div(rhs)
    }

    #[inline]
    fn wrapping_div_euclid(self, rhs: i32) -> Self::Output {
        self.wrapping_div_euclid(rhs)
    }

    #[inline]
    fn wrapping_mul(self, rhs: i32) -> Self::Output {
        self.wrapping_mul(rhs)
    }

    #[inline]
    fn wrapping_neg(self) -> Self::Output {
        self.wrapping_neg()
    }

    #[inline]
    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        self.wrapping_pow(rhs)
    }

    #[inline]
    fn wrapping_rem(self, rhs: i32) -> Self::Output {
        self.wrapping_rem(rhs)
    }

    #[inline]
    fn wrapping_rem_euclid(self, rhs: i32) -> Self::Output {
        self.wrapping_rem_euclid(rhs)
    }

    #[inline]
    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        self.wrapping_shl(rhs)
    }

    #[inline]
    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        self.wrapping_shr(rhs)
    }

    #[inline]
    fn wrapping_sub(self, rhs: i32) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}
