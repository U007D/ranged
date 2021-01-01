use crate::{consts::msg, RangedI32};
use std::ops::{Add, AddAssign};

#[allow(clippy::use_self)]

// TODO: expose to users
macro_rules! add_impl {
    ($ty:ident, $start:expr, $end:expr, $start_rhs:expr, $end_rhs:expr) => {
        impl Add<RangedI32<$start_rhs, $end_rhs>> for RangedI32<$start, $end> {
            type Output = RangedI32<{ $start + $start_rhs }, { $end + $end_rhs }>;

            // RangedI32::value cannot overflow at runtime because value is always between range
            // bounds.  Range bounds are const generics and are evaluated at compile
            // time--i.e. they will abort the compile if they overflow.  Therefore, since compiled range
            // bounds are always valid, then the sum of the contained values cannot overflow.
            #[allow(clippy::integer_arithmetic)]

            fn add(self, rhs: RangedI32<$start_rhs, $end_rhs>) -> Self::Output {
                Self::Output::new(self.value + rhs.value).unwrap_or_else(|| {
                    unreachable!(msg::ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS)
                })
            }
        }
    }
}

add_impl!(RangedI32, 0, 100, -1000, 2000);

/* compile-time error: "attempt to add with overflow" */
// add_impl!(RangedI32, 0, i32::MAX, -1000, 2000);


// Suppress false positive recursion warning
#[allow(unconditional_recursion)]

impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    AddAssign<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    #[inline]

    fn add_assign(&mut self, rhs: RangedI32<START_RHS, END_RHS>) { *self += rhs }
}
