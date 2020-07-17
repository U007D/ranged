mod add;
mod checked;
mod overflowing;
mod sub;
#[cfg(test)]
mod unit_tests;
mod wrapping;

use crate::{arith_helpers::i32_to_u32, consts::msg};
use std::panic;

#[derive(Debug)]

pub struct RangedI32<const START: i32, const END: i32> {
    value: i32,
}

impl<const START: i32, const END: i32> RangedI32<START, END> {
    const INVARIANT: () = assert!(START < END, msg::ERR_INVALID_RANGE_BOUNDS);
    const RANGE_SPAN: u32 = i32_to_u32(END) - i32_to_u32(START);

    #[must_use]
    #[allow(clippy::let_unit_value)]

    pub const fn new(value: i32) -> Option<Self> {
        let _ = Self::INVARIANT;

        match START <= value && value < END {
            true => Some(Self { value }),
            false => None,
        }
    }
}

impl<const START: i32, const END: i32> Eq for RangedI32<START, END> {}

impl<const START: i32, const END: i32> PartialEq for RangedI32<START, END> {
    #[inline]

    fn eq(&self, rhs: &Self) -> bool { self.value == rhs.value }
}
