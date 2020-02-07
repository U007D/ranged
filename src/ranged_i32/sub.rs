use super::*;
use crate::{arith_helpers::*, OverflowingSub};
use std::{
    ops::Sub,
};

#[allow(clippy::use_self)]
impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Sub<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    type Output = RangedI32<
        { compute_bounds(START, END, START_RHS, END_RHS).0 },
        { compute_bounds(START, END, START_RHS, END_RHS).1 },
    >;

    // See `RangedI32::add()` impl for explanation of why no overflow checks need to be performed
    // here.
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

#[allow(clippy::integer_arithmetic)]
const fn compute_bounds(start: i32, end: i32, start_rhs: i32, end_rhs: i32) -> (i32, i32) {
    let new_start = start - start_rhs;
    let new_end = end - end_rhs;

    match new_start <= new_end {
        true => (new_start, new_end),
        false => (new_end - 1, new_start + 1),
    }
}