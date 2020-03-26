use crate::{consts::msg, RangedI32};
use std::ops::{Sub, SubAssign};

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

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> SubAssign<i32> for RangedI32<START, END> {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self -= rhs
    }
}
