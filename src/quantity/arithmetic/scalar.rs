use core::ops::{Div, DivAssign, Mul, MulAssign};

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

impl<T, D> MulAssign<T> for Quantity<T, D>
where
    T: FloatElement + MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;
    }
}

impl<T, D> DivAssign<T> for Quantity<T, D>
where
    T: FloatElement + DivAssign,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
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

impl<T, D> MulAssign<T> for Quantity<Complex<T>, D>
where
    T: RealField,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.value = self.value.scale(rhs);
    }
}

impl<T, D> DivAssign<T> for Quantity<Complex<T>, D>
where
    T: RealField,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.value = self.value.scale(rhs.recip());
    }
}

// Commutative scalar multiplication (`scalar * quantity`) cannot be written as a
// blanket `impl<T, D> Mul<Quantity<T, D>> for T` because that violates the orphan
// rule (an uncovered type parameter precedes the first local type). Provide the
// impls for the concrete primitive scalars instead, mirroring `Quantity * scalar`.
macro_rules! impl_scalar_mul_quantity {
    ($($t:ty),+ $(,)?) => {$(
        impl<D> Mul<Quantity<$t, D>> for $t {
            type Output = Quantity<$t, D>;

            #[inline]
            fn mul(self, rhs: Quantity<$t, D>) -> Self::Output {
                Quantity::from_base(self * rhs.value)
            }
        }

        impl<D> Mul<Quantity<Complex<$t>, D>> for $t {
            type Output = Quantity<Complex<$t>, D>;

            #[inline]
            fn mul(self, rhs: Quantity<Complex<$t>, D>) -> Self::Output {
                Quantity::from_base(rhs.value.scale(self))
            }
        }
    )+};
}

impl_scalar_mul_quantity!(f32, f64);
