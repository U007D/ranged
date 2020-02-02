#![allow(dead_code)]
pub const ERR_INVALID_INT_REPR: &str = "Error: String contents are not a valid representation of \
                                        an integer";
pub const ERR_INVALID_RANGE_BOUNDS: &str =
    "Error: Invalid Range bounds specified.  Range start bound is greater than end bound";
pub const ERR_INTERNAL_VALUE_UNEXPECTEDLY_EXCEEDED_RANGE_BOUNDS: &str =
    "Internal error: Valud value(s) (within range bounds) unexpectedly detected outside of range \
    bounds following arithmetic operation.";
