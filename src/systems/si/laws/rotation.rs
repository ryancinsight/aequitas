//! Rotation kinematics: the cycle/phase-rate bridge.
//!
//! One cycle advances the phase by exactly 2π radians (SI Brochure §2.3.4),
//! so [`AngularVelocity::from_frequency`] multiplies by τ and
//! [`AngularVelocity::frequency`] divides by it. The cycle frequency
//! (`Frequency`, Hz) and the phase rate (`AngularVelocity`, rad/s) are
//! distinct semantic dimensions; this bridge is the only typed conversion
//! between them.

use eunomia::UnitScalar;

use super::super::quantities::{AngularVelocity, Frequency};

impl<T: UnitScalar> AngularVelocity<T> {
    /// Angular frequency of a periodic phenomenon. One cycle advances its
    /// phase by exactly 2π radians (SI Brochure §2.3.4).
    ///
    /// ```compile_fail,E0277
    /// use aequitas::systems::si::{quantities::AngularFrequency, units::Hertz};
    /// let invalid = AngularFrequency::from_unit::<Hertz>(1.0_f64);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_frequency(frequency: Frequency<T>) -> Self {
        Self::from_base(
            frequency
                .into_base()
                .scale_by_factor(core::f64::consts::TAU),
        )
    }

    /// Frequency in cycles per unit time for this phase rate.
    #[inline]
    #[must_use]
    pub fn frequency(self) -> Frequency<T> {
        Frequency::from_base(self.into_base().divide_by_factor(core::f64::consts::TAU))
    }
}
