//! Additive arithmetic for quantities whose dimension composes additively.
//!
//! `AdditiveDimension` admits linear quantities to addition and subtraction.
//! Base dimensions and surface tension have this contract. A
//! surface-tension difference retains force per length and its semantic marker.
//! The pair of dimensions that
//! addition relates across -- absolute temperature and a temperature
//! difference -- is the exception the kernel cannot express, and lives in
//! `affine`.

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    dimension::{BaseSemantics, Dimension},
    quantity::Quantity,
    systems::si::dimensions::SurfaceTension,
};

trait AdditiveDimension {}

impl<Length, Mass, Time, Current, Temperature, Amount, Luminosity> AdditiveDimension
    for Dimension<Length, Mass, Time, Current, Temperature, Amount, Luminosity, BaseSemantics>
{
}

/// Surface tension is linear under addition and subtraction of like values.
impl AdditiveDimension for SurfaceTension {}

impl<T, D> Add for Quantity<T, D>
where
    D: AdditiveDimension,
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
    D: AdditiveDimension,
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T, D> Sub for Quantity<T, D>
where
    D: AdditiveDimension,
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
    D: AdditiveDimension,
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}
