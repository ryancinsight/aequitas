//! Additive arithmetic for quantities whose dimension composes additively.
//!
//! `BaseAdditiveDimension` is the private witness that admits a dimension to
//! addition at all. Every base dimension implements it, while a semantic
//! dimension such as `Angle` deliberately does not, so an angle can be
//! converted between units but not summed. The one pair of dimensions that
//! addition relates across -- absolute temperature and a temperature
//! difference -- is the exception the kernel cannot express, and lives in
//! `affine`.

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    dimension::{BaseSemantics, Dimension},
    quantity::Quantity,
};

trait BaseAdditiveDimension {}

impl<Length, Mass, Time, Current, Temperature, Amount, Luminosity> BaseAdditiveDimension
    for Dimension<Length, Mass, Time, Current, Temperature, Amount, Luminosity, BaseSemantics>
{
}

impl<T, D> Add for Quantity<T, D>
where
    D: BaseAdditiveDimension,
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
    D: BaseAdditiveDimension,
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T, D> Sub for Quantity<T, D>
where
    D: BaseAdditiveDimension,
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
    D: BaseAdditiveDimension,
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}
