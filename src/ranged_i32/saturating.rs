mod saturating_add;
mod saturating_sub;

use crate::RangedI32;
use arith_traits::Saturate;

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> Saturate<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    #[inline]
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }

    #[inline]
    #[must_use]
    fn saturating_add(self, rhs: i32) -> Self::Output {
        self.saturating_add(rhs)
    }

    #[inline]
    fn saturating_div(self, rhs: i32) -> Self::Output {
        self.saturating_div(rhs)
    }

    #[inline]
    fn saturating_div_euclid(self, rhs: i32) -> Self::Output {
        self.saturating_div_euclid(rhs)
    }

    #[inline]
    fn saturating_mul(self, rhs: i32) -> Self::Output {
        self.saturating_mul(rhs)
    }

    #[inline]
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }

    #[inline]
    fn saturating_pow(self, rhs: u32) -> Self::Output {
        self.saturating_pow(rhs)
    }

    #[inline]
    fn saturating_rem(self, rhs: i32) -> Self::Output {
        self.saturating_rem(rhs)
    }

    #[inline]
    fn saturating_rem_euclid(self, rhs: i32) -> Self::Output {
        self.saturating_rem_euclid(rhs)
    }

    #[inline]
    fn saturating_shl(self, rhs: u32) -> Self::Output {
        self.saturating_shl(rhs)
    }

    #[inline]
    fn saturating_shr(self, rhs: u32) -> Self::Output {
        self.saturating_shr(rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: i32) -> Self::Output {
        self.saturating_sub(rhs)
    }
}
