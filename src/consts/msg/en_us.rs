#![allow(dead_code)]
pub const ERR_INVALID_INT_REPR: &str =
    "Error: String contents are not a valid representation of an integer";

pub const ERR_INVALID_RANGE_BOUNDS: &str =
    "Error: Invalid Range bounds specified.  Range start bound must be less than end bound";

pub const ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS: &str =
    "Internal error: Valid value(s) (within range bounds) unexpectedly detected outside of range \
     bounds following arithmetic operation.";

pub const ERR_OVERFLOW_RANGE_START: &str = "Error: Overflow of range `START` bound";
pub const ERR_OVERFLOW_RANGE_END: &str = "Error: Overflow of range `END` bound";

pub const ERR_OVERFLOW_RANGED_I32_ADD_I32_A: &str = "Add<i32> for RangedI32<";

pub const ERR_OVERFLOW_RANGED_I32_ADD_I32_B: &str = "> i32 overflow";

pub const ERR_OVERFLOW_RANGED_I32_SUB_I32_A: &str = "Sub<i32> for RangedI32<";

pub const ERR_OVERFLOW_RANGED_I32_SUB_I32_B: &str = "> i32 overflow";
