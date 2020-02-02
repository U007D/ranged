use super::*;
use crate::Result;

#[test]
fn overflowing_add__adding_a_non_overflowing_scalar_to_a_ranged_does_not_overflow() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();
    let scalar = 42;

    // when
    let res = ranged.overflowing_add(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(84).unwrap(), false));
}

#[test]
fn overflowing_add__adding_an_overflowing_scalar_to_a_ranged_overflows() {
    // given
    let ranged = RangedI32::<0, 100>::new(72).unwrap();
    let scalar = i32::max_value();

    // when
    let res = ranged.overflowing_add(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(19).unwrap(), true));
}
