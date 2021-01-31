use crate::{consts::msg, RangedI32};
use std::ops::{Add, AddAssign};

#[allow(clippy::use_self)]

impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Add<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
where
    RangedI32<{ START + START_RHS }, { END + END_RHS - 1 }>: ,
{
    #[allow(clippy::suspicious_arithmetic_impl)]
    type Output = RangedI32<{ START + START_RHS }, { END + END_RHS - 1 }>;

    // RangedI32::value cannot overflow at runtime because value is always between range
    // bounds.  Range bounds are const generics and are evaluated at compile
    // time--i.e. they will abort the compile if they overflow.  Therefore, since compiled range
    // bounds are always valid, then the sum of the contained values cannot overflow.
    #[allow(clippy::integer_arithmetic)]
    fn add(self, rhs: RangedI32<START_RHS, END_RHS>) -> Self::Output {
        Self::Output::new(self.value + rhs.value).unwrap_or_else(|| {
            unreachable!(msg::ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS)
        })
    }
}

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]

impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    AddAssign<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    #[inline]

    fn add_assign(&mut self, rhs: RangedI32<START_RHS, END_RHS>) { *self += rhs }
}
