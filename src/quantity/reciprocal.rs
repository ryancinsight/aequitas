//! Reciprocal (multiplicative inverse) operations on quantities.
//!
//! [`Quantity::reciprocal`] inverts the value and every SI dimension
//! exponent at the type level — `Length::reciprocal()` is a
//! `ReciprocalLength` and `Time::reciprocal()` is a `ReciprocalTime`.
//! This is the named complement of [`Quantity::powi::<N1>()`][super::pow],
//! giving a readable spelling for the most common negative power while
//! reusing the exact same [`PowDimension`] machinery.

use eunomia::FloatElement;
use typenum::N1;

use crate::dimension::PowDimension;

use super::Quantity;

impl<T, D> Quantity<T, D>
where
    T: FloatElement,
{
    /// Multiplicative inverse of this quantity, negating every dimension
    /// exponent.
    ///
    /// For example `Length::reciprocal()` yields a `ReciprocalLength` and
    /// `Time::reciprocal()` yields a `ReciprocalTime`. The scalar value is
    /// inverted at the native scalar precision.
    ///
    /// Semantics-marked dimensions normalize their marker away:
    /// `Angle::reciprocal()` is plain dimensionless, so assigning the result
    /// back to an `Angle` fails:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Angle;
    ///
    /// let angle = Angle::from_base(3.0);
    /// let inv: Angle = angle.reciprocal(); // yields dimensionless, not Angle
    /// ```
    #[inline]
    #[must_use]
    pub fn reciprocal(self) -> Quantity<T, <D as PowDimension<N1>>::Output>
    where
        D: PowDimension<N1>,
    {
        Quantity::from_base(self.value.recip())
    }
}
