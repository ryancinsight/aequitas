use core::fmt;
use eunomia::RealField;
use num_bigint::BigInt;
use num_rational::BigRational;

/// A reduced, exact rational exponent of a physical dimension.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Exponent(pub(super) BigRational);

impl Exponent {
    /// Form an exact rational. A zero denominator has no value.
    #[must_use]
    pub fn ratio(numerator: i64, denominator: i64) -> Option<Self> {
        (denominator != 0).then(|| Self(BigRational::new(numerator.into(), denominator.into())))
    }

    /// Reflect the exact binary value of a finite scalar as a rational.
    ///
    /// Binary normalization and subtraction of the leading bit are exact.
    /// Each iteration consumes one significand bit, including for subnormals.
    /// The integer power of two carries the scale outside scalar arithmetic.
    #[must_use]
    pub fn from_scalar<T: RealField>(value: T) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let Some(mut scale) = value.binary_exponent() else {
            return Some(Self::from(0));
        };
        let mut fraction = value.abs().scale_binary(-scale);
        let mut integer = BigInt::from(0);
        loop {
            let bit = fraction >= T::ONE;
            integer = (integer << 1) + u8::from(bit);
            if bit {
                fraction -= T::ONE;
            }
            if fraction == T::ZERO {
                break;
            }
            fraction += fraction;
            scale -= 1;
        }
        if value < T::ZERO {
            integer = -integer;
        }
        let shift = scale.unsigned_abs();
        Some(Self(if scale >= 0 {
            BigRational::from_integer(integer << shift)
        } else {
            BigRational::new(integer, BigInt::from(1) << shift)
        }))
    }
}

impl From<i64> for Exponent {
    fn from(value: i64) -> Self {
        Self(BigRational::from_integer(value.into()))
    }
}

impl fmt::Display for Exponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
