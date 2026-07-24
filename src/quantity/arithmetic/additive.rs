use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    dimension::{BaseSemantics, Dimension},
    quantity::Quantity,
    systems::si::dimensions,
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
