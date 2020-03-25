use super::{msg, panic, RangedI32};
use arith_traits::Overflow;
use std::ops::Sub;

#[allow(clippy::use_self)]
impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Sub<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    type Output = RangedI32<{ START - END_RHS + 1 }, { END - START_RHS - 1 }>;

    // `sub()` is panic-safe ∵ `RangedI32::value` is always between range bounds (verified at
    // compile time).  ∴ the sum of the contained values cannot overflow.
    #[allow(clippy::integer_arithmetic)]
    fn sub(self, rhs: RangedI32<START_RHS, END_RHS>) -> Self::Output {
        Self::Output::new(self.value - rhs.value)
            .expect(msg::ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS)
    }
}

impl<const START: i32, const END: i32> Sub<i32> for RangedI32<START, END> {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        match self.overflowing_sub(rhs) {
            (_, true) => panic!(
                "{}{}, {}{}.",
                msg::ERR_OVERFLOW_RANGED_I32_SUB_I32_A,
                START,
                END,
                msg::ERR_OVERFLOW_RANGED_I32_SUB_I32_B
            ),
            (value, _) => value,
        }
    }
}
