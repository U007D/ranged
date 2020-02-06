use crate::consts::*;
use std::ops::Add;
use std::panic;

#[cfg(test)]
mod unit_tests;

#[derive(Debug)]
pub struct RangedI32<const START: i32, const END: i32> {
    value: i32,
}

impl<const START: i32, const END: i32> RangedI32<START, END> {
    const INVARIANT: () = assert!(START < END, msg::ERR_INVALID_RANGE_BOUNDS);
    const U32_MSB_MASK: u32 = 0x8000_0000;
    const RANGE_SPAN: u32 = Self::i32_to_u32(END) - Self::i32_to_u32(START);

    #[must_use]
    #[allow(clippy::let_unit_value)]
    pub const fn new(value: i32) -> Option<Self> {
        let _ = Self::INVARIANT;

        match START <= value && value < END {
            true => Some(Self { value }),
            false => None,
        }
    }

    #[inline]
    #[allow(clippy::integer_arithmetic, clippy::checked_conversions)]
    const fn wrapping_offset_to_value(offset: u32, base: i32) -> i32 {
        #[allow(clippy::cast_possible_wrap)]
        match offset <= (i32::max_value() as u32) {
            true => base.wrapping_add(offset as i32),
            false => base
                .wrapping_add(i32::max_value())
                .wrapping_add((offset - i32::max_value() as u32) as i32),
        }
    }

    #[inline]
    #[allow(clippy::integer_arithmetic)]
    const fn value_to_offset(base: i32, value: i32) -> u32 {
        let u_base = Self::i32_to_u32(base);
        let u_value = Self::i32_to_u32(value);
        Self::max(u_base, u_value) - Self::min(u_base, u_value)
    }

    #[inline]
    #[allow(clippy::integer_arithmetic)]
    const fn i32_to_u32(value: i32) -> u32 {
        match value >= 0 {
            #[allow(clippy::cast_sign_loss)]
            true => value as u32 | Self::U32_MSB_MASK,
            #[allow(clippy::cast_sign_loss)]
            false => (value - i32::min_value()) as u32,
        }
    }

    #[inline]
    const fn max(a: u32, b: u32) -> u32 {
        match a >= b {
            true => a,
            false => b,
        }
    }

    #[inline]
    const fn min(a: u32, b: u32) -> u32 {
        match a <= b {
            true => a,
            false => b,
        }
    }

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    pub fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        let value_start_offset = Self::value_to_offset(START, self.value);
        match rhs >= 0 {
            true => {
                #[allow(clippy::cast_sign_loss)]
                let u_rhs = rhs as u32;
                let (value, overflow) = value_start_offset.overflowing_add(u_rhs);
                match overflow {
                    true => (
                        Self::new(Self::wrapping_offset_to_value(
                            (u32::max_value() % Self::RANGE_SPAN + value % Self::RANGE_SPAN)
                                % Self::RANGE_SPAN,
                            START,
                        ))
                        .unwrap_or_else(|| unreachable!()),
                        true,
                    ),
                    false => (
                        Self::new(Self::wrapping_offset_to_value(
                            value % Self::RANGE_SPAN,
                            START,
                        ))
                        .unwrap_or_else(|| unreachable!()),
                        value / Self::RANGE_SPAN > 0,
                    ),
                }
            }
            false => unimplemented!(),
        }
    }
}

impl<const START: i32, const END: i32> Eq for RangedI32<START, END> {}

impl<const START: i32, const END: i32> PartialEq for RangedI32<START, END> {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

#[allow(clippy::use_self)]
impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Add<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    type Output = RangedI32<{ START + START_RHS }, { END + END_RHS }>;

    // Arithmetic cannot overflow at runtime because const generics are evaluated at compile time
    // and if they do not overflow, then the sum of the contained values cannot either.
    #[allow(clippy::integer_arithmetic)]
    fn add(self, rhs: RangedI32<START_RHS, END_RHS>) -> Self::Output {
        Self::Output::new(self.value + rhs.value)
            .expect(msg::ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS)
    }
}

impl<const START: i32, const END: i32> Add<i32> for RangedI32<START, END> {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        match self.overflowing_add(rhs) {
            (_, true) => panic!("Add<i32> for RangedI32<{}, {}> i32 overflow", START, END),
            (value, _) => value,
        }
    }
}
