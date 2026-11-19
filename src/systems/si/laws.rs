//! Quantity identities that require more information than a common dimension.

use eunomia::UnitScalar;

use super::quantities::{AbsorbedDose, AngularVelocity, Dimensionless, DoseEquivalent, Frequency};

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

impl<T: core::ops::Mul<Output = T>> DoseEquivalent<T> {
    /// Dose equivalent `H = Q D` for a specified dimensionless quality factor
    /// `Q` and absorbed dose `D` (SI Brochure §2.3.5).
    #[inline]
    #[must_use]
    pub fn from_absorbed_dose(dose: AbsorbedDose<T>, quality_factor: Dimensionless<T>) -> Self {
        Self::from_base(dose.into_base() * quality_factor.into_base())
    }
}
