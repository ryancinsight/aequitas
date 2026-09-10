use eunomia::{RealField, UnitScalar};

use crate::unit::{AffineUnit, LinearUnit};

use super::Quantity;

impl<T, D> Quantity<T, D>
where
    T: UnitScalar,
{
    /// Construct from a value expressed in linear unit `U`.
    ///
    /// The unit coefficient is materialized directly as `T`; multiplication
    /// then executes in `T` without a widened intermediate.
    #[inline]
    #[must_use]
    pub fn from_unit<U>(value: T) -> Self
    where
        U: LinearUnit<D>,
    {
        Self::from_base(U::to_base(value))
    }

    /// Return the value expressed in linear unit `U`.
    ///
    /// IEEE NaN, infinity, and signed-zero behavior follows `T`.
    #[inline]
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<D>,
    {
        U::from_base(self.value)
    }
}

impl<T, D> Quantity<T, D>
where
    T: UnitScalar + RealField,
{
    /// Construct from a value expressed in affine unit `U`.
    ///
    /// Separate from [`from_unit`](Self::from_unit) because the conversions
    /// differ: an affine unit's zero is not the base unit's zero, so this one
    /// adds an offset. Keeping the names apart means a reader can see which
    /// arithmetic a call site performs, and the type system decides which is
    /// available — `DegreeCelsius` offers this for a temperature and
    /// `from_unit` for a temperature difference, because a difference has no
    /// offset.
    ///
    /// The scalar bound is narrower than `from_unit`'s for the same reason
    /// [`AffineUnit`] narrows it: there is no phasor whose zero is 273.15 K.
    ///
    /// IEEE NaN, infinity, and signed-zero behavior follows `T`.
    #[inline]
    #[must_use]
    pub fn from_affine_unit<U>(value: T) -> Self
    where
        U: AffineUnit<D>,
    {
        Self::from_base(U::to_base(value))
    }

    /// Return the value expressed in affine unit `U`.
    ///
    /// IEEE NaN, infinity, and signed-zero behavior follows `T`.
    #[inline]
    #[must_use]
    pub fn in_affine_unit<U>(&self) -> T
    where
        U: AffineUnit<D>,
    {
        U::from_base(self.value)
    }
}
