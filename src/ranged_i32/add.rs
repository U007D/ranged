use super::{msg, panic, Add, RangedI32};
use crate::OverflowingAdd;

#[allow(clippy::use_self)]
impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Add<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    type Output = RangedI32<{ START + START_RHS }, { END + END_RHS }>;

    // RangedI32::value cannot overflow at runtime because value is always between range
    // bounds.  Range bounds are const generics and are evaluated at compile
    // time--i.e. they will abort the compile if they overflow.  As a result, since compiling range
    // bounds must be valid, then the sum of the contained values cannot overflow.
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
