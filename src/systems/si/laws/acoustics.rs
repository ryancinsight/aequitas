//! Acoustic wave laws over typed quantities.
//!
//! Newton–Laplace sound speed, characteristic impedance, the
//! intensity–pressure relation and the particle-velocity bridge. Scalar
//! transcendence runs through [`eunomia::FloatElement`].
//!
//! The bulk modulus enters as a [`Pressure`](super::super::quantities::Pressure):
//! `K = −V·dp/dV` carries pressure units, and engineers quote it in Pa/GPa.
//! A dedicated modulus semantic belongs with a Proteus-driven material
//! need; the dimension here is exact.

use eunomia::FloatElement;

use super::super::quantities::{AcousticImpedance, Intensity, MassDensity, Pressure, Velocity};

impl<T: FloatElement> MassDensity<T> {
    /// Newton–Laplace sound speed `c = sqrt(K / rho)`.
    ///
    /// The `K / rho` quotient resolves through the type-level root to a
    /// velocity with no raw scalar.
    #[inline]
    #[must_use]
    pub fn sound_speed(self, bulk_modulus: Pressure<T>) -> Velocity<T> {
        (bulk_modulus / self).sqrt()
    }

    /// Characteristic acoustic impedance `Z = rho · c`.
    #[inline]
    #[must_use]
    pub fn characteristic_impedance(self, sound_speed: Velocity<T>) -> AcousticImpedance<T> {
        self * sound_speed
    }
}

impl<T: FloatElement> Intensity<T> {
    /// RMS pressure amplitude from intensity `p = sqrt(I · Z)`.
    ///
    /// Plane progressive wave; `p` is the root-mean-square pressure.
    #[inline]
    #[must_use]
    pub fn rms_pressure(self, impedance: AcousticImpedance<T>) -> Pressure<T> {
        (self * impedance).sqrt()
    }
}

impl<T: FloatElement> Pressure<T> {
    /// Time-averaged intensity from RMS pressure `I = p² / Z`.
    ///
    /// Plane progressive wave; `self` is the root-mean-square pressure.
    #[inline]
    #[must_use]
    pub fn rms_intensity(self, impedance: AcousticImpedance<T>) -> Intensity<T> {
        let p = self.into_base();
        Intensity::from_base(p * p / impedance.into_base())
    }

    /// Particle velocity amplitude `u = p / Z`.
    ///
    /// Plane progressive wave; `self` is the pressure amplitude and the
    /// result its in-phase particle-velocity amplitude.
    #[inline]
    #[must_use]
    pub fn particle_velocity(self, impedance: AcousticImpedance<T>) -> Velocity<T> {
        self / impedance
    }
}
