//! Dimension-preserving value-only operations on quantities.
//!
//! [`Quantity::abs`], [`Quantity::min`] and [`Quantity::max`] transform only
//! the scalar value through the [`FloatElement`] surface, leaving the
//! dimension type untouched — so the result is the *same* `Quantity<T, D>`
//! type as the operand, and a semantics-marked dimension such as `Angle`
//! survives unchanged (unlike [`Quantity::sqrt`][super::root],
//! [`Quantity::cbrt`][super::root] or [`Quantity::reciprocal`][super::reciprocal],
//! which normalize the marker away).
//!
//! Because the dimension never changes, these are plain scalar
//! transformations: no trait machinery, no dimension algebra, and a return
//! type identical to the operand's. This is the capability `uom` also
//! provides on its quantities; aequitas matches it here for parity while
//! relying on the `FloatElement` scalar surface for `f32`/`f64` (and the
//! eunomia wrappers), rather than on a hardcoded float primitive.

use eunomia::FloatElement;

use super::Quantity;

impl<T, D> Quantity<T, D>
where
    T: FloatElement,
{
    /// Absolute value of this quantity, preserving the dimension.
    ///
    /// Only the scalar magnitude changes; the dimension `D` is carried
    /// through unchanged, so the result has the same type as the operand:
    ///
    /// ```
    /// use aequitas::systems::si::quantities::Length;
    ///
    /// let depth = Length::from_base(-3.0);
    /// let magnitude: Length = depth.abs();
    /// assert!((*magnitude.as_base() - 3.0).abs() < f64::EPSILON);
    /// ```
    ///
    /// Because the dimension is preserved, a semantics-marked quantity like
    /// `Angle` keeps its marker, and the result can be assigned straight
    /// back to the operand's type:
    ///
    /// ```
    /// use aequitas::systems::si::quantities::Angle;
    ///
    /// let angle = Angle::from_base(-1.5);
    /// let magnitude: Angle = angle.abs();
    /// assert!((*magnitude.as_base() - 1.5).abs() < f64::EPSILON);
    /// ```
    ///
    /// The scalar absolute value follows `FloatElement::abs` semantics —
    /// `abs(-0.0)` is `+0.0` and `abs(NaN)` is `NaN`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Quantity<T, D> {
        Quantity::from_base(self.value.abs())
    }

    /// Minimum of this quantity and `other`, preserving the dimension.
    ///
    /// Both operands must share the dimension `D`; the result is the smaller
    /// scalar value with the same dimension:
    ///
    /// ```
    /// use aequitas::systems::si::quantities::Length;
    ///
    /// let short = Length::from_base(2.0);
    /// let long = Length::from_base(5.0);
    /// let min: Length = short.min(long);
    /// assert!((*min.as_base() - 2.0).abs() < f64::EPSILON);
    /// ```
    ///
    /// The scalar comparison follows `FloatElement::min` semantics — the IEEE
    /// 754 `min`, which propagates `NaN` when either operand is `NaN`.
    #[inline]
    #[must_use]
    pub fn min(self, other: Quantity<T, D>) -> Quantity<T, D> {
        Quantity::from_base(self.value.min(other.value))
    }

    /// Maximum of this quantity and `other`, preserving the dimension.
    ///
    /// Both operands must share the dimension `D`; the result is the larger
    /// scalar value with the same dimension:
    ///
    /// ```
    /// use aequitas::systems::si::quantities::Length;
    ///
    /// let short = Length::from_base(2.0);
    /// let long = Length::from_base(5.0);
    /// let max: Length = short.max(long);
    /// assert!((*max.as_base() - 5.0).abs() < f64::EPSILON);
    /// ```
    ///
    /// The scalar comparison follows `FloatElement::max` semantics — the IEEE
    /// 754 `max`, which propagates `NaN` when either operand is `NaN`.
    #[inline]
    #[must_use]
    pub fn max(self, other: Quantity<T, D>) -> Quantity<T, D> {
        Quantity::from_base(self.value.max(other.value))
    }
}
