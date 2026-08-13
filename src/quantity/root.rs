//! Rational-power operations on quantities.
//!
//! [`Quantity::sqrt`] and [`Quantity::cbrt`] apply the scalar square/cube root
//! through the [`FloatElement`] power surface while carrying the correct
//! half/third dimension at the type level. This is the capability `uom` cannot
//! express generically (its dimensions are integer-exponent only and its
//! `sqrt` is hardcoded per quantity).

use eunomia::FloatElement;

use crate::dimension::{CbrtDimension, SqrtDimension};

use super::Quantity;

impl<T, D> Quantity<T, D>
where
    T: FloatElement,
    D: SqrtDimension,
{
    /// Square root of this quantity, halving every dimension exponent.
    ///
    /// For example `sqrt(area)` yields a length and `sqrt(speed_squared)`
    /// yields a speed. The scalar root uses the `FloatElement` power surface
    /// at the native scalar precision.
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Quantity<T, <D as SqrtDimension>::Output> {
        Quantity::from_base(self.value.powf(T::from_f64(0.5)))
    }
}

impl<T, D> Quantity<T, D>
where
    T: FloatElement,
    D: CbrtDimension,
{
    /// Cube root of this quantity, thirding every dimension exponent.
    ///
    /// For example `cbrt(volume)` yields a length. The scalar is rooted
    /// through the `FloatElement` power surface (`powf(1/3)`), so a negative
    /// value yields `NaN` — the same undefined-domain behavior `sqrt` has
    /// for negative inputs. A sign-preserving cube root is a follow-up if a
    /// consumer needs negative operands.
    #[inline]
    #[must_use]
    pub fn cbrt(self) -> Quantity<T, <D as CbrtDimension>::Output> {
        Quantity::from_base(self.value.powf(T::from_f64(1.0 / 3.0)))
    }
}
