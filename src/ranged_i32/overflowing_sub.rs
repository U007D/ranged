use super::{panic, RangedI32};
use crate::OverflowingSub;

impl<const START: i32, const END: i32> OverflowingSub<i32> for RangedI32<START, END> {
    type Output = (Self, bool);

    #[must_use]
    // The difference between two signed values when interpreted `as u32` (below) is the absolute
    // value of their difference, regardless of sign.  `as i32` below is non-lossy since `value`
    // is guaranteed to be within `i32::max_value()` elements of `START` (`value` is the result of a
    // subtraction of an `i32` from an `i32`) and thus these integer arithmetic operations cannot
    // overflow.
    #[allow(clippy::integer_arithmetic)]
    fn overflowing_sub(self, rhs: i32) -> (Self, bool) {
        match rhs >= 0 {
            true => {
                let (value, overflow) = self.value.overflowing_sub(rhs);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                match overflow {
                    true => (
                        Self::new(
                            END - (((START - i32::min_value()) as u32
                                + (i32::max_value() - value) as u32)
                                % Self::RANGE_SPAN) as i32
                                - 1,
                        )
                        .unwrap_or_else(|| unreachable!()),
                        true,
                    ),
                    false => match value < START {
                        true => (
                            Self::new(END - ((START - value) as u32 % Self::RANGE_SPAN) as i32)
                                .unwrap_or_else(|| unreachable!()),
                            true,
                        ),
                        false => (Self::new(value).unwrap_or_else(|| unreachable!()), false),
                    },
                }
            }
            false => unimplemented!(),
        }
    }
}
