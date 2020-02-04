use super::*;

// Uncomment this test to demonstrate that an invalid range specification is a compile-time error
//#[test]
//fn new__with_invalid_range_fails_at_compile_time() {
//    // given
//    let sut = RangedI32::<7, 2>::new;
//
//    // when
//    let res = sut(4);
//
//    // then
//    assert!(res.is_none());
//}

#[test]
fn new__with_invalid_value_fails() {
    // given
    let sut = RangedI32::<-2, 2>::new;

    // when
    let res = sut(3);

    // then
    assert!(res.is_none());
}

#[test]
fn new__with_valid_range_succeeds() {
    // given
    let sut = RangedI32::<2, 5>::new;

    // when
    let res = sut(3);

    // then
    assert!(res.is_some());
}
