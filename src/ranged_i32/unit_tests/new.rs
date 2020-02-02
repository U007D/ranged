use super::*;

#[test]
fn new__with_invalid_range_fails() {
    // when
    let res = std::panic::catch_unwind(|| RangedI32::<5, 2>::new(3));

    // then
    assert!(res.is_err());
}

#[test]
fn new__with_valid_range_succeeds() {
    // when
    let res = std::panic::catch_unwind(|| RangedI32::<2, 5>::new(3));

    // then
    assert!(res.is_ok());
}
