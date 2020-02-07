use super::*;
use crate::OverflowingSub;

impl<const START: i32, const END: i32> OverflowingSub<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    fn overflowing_sub(self, rhs: i32) -> (Self, bool) {
        let value_start_offset = signed_value_to_offset(START, self.value);
        match rhs >= 0 {
            true => {
                #[allow(clippy::cast_sign_loss)]
                let u_rhs = rhs as u32;
                let (value, overflow) = value_start_offset.overflowing_sub(u_rhs);
                match overflow {
                    true => (
                        Self::new(wrapping_offset_to_value(
                            (u32::max_value() % Self::RANGE_SPAN + value % Self::RANGE_SPAN)
                                % Self::RANGE_SPAN,
                            START,
                        ))
                        .unwrap_or_else(|| unreachable!()),
                        true,
                    ),
                    false => (
                        Self::new(wrapping_offset_to_value(value % Self::RANGE_SPAN, START))
                            .unwrap_or_else(|| unreachable!()),
                        value / Self::RANGE_SPAN > 0,
                    ),
                }
            }
            false => unimplemented!(),
        }
    }
}

#[inline]
#[allow(clippy::integer_arithmetic)]
pub(super) const fn i32_to_u32(value: i32) -> u32 {
    const I32_SIGN_BIT: u32 = 0x8000_0000;

    match value >= 0 {
        #[allow(clippy::cast_sign_loss)]
        true => value as u32 | I32_SIGN_BIT,
        #[allow(clippy::cast_sign_loss)]
        false => (value - i32::min_value()) as u32,
    }
}

#[inline]
const fn min(a: u32, b: u32) -> u32 {
    match a <= b {
        true => a,
        false => b,
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
#[allow(clippy::integer_arithmetic)]
const fn signed_value_to_offset(base: i32, value: i32) -> u32 {
    let u_base = i32_to_u32(base);
    let u_value = i32_to_u32(value);
    max(u_base, u_value) - min(u_base, u_value)
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
