use eunomia::{RealField, UnitScalar};

use super::private;

/// Sealed zero-sized marker contract for an affine unit of dimension `D`.
///
/// Affine conversion is `base = value × SCALE + OFFSET`, where `OFFSET` is the
/// base-unit reading when this unit reads zero. It is the contract
/// [`LinearUnit`](super::LinearUnit) deliberately excludes: a linear unit's
/// zero is the base unit's zero, and a degree Celsius's is not.
///
/// # Why the scalar bound is narrower
///
/// [`UnitScalar`] alone cannot express this, because it offers only scaling.
/// The missing operation is addition, which arrives through [`RealField`] —
/// and that bound is the physics rather than a convenience. `UnitScalar` is
/// implemented for complex scalars, where scaling is componentwise because the
/// imaginary part is quadrature, not a second physical unit. Translating such
/// a value by an offset has no meaning: there is no phasor whose zero is
/// 273.15 K. Bounding on [`RealField`] makes an affine conversion of a phasor
/// a compile error rather than a silently wrong number.
///
/// # Why a temperature difference is not affine
///
/// The offset belongs to the *point*, never to the *interval* between two
/// points. Ten degrees Celsius warmer is ten kelvin warmer; ten degrees
/// Celsius is 283.15 kelvin. Aequitas keeps these as separate dimensions, so
/// a unit expresses both facts by implementing this trait for
/// `ThermodynamicTemperature` and [`LinearUnit`](super::LinearUnit) for
/// `TemperatureDifference`. The dimension of the quantity then selects the
/// conversion, and no call site has to remember which one it wanted.
pub trait AffineUnit<D>: private::Sealed {
    /// Unit abbreviation.
    const SYMBOL: &'static str;

    /// Multiplicative factor from this unit's degree to the base unit's.
    ///
    /// Implementations must provide a finite positive value.
    const SCALE: f64;

    /// The base-unit value this unit reads as zero.
    ///
    /// Implementations must provide a finite value.
    const OFFSET: f64;

    /// Convert a value from this unit to the canonical SI base unit.
    #[inline]
    #[must_use]
    fn to_base<T>(value: T) -> T
    where
        T: UnitScalar + RealField,
    {
        value.scale_by_f64(Self::SCALE) + T::ONE.scale_by_f64(Self::OFFSET)
    }

    /// Convert a value from the canonical SI base unit to this unit.
    #[inline]
    #[must_use]
    fn from_base<T>(value: T) -> T
    where
        T: UnitScalar + RealField,
    {
        (value - T::ONE.scale_by_f64(Self::OFFSET)).scale_by_f64(1.0 / Self::SCALE)
    }
}
