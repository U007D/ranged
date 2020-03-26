use crate::{consts::msg, RangedI32};
use std::ops::{Add, AddAssign};

impl<const START: i32, const END: i32> Add<i32> for RangedI32<START, END> {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        match self.overflowing_add(rhs) {
            (_, true) => panic!(
                "{}{}, {}{}.",
                msg::ERR_OVERFLOW_RANGED_I32_ADD_I32_A,
                START,
                END,
                msg::ERR_OVERFLOW_RANGED_I32_ADD_I32_B
            ),
            (value, _) => value,
        }
    }
}

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<const START: i32, const END: i32> AddAssign<i32> for RangedI32<START, END> {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self += rhs
    }
}
