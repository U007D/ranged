use super::{panic, RangedI32};
use crate::arith_helpers::{signed_value_to_offset, wrapping_offset_to_value};
use arith_traits::Overflow;

impl<const START: i32, const END: i32> Overflow<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    fn overflowing_abs(self) -> Self::Output {
        unimplemented!()
    }

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        let value_start_offset = signed_value_to_offset(START, self.value);
        let rhs = rhs;
        match rhs >= 0 {
            true => {
                #[allow(clippy::cast_sign_loss)]
                let u_rhs = rhs as u32;
                let (value, overflow) = value_start_offset.overflowing_add(u_rhs);
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

    fn overflowing_div(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_div_euclid(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_mul(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_neg(self) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_pow(self, rhs: u32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_rem(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_rem_euclid(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_shl(self, rhs: u32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_shr(self, rhs: u32) -> Self::Output {
        unimplemented!()
    }

    fn overflowing_sub(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }
}
