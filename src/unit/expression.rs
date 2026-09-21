use core::fmt;

use super::{LinearUnit, private::Sealed};

/// Conversion and formatting for a named or composed linear unit of `D`.
///
/// Named units inherit this contract from [`LinearUnit`]. Composed units
/// derive their dimension and scale without requiring a catalog entry.
/// Conversion rounds the coefficient to the selected scalar precision, then
/// applies native arithmetic. Coefficients outside that scalar's representable
/// range retain its IEEE overflow/underflow behavior.
pub trait Unit<D>: Sealed {
    /// Positive finite multiplier from this unit to canonical SI units.
    const SCALE: f64;

    /// Write the unit expression without allocating a string.
    ///
    /// # Errors
    /// Returns the error from the formatter's destination.
    fn fmt_symbol(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<D, U: LinearUnit<D>> Unit<D> for U {
    const SCALE: f64 = U::SCALE;

    fn fmt_symbol(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(U::SYMBOL)
    }
}

/// The dimension a unit contributes when used in a compound expression.
///
/// Multiplication, division and powers discard semantic markers exactly as
/// quantity arithmetic does. Temperature degrees contribute intervals, never
/// affine offsets. A named unit may still support additional semantic readings
/// through [`LinearUnit`].
pub trait UnitDimension: Sealed {
    /// Dimension used by dimensional algebra.
    type Dimension;
}

pub(super) const fn checked_scale(scale: f64) -> f64 {
    assert!(
        scale > 0.0 && scale.is_finite(),
        "unit scale must be positive and finite"
    );
    scale
}
