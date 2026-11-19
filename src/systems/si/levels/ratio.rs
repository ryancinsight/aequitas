use core::marker::PhantomData;

use eunomia::{FloatElement, RealField, UnitScalar};

use crate::Quantity;

use super::{AmplitudeRatio, LogarithmicUnit, PowerRatio};

/// Logarithmic amplitude ratio, conventionally measured in nepers.
pub type AmplitudeLevel<T = f64> = LogRatio<T, AmplitudeRatio>;

/// Logarithmic power ratio, conventionally measured in bels or decibels.
pub type PowerLevel<T = f64> = LogRatio<T, PowerRatio>;

/// Logarithmic level stored as the natural logarithm of a ratio.
///
/// The kind parameter distinguishes amplitude and power references.
///
/// ```compile_fail
/// use aequitas::systems::si::levels::{AmplitudeLevel, Decibel, Neper};
/// let amplitude = AmplitudeLevel::from_unit::<Neper>(1.0_f64);
/// let invalid = amplitude.in_unit::<Decibel>();
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogRatio<T, Kind> {
    log: T,
    kind: PhantomData<Kind>,
}

impl<T, Kind> LogRatio<T, Kind> {
    #[inline]
    const fn from_natural_log(log: T) -> Self {
        Self {
            log,
            kind: PhantomData,
        }
    }
}

impl<T: UnitScalar + RealField, Kind> LogRatio<T, Kind> {
    /// Construct a level from a reading in unit `U` of this ratio kind.
    #[inline]
    #[must_use]
    pub fn from_unit<U: LogarithmicUnit<Kind = Kind>>(value: T) -> Self {
        Self::from_natural_log(value.scale_by_factor(U::FACTOR))
    }

    /// Return the reading in unit `U` of this ratio kind.
    #[inline]
    #[must_use]
    pub fn in_unit<U: LogarithmicUnit<Kind = Kind>>(&self) -> T {
        self.log.divide_by_factor(U::FACTOR)
    }

    /// Compute the level of a positive measurement relative to a reference
    /// of the same dimension. The caller selects the kind of ratio and a
    /// reference appropriate to the measurement being reported.
    ///
    /// Returns `None` if either value is nonpositive or nonfinite. Logging
    /// the two values separately handles any overflow of the intermediate
    /// ratio. Around unity, `ln_1p` preserves small nonzero level changes.
    #[inline]
    #[must_use]
    pub fn from_ratio<D>(value: Quantity<T, D>, reference: Quantity<T, D>) -> Option<Self> {
        let value = value.into_base();
        let reference = reference.into_base();
        if !(value.is_finite() && reference.is_finite() && value > T::ZERO && reference > T::ZERO) {
            return None;
        }

        let log = if value >= reference {
            positive_log_ratio(value, reference)
        } else {
            -positive_log_ratio(reference, value)
        };
        Some(Self::from_natural_log(log))
    }

    /// Recover the underlying ratio with the scalar's native exponential.
    /// The result can overflow or underflow when the ratio is outside the
    /// scalar's representable range.
    #[inline]
    #[must_use]
    pub fn ratio(&self) -> T {
        FloatElement::exp(self.log)
    }
}

impl<T: UnitScalar + RealField> AmplitudeLevel<T> {
    /// Convert to a power level when signal and reference powers are
    /// proportional to squared amplitudes with the same coefficient.
    ///
    /// Since `ln((A/A₀)²) = 2 ln(A/A₀)`, no intermediate square is required.
    #[inline]
    #[must_use]
    pub fn square_law_power(self) -> PowerLevel<T> {
        LogRatio::from_natural_log(self.log.scale_by_factor(2.0))
    }
}

impl<T: UnitScalar + RealField> PowerLevel<T> {
    /// Recover an amplitude level under the same square-law assumption as
    /// [`AmplitudeLevel::square_law_power`].
    #[inline]
    #[must_use]
    pub fn square_law_amplitude(self) -> AmplitudeLevel<T> {
        LogRatio::from_natural_log(self.log.scale_by_factor(0.5))
    }
}

#[inline]
fn positive_log_ratio<T: RealField>(larger: T, smaller: T) -> T {
    let difference = larger - smaller;
    if difference <= smaller {
        // Sterbenz's lemma makes subtraction exact within a factor of two.
        FloatElement::ln_1p(difference / smaller)
    } else {
        let quotient = larger / smaller;
        if quotient.is_finite() {
            FloatElement::ln(quotient)
        } else {
            FloatElement::ln(larger) - FloatElement::ln(smaller)
        }
    }
}
