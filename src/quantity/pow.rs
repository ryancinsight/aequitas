//! Integer-power operations on quantities.
//!
//! [`Quantity::powi`] raises a quantity to a type-level integer power `P`,
//! scaling the value through the [`FloatElement`] scalar surface while raising
//! every SI dimension exponent at the type level, as `uom` 0.38 also does.
//! A length squared is an area and an inverted time is a reciprocal time.

use eunomia::FloatElement;
use typenum::{B1, Integer, IsGreaterOrEqual, IsLessOrEqual, N2147483648, P2147483647};

use crate::dimension::PowDimension;

use super::Quantity;

impl<T, D> Quantity<T, D>
where
    T: FloatElement,
{
    /// Raise this quantity to the type-level integer power `P`.
    ///
    /// For example `Length::powi::<P2>()` yields an `Area` and
    /// `Time::powi::<N1>()` yields a `ReciprocalTime`. The scalar value is
    /// raised via `FloatElement::powi` at the native scalar precision.
    ///
    /// Semantics-marked dimensions normalize their marker away:
    /// `Angle::powi::<P2>()` is a plain (marker-free) squared angle, so
    /// assigning the result back to an `Angle` fails:
    ///
    /// ```compile_fail
    /// use aequitas::systems::si::quantities::Angle;
    ///
    /// let angle = Angle::from_base(3.0);
    /// let squared: Angle = angle.powi::<typenum::P2>(); // yields squared angle, not Angle
    /// ```
    ///
    /// The exponent must fit the scalar operation's signed 32-bit input;
    /// larger values cannot silently truncate while the dimension stays large:
    ///
    /// ```compile_fail,E0277
    /// use aequitas::systems::si::quantities::Length;
    /// let _ = Length::from_base(2.0_f64).powi::<typenum::P4294967296>();
    /// ```
    ///
    /// ```compile_fail,E0277
    /// use aequitas::systems::si::quantities::Length;
    /// type TooNegative = typenum::NInt<typenum::Sum<typenum::U4294967296, typenum::U1>>;
    /// let _ = Length::from_base(2.0_f64).powi::<TooNegative>();
    /// ```
    #[inline]
    #[must_use]
    pub fn powi<P>(self) -> Quantity<T, <D as PowDimension<P>>::Output>
    where
        D: PowDimension<P>,
        P: Integer
            + IsGreaterOrEqual<N2147483648, Output = B1>
            + IsLessOrEqual<P2147483647, Output = B1>,
    {
        Quantity::from_base(self.value.powi(P::to_i32()))
    }
}
