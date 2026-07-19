use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::quantity::Quantity;

impl<T, D> Add for Quantity<T, D>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value + rhs.value)
    }
}

impl<T, D> AddAssign for Quantity<T, D>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T, D> Sub for Quantity<T, D>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value - rhs.value)
    }
}

impl<T, D> SubAssign for Quantity<T, D>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}
