use super::{assert, *};

#[test]

fn add__adding_two_ranged_i32s_without_overflowing_succeeds() {
    // given
    let r1 = RangedI32::<0, 100>::new(50).unwrap();

    let r2 = RangedI32::<-1000, 2000>::new(1999).unwrap();

    // when
    let res = r1 + r2;

    // then
    assert!(res == RangedI32::<-1000, 2100>::new(2049).unwrap());
}

#[test]

fn add__adding_a_scalar_to_a_ranged_without_overflowing_succeeds() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();

    let scalar = 42;

    // when
    let res = ranged + scalar;

    // then
    assert!(res == RangedI32::<0, 100>::new(84).unwrap());
}

// This test demonstrates that overflowing the addition of two ranges causes a compile  error.
// You may uncomment this test to demonstrate the behavior, but as it inhibits compilation, it
// should be left commented out.

// #[test]
// fn add_two_valid_ranges_yielding_a_overflowing_end_range_fails_to_compile() {
//    // given
//    let r1 = RangedI32::<0, { i32::max_value() }>::new(50).unwrap();
//    let r2 = RangedI32::<-1000, 2000>::new(1999).unwrap();

//    // when
//    let res = r1 + r2;

//    // then
//    // compile fails--never gets here
// }

// This test demonstrates that overflowing the addition of two ranges causes a compile  error.
// You may uncomment this test to demonstrate the behavior, but as it inhibits compilation, it
// should be left commented out.
//#[test]
// fn add_two_valid_ranges_yielding_a_overflowing_start_range_fails_to_compile() {
//    // given
//    let r1 = RangedI32::<{i32::min_value()}, 100>::new(50).unwrap();
//    let r2 = RangedI32::<-1000, 2000>::new(1999).unwrap();
//
//    // when
//    let res = r1 + r2;
//
//    // then
//    // compile fails--never gets here
//}
