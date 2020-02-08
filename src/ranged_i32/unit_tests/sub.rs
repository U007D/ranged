use super::*;

#[test]
fn sub__subtracting_two_ranged_i32s_without_overflowing_succeeds() {
    // given
    let r1 = RangedI32::<-5, 5>::new(0).unwrap();
    let r2 = RangedI32::<-100, -50>::new(-75).unwrap();

    // when
    let res = r1 - r2;

    // then
    assert_eq!(res, RangedI32::<46, 104>::new(75).unwrap());
}

#[test]
fn sub__subtracting_a_scalar_from_a_ranged_without_overflowing_succeeds() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();
    let scalar = 42;

    // when
    let res = ranged - scalar;

    // then
    assert_eq!(res, RangedI32::<0, 100>::new(0).unwrap());
}

#[test]
fn sub__subtracting_a_scalar_from_a_ranged_overflowing_range_fails() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();
    let scalar = 43;

    // when
    let res = panic::catch_unwind(|| ranged - scalar);

    // then
    assert!(res.is_err());
}

#[test]
fn sub__subtracting_a_scalar_from_a_ranged_overflowing_machine_word_fails() {
    // given
    let ranged = RangedI32::<-100, 0>::new(-42).unwrap();
    let scalar = i32::max_value();

    // when
    let res = panic::catch_unwind(|| ranged - scalar);

    // then
    assert!(res.is_err());
}
