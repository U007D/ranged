#![allow(clippy::option_unwrap_used, clippy::option_result_used)]
use super::*;

#[test]
fn new__with_invalid_range_fails() {
    // when
    let res = std::panic::catch_unwind(|| Ranged::<5, 2>::from_i32(3));

    // then
    assert!(res.is_err());
}

#[test]
fn new__with_valid_range_succeeds() {
    // when
    let res = std::panic::catch_unwind(|| Ranged::<2, 5>::from_i32(3));

    // then
    assert!(res.is_ok());
}
