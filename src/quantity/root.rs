//! Rational-power operations on quantities.
//!
//! [`Quantity::sqrt`] and [`Quantity::cbrt`] apply the scalar square/cube root
//! through the [`FloatElement`] scalar surface (native `cbrt`; `sqrt` via
//! `powf(1/2)`) while carrying the correct half/third dimension at the type
//! level. This is the capability `uom` cannot
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
    ///
    /// Dimensions with an odd exponent have no square root by construction,
    /// so the call fails to compile:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Length;
    ///
    /// let side = Length::from_base(4.0);
    /// let root = side.sqrt(); // `Length` has exponent `P1` (not even)
    /// ```
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
    /// through `FloatElement::cbrt`, the sign-preserving cube root defined
    /// for all reals — `cbrt(-8 m³)` is `-2 m`, unlike `powf(x, 1/3)` which
    /// is NaN for negative operands.
    ///
    /// Dimensions with an exponent not divisible by three have no cube root
    /// by construction, so the call fails to compile:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Time;
    ///
    /// let t = Time::from_base(8.0);
    /// let root = t.cbrt(); // `Time` has exponent `P1` (not divisible by 3)
    /// ```
    #[inline]
    #[must_use]
    pub fn cbrt(self) -> Quantity<T, <D as CbrtDimension>::Output> {
        Quantity::from_base(self.value.cbrt())
    }
}
