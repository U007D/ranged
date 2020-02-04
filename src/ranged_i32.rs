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
    const INVARIANT: () = assert!(START < END, msg::ERR_INVALID_RANGE_BOUNDS);
    const U32_MSB_MASK: u32 = 0x8000_0000;
    const RANGE_SPAN: u32 = Self::unsigned_prep_span(END) - Self::unsigned_prep_span(START);
    const WORD_OFFSET_1: i32 = match RANGE_SPAN > i32::max_value() as u32 {
        true => i32::max_value(),
        false => RANGE_SPAN as i32,
    };
    const WORD_OFFSET_2: i32 = match RANGE_SPAN > i32::max_value() as u32 {
        true => RANGE_SPAN - i32::max_value(),
        false => 0,
    };

    #[must_use]
    #[allow(clippy::let_unit_value)]
    pub const fn new(value: i32) -> Option<Self> {
        let _ = Self::INVARIANT;

        match START <= value && value < END {
            true => Some(Self { value }),
            false => None,
        }
    }

    const fn unsigned_prep_span(n: i32) -> u32 {
        #[allow(clippy::cast_sign_loss)]
        match n >= 0 {
            true => n as u32 | Self::U32_MSB_MASK,
            false => (n - i32::min_value()) as u32,
        }
    }

    #[must_use]
    #[allow(clippy::integer_arithmetic)]
    pub const fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        match Self::RANGE_SPAN > 0 {
            true => {
                let r_value = rhs % Self::RANGE_SPAN;
                match self.value < END - r_value {
                    // summed value within type's range
                    true => (
                        Self {
                            value: self.value + r_value,
                        },
                        rhs >= Self::RANGE_SPAN,
                    ),
                    false => {
                        match self.value < i32::max_value() - r_value {
                            // summed value within machine word range
                            true => (
                                Self {
                                    value: (self.value + r_value) % Self::RANGE_SPAN,
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
