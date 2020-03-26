use crate::RangedI32;

impl<const START: i32, const END: i32> RangedI32<START, END> {
    #[allow(clippy::integer_arithmetic)]
    #[must_use]
    pub fn overflowing_sub(self, rhs: i32) -> (Self, bool) {
        match rhs >= 0 {
            true => {
                let (value, overflow) = self.value.overflowing_sub(rhs);
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
                match overflow {
                    true => (
                        Self::new(
                            END - (((START - i32::min_value()) as u32
                                + (i32::max_value() - value) as u32)
                                % Self::RANGE_SPAN) as i32
                                - 1,
                        )
                        .unwrap_or_else(|| unreachable!()),
                        true,
                    ),
                    false => match value < START {
                        true => (
                            Self::new(END - ((START - value) as u32 % Self::RANGE_SPAN) as i32)
                                .unwrap_or_else(|| unreachable!()),
                            true,
                        ),
                        false => (Self::new(value).unwrap_or_else(|| unreachable!()), false),
                    },
                }
            }
            false => unimplemented!(),
        }
    }
}
