use super::*;
use arith_traits::Overflow;

#[test]
fn overflowing_sub__subtracting_a_non_overflowing_scalar_does_not_overflow() {
    // given
    let ranged = RangedI32::<0, 100>::new(42).unwrap();
    let scalar = 42;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(0).unwrap(), false));
}

#[test]
fn overflowing_sub__subtracting_a_range_overflowing_scalar_overflows() {
    // given
    let ranged = RangedI32::<0, 100>::new(80).unwrap();
    let scalar = 95;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<0, 100>::new(85).unwrap(), true));
}

#[test]
fn overflowing_sub__subtracting_a_near_signed_range_overflowing_scalar_does_not_overflow() {
    // given
    let ranged = RangedI32::<-100, 100>::new(-72).unwrap();
    let scalar = 27;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<-100, 100>::new(-99).unwrap(), false));
}

#[test]
fn overflowing_sub__subtracting_an_at_signed_range_overflowing_scalar_does_not_overflow() {
    // given
    let ranged = RangedI32::<-100, 100>::new(-72).unwrap();
    let scalar = 28;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<-100, 100>::new(-100).unwrap(), false));
}

#[test]
fn overflowing_sub__subtracting_a_minimal_signed_range_overflowing_scalar_overflows() {
    // given
    let ranged = RangedI32::<-100, 100>::new(-72).unwrap();
    let scalar = 29;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<-100, 100>::new(99).unwrap(), true));
}

#[test]
fn overflowing_sub__subtracting_a_machine_word_overflowing_scalar_overflows() {
    // given
    let ranged = RangedI32::<-100, 100>::new(-72).unwrap();
    let scalar = 47;

    // when
    let res = ranged.overflowing_sub(scalar);

    // then
    assert_eq!(res, (RangedI32::<-100, 100>::new(81).unwrap(), true));
}

// non-generalized reference impl: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=60b90923c45252ce3a32c9c089c4914c
#[test]
fn overflowing_sub__subtracting_a_machine_word_overflowing_scalar_from_a_mid_sized_range_overflows()
{
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
            RangedI32::<-1_147_483_648, 1_147_483_647>::new(1_147_483_646).unwrap(),
            true
        )
    );
}
