//! Integer-power operations on quantities.
//!
//! [`Quantity::powi`] raises a quantity to a type-level integer power `P`,
//! scaling the value through the [`FloatElement`] scalar surface while raising
//! every SI dimension exponent at the type level. This is the capability
//! `uom` cannot express generically: its `powi` only scales the *value* at
//! runtime and hardcodes a per-quantity result dimension, so it cannot square
//! a `Length` into an `Area` or invert a `Time` into a `ReciprocalTime`
//! through the type system.

use eunomia::FloatElement;
use typenum::Integer;

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
    #[inline]
    #[must_use]
    pub fn powi<P>(self) -> Quantity<T, <D as PowDimension<P>>::Output>
    where
        D: PowDimension<P>,
        P: Integer,
    {
        Quantity::from_base(self.value.powi(P::to_i32()))
    }
}
