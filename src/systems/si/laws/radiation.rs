//! Radiation-transport laws over typed quantities.
//!
//! Decay, geometric attenuation, Beer–Lambert transmission, deposition and
//! the absorbed-dose bridge. Scalar transcendence runs through
//! [`eunomia::FloatElement`].

use eunomia::FloatElement;

use super::super::quantities::{
    AbsorbedDose, Activity, Dimensionless, DoseEquivalent, Intensity, Length, MassDensity,
    ReciprocalLength, SpecificAbsorptionRate, Time, VolumetricPowerDensity,
};

impl<T: FloatElement> Activity<T> {
    /// Radioactive decay `A(t) = A₀ · e^(−λt)`.
    #[inline]
    #[must_use]
    pub fn decayed(
        self,
        decay_constant: super::super::quantities::ReciprocalTime<T>,
        time: Time<T>,
    ) -> Self {
        let exponent = decay_constant.into_base() * time.into_base();
        Self::from_base(self.into_base() * (T::ZERO - exponent).exp())
    }
}

impl<T: FloatElement> Intensity<T> {
    /// Inverse-square falloff `I(r) = I₀ / r²`.
    ///
    /// Takes the distance from a point source; the `1/r²` runs through the
    /// type-level reciprocal-square power.
    #[inline]
    #[must_use]
    pub fn inverse_square(self, distance: Length<T>) -> Self
    where
        super::super::dimensions::Length: crate::dimension::PowDimension<typenum::N2>,
    {
        let inv_sq = distance.powi::<typenum::N2>();
        Self::from_base(self.into_base() * inv_sq.into_base())
    }

    /// Beer–Lambert transmission `I(x) = I₀ · e^(−μx)`.
    #[inline]
    #[must_use]
    pub fn beer_lambert(self, attenuation: ReciprocalLength<T>, path: Length<T>) -> Self {
        let depth = attenuation.into_base() * path.into_base();
        Self::from_base(self.into_base() * (T::ZERO - depth).exp())
    }
}

impl<T: FloatElement> ReciprocalLength<T> {
    /// Optical depth `tau = μ · x` (dimensionless).
    #[inline]
    #[must_use]
    pub fn optical_depth(self, path: Length<T>) -> Dimensionless<T> {
        self * path
    }
}

impl<T: FloatElement> VolumetricPowerDensity<T> {
    /// Specific absorption rate `SAR = P_vol / rho`.
    ///
    /// The `W/kg ≡ Gy/s` identity is already the shared
    /// `SpecificAbsorptionRate` dimension; this names the deposition step.
    #[inline]
    #[must_use]
    pub fn specific_absorption_rate(self, density: MassDensity<T>) -> SpecificAbsorptionRate<T> {
        self / density
    }

    /// Absorbed dose from a constant deposition rate: `D = P_vol · t / rho`.
    #[inline]
    #[must_use]
    pub fn absorbed_dose(self, density: MassDensity<T>, duration: Time<T>) -> AbsorbedDose<T> {
        self * duration / density
    }
}

impl<T: core::ops::Mul<Output = T>> DoseEquivalent<T> {
    /// Dose equivalent `H = Q · D` for a specified dimensionless quality
    /// factor `Q` and absorbed dose `D` (SI Brochure §2.3.5).
    #[inline]
    #[must_use]
    pub fn from_absorbed_dose(dose: AbsorbedDose<T>, quality_factor: Dimensionless<T>) -> Self {
        Self::from_base(dose.into_base() * quality_factor.into_base())
    }
}

impl<T: FloatElement> SpecificAbsorptionRate<T> {
    /// Absorbed dose from a constant SAR: `D = SAR · t`.
    #[inline]
    #[must_use]
    pub fn absorbed_dose(self, duration: Time<T>) -> AbsorbedDose<T> {
        self * duration
    }
}
