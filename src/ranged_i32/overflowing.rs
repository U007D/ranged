mod overflowing_add;
mod overflowing_sub;

use crate::RangedI32;
use arith_traits::Overflow;

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> Overflow<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    fn overflowing_abs(self) -> Self::Output {
        unimplemented!()
    }

    #[inline]
    #[must_use]
    fn overflowing_add(self, rhs: i32) -> Self::Output {
        self.overflowing_add(rhs)
    }

    fn overflowing_div(self, _rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_div_euclid(self, _rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_mul(self, _rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_neg(self) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_pow(self, _rhs: u32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_rem(self, _rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_rem_euclid(self, _rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_shl(self, _rhs: u32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_shr(self, _rhs: u32) -> Self::Output {
        unimplemented!()
    }

    #[inline]
    fn overflowing_sub(self, rhs: i32) -> Self::Output {
        self.overflowing_sub(rhs)
    }
}
