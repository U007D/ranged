use crate::consts::*;
use std::ops::Add;
use std::panic;

#[cfg(test)]
mod unit_tests;

#[derive(Debug)]
pub struct RangedI32<const START: i32, const END: i32> {
    value: i32,
}

impl<const START: i32, const END: i32> RangedI32<START, END> {
    #[must_use]
    pub const fn new(value: i32) -> Option<Self> {
        // TODO: Upgrade to compile-time range bounds validity checking once supported/discovered
        match START <= END {
            true => match START <= value && value < END {
                true => Some(Self { value }),
                false => None,
            },
            false => panic!(msg::ERR_INVALID_RANGE_BOUNDS),
        }
    }

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    pub const fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        let span = END - START;

        match span > 0 {
            true => {
                let r_value = rhs % span;
                match self.value < END - r_value {
                    // summed value within type's range
                    true => (
                        Self {
                            value: self.value + r_value,
                        },
                        rhs >= span,
                    ),
                    false => {
                        match self.value < i32::max_value() - r_value {
                            // summed value within machine word range
                            true => (
                                Self {
                                    value: (self.value + r_value) % span,
                                },
                                true,
                            ),
                            // summed value overflows machine word range
                            false => {
                                let offset = START - i32::min_value();
                                (
                                    Self {
                                        value: (self.value.wrapping_add(r_value) + offset),
                                    },
                                    true,
                                )
                            }
                        }
                    }
                }
            }
            false => (self, true),
        }
    }
}

impl<const START: i32, const END: i32> Eq for RangedI32<START, END> {}

impl<const START: i32, const END: i32> PartialEq for RangedI32<START, END> {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

#[allow(clippy::use_self)]
impl<const START: i32, const END: i32, const START_RHS: i32, const END_RHS: i32>
    Add<RangedI32<START_RHS, END_RHS>> for RangedI32<START, END>
{
    type Output = RangedI32<{ START + START_RHS }, { END + END_RHS }>;

    // Arithmetic cannot overflow at runtime because const generics are evaluated at compile time
    // and if they do not overflow, then the sum of the contained values cannot either.
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
            (_, true) => panic!("Add<i32> for RangedI32<{}, {}> i32 overflow", START, END),
            (value, _) => value,
        }
    }
}
