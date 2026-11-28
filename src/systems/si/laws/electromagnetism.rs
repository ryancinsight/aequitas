//! Electromagnetic laws over typed quantities.
//!
//! Ohm, Lorentz force, capacitance, inductive/capacitive reactance and the
//! vacuum wave speed. Scalar transcendence runs through
//! [`eunomia::FloatElement`].

use eunomia::FloatElement;

use super::super::constants::{VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY};
use super::super::quantities::{
    Capacitance, ElectricCharge, ElectricCurrent, ElectricPotential, Force, Length,
    MagneticFluxDensity, MagneticPermeability, Permittivity, Velocity,
};

impl<T: FloatElement> ElectricPotential<T> {
    /// Ohm current `I = V / R`.
    #[inline]
    #[must_use]
    pub fn ohm_current(
        self,
        resistance: super::super::quantities::ElectricalImpedance<T>,
    ) -> ElectricCurrent<T> {
        self / resistance
    }

    /// Capacitance from charge and voltage: `C = Q / V`.
    #[inline]
    #[must_use]
    pub fn capacitance_for_charge(self, charge: ElectricCharge<T>) -> Capacitance<T> {
        charge / self
    }
}

impl<T: FloatElement> ElectricCharge<T> {
    /// Lorentz electric force `F = q · E`.
    #[inline]
    #[must_use]
    pub fn electric_force(
        self,
        field: super::super::quantities::ElectricFieldStrength<T>,
    ) -> Force<T> {
        self * field
    }
}

impl<T: FloatElement> MagneticFluxDensity<T> {
    /// Lorentz magnetic force magnitude `F = q · v · B` (perpendicular case).
    ///
    /// The general `q·v×B` needs the angle; the perpendicular magnitude is
    /// the common accelerator/spectrometer form.
    #[inline]
    #[must_use]
    pub fn magnetic_force(self, charge: ElectricCharge<T>, velocity: Velocity<T>) -> Force<T> {
        self * charge * velocity
    }
}

impl<T: FloatElement> Permittivity<T> {
    /// Vacuum wave speed `c = 1 / sqrt(mu₀ · eps₀)`.
    ///
    /// Takes the permeability as the companion constant; both resolve
    /// through the type-level root to a velocity with no raw scalar.
    #[inline]
    #[must_use]
    pub fn vacuum_wave_speed(self, permeability: MagneticPermeability<T>) -> Velocity<T> {
        let product = self.into_base() * permeability.into_base();
        Velocity::from_base(T::ONE / product.powf(T::from_f64(0.5)))
    }
}

impl<T: FloatElement> MagneticPermeability<T> {
    /// Relative wave speed `v = c₀ / sqrt(eps_r · mu_r)`.
    ///
    /// Scales the exact vacuum speed by the relative material constants.
    #[inline]
    #[must_use]
    pub fn medium_wave_speed(
        relative_permittivity: super::super::quantities::Dimensionless<T>,
        relative_permeability: super::super::quantities::Dimensionless<T>,
    ) -> Velocity<T> {
        let vacuum = VACUUM_PERMITTIVITY
            .cast::<T>()
            .vacuum_wave_speed(VACUUM_PERMEABILITY.cast::<T>());
        let index = (relative_permittivity.into_base() * relative_permeability.into_base())
            .powf(T::from_f64(0.5));
        Velocity::from_base(vacuum.into_base() / index)
    }
}

impl<T: FloatElement> Length<T> {
    /// Free-space wavelength `lambda = c / f`.
    #[inline]
    #[must_use]
    pub fn wavelength_for_frequency(
        speed: Velocity<T>,
        frequency: super::super::quantities::Frequency<T>,
    ) -> Self {
        speed / frequency
    }
}
