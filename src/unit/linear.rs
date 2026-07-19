use eunomia::FloatElement;

use super::private;

/// Sealed zero-sized marker contract for a linear unit of dimension `D`.
///
/// Linear conversion is `base = value × SCALE`. Implementations live in
/// Aequitas so the positive, finite scale invariant and symbol vocabulary have
/// one owner. Affine units require a distinct future contract.
pub trait LinearUnit<D>: private::Sealed {
    /// Unit abbreviation.
    const SYMBOL: &'static str;

    /// Multiplicative factor from this unit to the canonical SI base unit.
    ///
    /// Implementations must provide a finite positive value.
    const SCALE: f64;

    /// Convert a value from this unit to the canonical SI base unit.
    #[inline]
    #[must_use]
    fn to_base<T>(value: T) -> T
    where
        T: FloatElement,
    {
        value * T::from_f64(Self::SCALE)
    }

    /// Convert a value from the canonical SI base unit to this unit.
    #[inline]
    #[must_use]
    fn from_base<T>(value: T) -> T
    where
        T: FloatElement,
    {
        value / T::from_f64(Self::SCALE)
    }
}
