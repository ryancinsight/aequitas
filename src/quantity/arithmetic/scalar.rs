use core::ops::{Div, Mul};

use eunomia::{Complex, FloatElement, RealField};

use crate::quantity::Quantity;

impl<T, D> Mul<T> for Quantity<T, D>
where
    T: FloatElement,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::from_base(self.value * rhs)
    }
}

impl<T, D> Div<T> for Quantity<T, D>
where
    T: FloatElement,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::from_base(self.value / rhs)
    }
}

impl<T, D> Mul<T> for Quantity<Complex<T>, D>
where
    T: RealField,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::from_base(self.value.scale(rhs))
    }
}

impl<T, D> Div<T> for Quantity<Complex<T>, D>
where
    T: RealField,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::from_base(self.value.scale(rhs.recip()))
    }
}
