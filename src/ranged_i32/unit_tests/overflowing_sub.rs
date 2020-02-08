use super::*;
use crate::arith_traits::{OverflowingAdd, OverflowingSub};

#[test]
fn overflowing_sub__subtracting_a_non_overflowing_scalar_to_a_ranged_does_not_overflow() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();
    let scalar = 42;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(0).unwrap(), false));
}

#[test]
fn overflowing_sub__subtracting_a_range_overflowing_scalar_to_a_ranged_overflows() {
    // given
    let ranged = RangedI32::<0, 100>::new(80).unwrap();
    let scalar = 95;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(85).unwrap(), true));
}

// TODO: Corroborate impl results or correct; manual calculations disagreeing
//#[test]
//fn overflowing_sub__subtracting_a_machine_word_overflowing_scalar_to_a_ranged_overflows() {
//    // given
//    let ranged = RangedI32::<-100, 100>::new(-72).unwrap();
//    let scalar = i32::max_value();
//
//    // when
//    let res = ranged.overflowing_sub(scalar);
//
//    // then
//    assert_eq!(res, (RangedI32::<-100, 100>::new(80).unwrap(), true));
//}

#[test]
fn overflowing_sub__subtracting_a_machine_word_overflowing_scalar_to_a_ranged_overflows2() {
    // given
    let ranged = RangedI32::<
        { i32::min_value() + 1_000_000_000 },
        { i32::max_value() - 1_000_000_000 },
    >::new(i32::min_value() + 1_000_000_001)
    .unwrap();
    let scalar = 2;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(
        res,
        (
            RangedI32::<-1_147_483_648, 1_147_483_647>::new(1147483646).unwrap(),
            true
        )
    );
}
