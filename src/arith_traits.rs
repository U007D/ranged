pub trait OverflowingAdd<Rhs = Self>
where
    Self: Sized,
{
    type Output;

    #[must_use]
    fn overflowing_add(self, rhs: Rhs) -> Self::Output;
}

pub trait OverflowingSub<Rhs = Self>
where
    Self: Sized,
{
    type Output;

    #[must_use]
    fn overflowing_sub(self, rhs: Rhs) -> Self::Output;
}
