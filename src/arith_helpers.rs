#[inline]
#[allow(clippy::integer_arithmetic)]
pub const fn i32_to_u32(value: i32) -> u32 {
    const I32_SIGN_BIT: u32 = 0x8000_0000;

    match value >= 0 {
        #[allow(clippy::cast_sign_loss)]
        true => value as u32 | I32_SIGN_BIT,
        #[allow(clippy::cast_sign_loss)]
        false => (value - i32::min_value()) as u32,
    }
}

#[inline]
pub const fn min_offset(a: u32, b: u32) -> u32 {
    match a <= b {
        true => a,
        false => b,
    }
}

#[inline]
pub const fn max_offset(a: u32, b: u32) -> u32 {
    match a >= b {
        true => a,
        false => b,
    }
}

#[inline]
#[allow(clippy::integer_arithmetic)]
pub const fn signed_value_to_offset(base: i32, value: i32) -> u32 {
    let u_base = i32_to_u32(base);
    let u_value = i32_to_u32(value);
    max_offset(u_base, u_value) - min_offset(u_base, u_value)
}

#[inline]
#[allow(clippy::integer_arithmetic, clippy::checked_conversions)]
pub const fn wrapping_offset_to_value(offset: u32, base: i32) -> i32 {
    #[allow(clippy::cast_possible_wrap)]
    match offset <= (i32::max_value() as u32) {
        true => base.wrapping_add(offset as i32),
        false => base
            .wrapping_add(i32::max_value())
            .wrapping_add((offset - i32::max_value() as u32) as i32),
    }
}
