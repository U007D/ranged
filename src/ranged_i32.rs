mod add;
mod overflowing_add;
//mod overflowing_sub;
mod sub;
#[cfg(test)]
mod unit_tests;

use crate::{arith_helpers::i32_to_u32, consts::msg};
use arith_traits::{Overflow, Wrap};
use std::panic;

#[derive(Debug)]
pub struct RangedI32<const START: i32, const END: i32> {
    value: i32,
}

impl<const START: i32, const END: i32> RangedI32<START, END> {
    const INVARIANT: () = assert!(START < END, msg::ERR_INVALID_RANGE_BOUNDS);
    const RANGE_SPAN: u32 = i32_to_u32(END) - i32_to_u32(START);

    #[must_use]
    #[allow(clippy::let_unit_value)]
    pub const fn new(value: i32) -> Option<Self> {
        let _ = Self::INVARIANT;

        match START <= value && value < END {
            true => Some(Self { value }),
            false => None,
        }
    }
}

impl<const START: i32, const END: i32> Eq for RangedI32<START, END> {}

impl<const START: i32, const END: i32> PartialEq for RangedI32<START, END> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

impl<const START: i32, const END: i32> Wrap<i32> for RangedI32<START, END> {
    type Output = Self;

    #[inline]
    fn wrapping_abs(self) -> Self::Output {
        self.overflowing_abs().0
    }

    #[inline]
    fn wrapping_add(self, rhs: i32) -> Self::Output {
        self.overflowing_add(rhs).0
    }

    #[inline]
    fn wrapping_div(self, rhs: i32) -> Self::Output {
        self.overflowing_div(rhs).0
    }

    #[inline]
    fn wrapping_div_euclid(self, rhs: i32) -> Self::Output {
        self.overflowing_div_euclid(rhs).0
    }

    #[inline]
    fn wrapping_mul(self, rhs: i32) -> Self::Output {
        self.overflowing_mul(rhs).0
    }

    #[inline]
    fn wrapping_neg(self) -> Self::Output {
        self.overflowing_neg().0
    }

    #[inline]
    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        self.overflowing_pow(rhs).0
    }

    #[inline]
    fn wrapping_rem(self, rhs: i32) -> Self::Output {
        self.overflowing_rem(rhs).0
    }

    #[inline]
    fn wrapping_rem_euclid(self, rhs: i32) -> Self::Output {
        self.overflowing_rem_euclid(rhs).0
    }

    #[inline]
    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        self.overflowing_shl(rhs).0
    }

    #[inline]
    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        self.overflowing_shr(rhs).0
    }

    #[inline]
    fn wrapping_sub(self, rhs: i32) -> Self::Output {
        self.overflowing_sub(rhs).0
    }
}
