use super::*;
use crate::{arith_helpers::*, OverflowingAdd};

impl<const START: i32, const END: i32> OverflowingAdd<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        let value_start_offset = signed_value_to_offset(START, self.value);
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
}
