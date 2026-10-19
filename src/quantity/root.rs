//! Rational-power operations on quantities.
//!
//! [`Quantity::sqrt`] and [`Quantity::cbrt`] apply the scalar square/cube root
//! through the [`FloatElement`] scalar surface (native `cbrt` and `sqrt`)
//! while carrying the correct half/third dimension at the type
//! level. Output exponents must remain integers, matching the exact-root
//! requirement also implemented by `uom` 0.38.

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
    /// yields a speed. The scalar root uses `NumericElement::sqrt` at the
    /// native scalar precision, preserving IEEE signed-zero and NaN behavior.
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
    ///
    /// The rule holds for every exponent shape, not only the shipped ones: a
    /// fifth power has an odd exponent, so it has no square root either:
    ///
    /// ```compile_fail,E0599
    /// use aequitas::dimension::{BaseSemantics, Dimension};
    /// use aequitas::quantity::Quantity;
    /// use typenum::{P5, Z0};
    ///
    /// let fifth = Quantity::<f64, Dimension<P5, Z0, Z0, Z0, Z0, Z0, Z0, BaseSemantics>>::from_base(32.0);
    /// let _root = fifth.sqrt(); // exponent `P5` is odd
    /// ```
    ///
    /// Semantics-marked dimensions normalize their marker away: `Angle::sqrt`
    /// is dimensionless, so assigning the result back to an `Angle` fails:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Angle;
    ///
    /// let angle = Angle::from_base(9.0);
    /// let still_angle: Angle = angle.sqrt(); // yields dimensionless, not Angle
    /// ```
    ///
    /// The normalization is irreversible: squaring a rooted `Angle` still
    /// yields dimensionless, so the `Angle` marker never re-forms:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Angle;
    ///
    /// let angle = Angle::from_base(9.0);
    /// let rooted = angle.sqrt();            // dimensionless
    /// let squared: Angle = rooted * rooted; // still dimensionless, not Angle
    /// ```
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Quantity<T, <D as SqrtDimension>::Output> {
        Quantity::from_base(self.value.sqrt())
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
    ///
    /// An even exponent does not make a cube root exist: each exponent must
    /// be divisible by three, and the square-rootable length to the eighth
    /// power is not:
    ///
    /// ```compile_fail,E0599
    /// use aequitas::dimension::{BaseSemantics, Dimension};
    /// use aequitas::quantity::Quantity;
    /// use typenum::{P8, Z0};
    ///
    /// let eighth = Quantity::<f64, Dimension<P8, Z0, Z0, Z0, Z0, Z0, Z0, BaseSemantics>>::from_base(256.0);
    /// let _root = eighth.cbrt(); // exponent `P8` is not divisible by three
    /// ```
    ///
    /// Semantics-marked dimensions normalize their marker away:
    /// `ReciprocalVolume::cbrt` is a reciprocal length, so assigning the
    /// result back to a `ReciprocalVolume` fails:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::ReciprocalVolume;
    ///
    /// let rv = ReciprocalVolume::from_base(27.0);
    /// let still_rv: ReciprocalVolume = rv.cbrt(); // yields reciprocal length
    /// ```
    ///
    /// The normalization is irreversible: cubing a rooted `ReciprocalVolume`
    /// still yields a plain reciprocal volume, so the marker never re-forms:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::ReciprocalVolume;
    ///
    /// let rv = ReciprocalVolume::from_base(27.0);
    /// let rooted = rv.cbrt();            // reciprocal length
    /// let cubed: ReciprocalVolume = rooted * rooted * rooted; // plain, not marked
    /// ```
    #[inline]
    #[must_use]
    pub fn cbrt(self) -> Quantity<T, <D as CbrtDimension>::Output> {
        Quantity::from_base(self.value.cbrt())
    }
}
