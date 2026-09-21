//! Affine arithmetic: the one place where addition and subtraction relate two
//! dimensions instead of preserving one.
//!
//! Absolute temperature and a temperature difference share a unit but are
//! distinct dimensions, so these operations are the only ones the
//! dimension-generic kernel in `additive` cannot express:
//!
//! * absolute + difference = absolute
//! * difference + absolute = absolute
//! * difference + difference = difference
//! * absolute - absolute = difference
//!
//! Each value is read through `into_base`, which for these units is a plain
//! value conversion rather than a scale, so the laws stay allocation-free.

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{quantity::Quantity, systems::si::dimensions};

impl<T> Add<Quantity<T, dimensions::TemperatureDifference>>
    for Quantity<T, dimensions::ThermodynamicTemperature>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Quantity<T, dimensions::TemperatureDifference>) -> Self::Output {
        Self::from_base(self.value + rhs.into_base())
    }
}

impl<T> Add<Quantity<T, dimensions::ThermodynamicTemperature>>
    for Quantity<T, dimensions::TemperatureDifference>
where
    T: Add<Output = T>,
{
    type Output = Quantity<T, dimensions::ThermodynamicTemperature>;

    #[inline]
    fn add(self, rhs: Quantity<T, dimensions::ThermodynamicTemperature>) -> Self::Output {
        Quantity::from_base(self.value + rhs.into_base())
    }
}

impl<T> Add<Quantity<T, dimensions::TemperatureDifference>>
    for Quantity<T, dimensions::TemperatureDifference>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Quantity<T, dimensions::TemperatureDifference>) -> Self::Output {
        Self::from_base(self.value + rhs.into_base())
    }
}

impl<T> AddAssign<Quantity<T, dimensions::TemperatureDifference>>
    for Quantity<T, dimensions::ThermodynamicTemperature>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Quantity<T, dimensions::TemperatureDifference>) {
        self.value += rhs.into_base();
    }
}

impl<T> AddAssign for Quantity<T, dimensions::TemperatureDifference>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T> Sub for Quantity<T, dimensions::ThermodynamicTemperature>
where
    T: Sub<Output = T>,
{
    type Output = Quantity<T, dimensions::TemperatureDifference>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::from_base(self.value - rhs.value)
    }
}

impl<T> Sub<Quantity<T, dimensions::TemperatureDifference>>
    for Quantity<T, dimensions::ThermodynamicTemperature>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Quantity<T, dimensions::TemperatureDifference>) -> Self::Output {
        Self::from_base(self.value - rhs.into_base())
    }
}

impl<T> Sub for Quantity<T, dimensions::TemperatureDifference>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value - rhs.value)
    }
}

impl<T> SubAssign for Quantity<T, dimensions::TemperatureDifference>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}
