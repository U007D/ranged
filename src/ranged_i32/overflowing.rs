mod overflowing_add;
mod overflowing_sub;

use crate::RangedI32;
use arith_traits::Overflow;

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> Overflow<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    #[inline]
    fn overflowing_abs(self) -> Self::Output {
        self.overflowing_abs()
    }

    #[inline]
    #[must_use]
    fn overflowing_add(self, rhs: i32) -> Self::Output {
        self.overflowing_add(rhs)
    }

    #[inline]
    fn overflowing_div(self, rhs: i32) -> Self::Output {
        self.overflowing_div(rhs)
    }

    #[inline]
    fn overflowing_div_euclid(self, rhs: i32) -> Self::Output {
        self.overflowing_div_euclid(rhs)
    }

    #[inline]
    fn overflowing_mul(self, rhs: i32) -> Self::Output {
        self.overflowing_mul(rhs)
    }

    #[inline]
    fn overflowing_neg(self) -> Self::Output {
        self.overflowing_neg()
    }

    #[inline]
    fn overflowing_pow(self, rhs: u32) -> Self::Output {
        self.overflowing_pow(rhs)
    }

    #[inline]
    fn overflowing_rem(self, rhs: i32) -> Self::Output {
        self.overflowing_rem(rhs)
    }

    #[inline]
    fn overflowing_rem_euclid(self, rhs: i32) -> Self::Output {
        self.overflowing_rem_euclid(rhs)
    }

    #[inline]
    fn overflowing_shl(self, rhs: u32) -> Self::Output {
        self.overflowing_shl(rhs)
    }

    #[inline]
    fn overflowing_shr(self, rhs: u32) -> Self::Output {
        self.overflowing_shr(rhs)
    }

    #[inline]
    fn overflowing_sub(self, rhs: i32) -> Self::Output {
        self.overflowing_sub(rhs)
    }
}
