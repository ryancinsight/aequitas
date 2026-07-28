use eunomia::UnitScalar;

use crate::unit::LinearUnit;

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
