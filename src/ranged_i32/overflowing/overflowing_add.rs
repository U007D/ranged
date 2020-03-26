use crate::arith_helpers::{signed_value_to_offset, wrapping_offset_to_value};
use crate::RangedI32;

impl<const START: i32, const END: i32> RangedI32<START, END> {
    #[allow(clippy::integer_arithmetic)]
    #[must_use]
    pub fn overflowing_add(self, rhs: i32) -> (Self, bool) {
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
}
