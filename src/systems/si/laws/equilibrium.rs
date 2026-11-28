//! Chemical-equilibrium laws over typed quantities.
//!
//! Equilibrium constants from Gibbs energy, van 't Hoff temperature
//! dependence and reaction quotients from activities. Scalar transcendence
//! runs through [`eunomia::FloatElement`].
//!
//! Boundary note: these are the *thermodynamic identities* —
//! `K = exp(−ΔG/RT)` and its temperature dependence. Reaction *networks*,
//! stoichiometric matrices and rate assembly belong to prometheus; the
//! temperature dependence of rate constants belongs to proteus. Nothing
//! here names a species or a mechanism.

use eunomia::FloatElement;

use super::super::constants::MOLAR_GAS_CONSTANT;
use super::super::quantities::{Dimensionless, MolarEnergy, ThermodynamicTemperature};

impl<T: FloatElement> MolarEnergy<T> {
    /// Equilibrium constant `K = exp(−ΔG°/RT)`.
    ///
    /// Takes the standard Gibbs reaction energy; resolves through the exact
    /// molar gas constant to a dimensionless constant with no raw scalar.
    #[inline]
    #[must_use]
    pub fn equilibrium_constant(
        self,
        temperature: ThermodynamicTemperature<T>,
    ) -> Dimensionless<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        let exponent = T::ZERO - self.into_base() / (gas.into_base() * temperature.into_base());
        Dimensionless::from_base(exponent.exp())
    }

    /// van 't Hoff extrapolation `K(T₂) = K(T₁) · exp[−ΔH°/R · (1/T₂ − 1/T₁)]`.
    ///
    /// Takes the standard reaction enthalpy (temperature-independent over
    /// the interval) and the known constant at `t1`; returns the constant
    /// at `t2`.
    #[inline]
    #[must_use]
    pub fn vant_hoff_extrapolate(
        self,
        known: Dimensionless<T>,
        t1: ThermodynamicTemperature<T>,
        t2: ThermodynamicTemperature<T>,
    ) -> Dimensionless<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        let exponent = T::ZERO
            - self.into_base() / gas.into_base()
                * (T::ONE / t2.into_base() - T::ONE / t1.into_base());
        Dimensionless::from_base(known.into_base() * exponent.exp())
    }
}

impl<T: FloatElement> Dimensionless<T> {
    /// Reaction quotient contribution `a^nu` for one species.
    ///
    /// Activity raised to its stoichiometric coefficient (positive for
    /// products, negative for reactants). Composes multiplicatively across
    /// species: `Q = Π q_i`.
    #[inline]
    #[must_use]
    pub fn stoichiometric_power(self, coefficient: T) -> Self {
        Self::from_base(self.into_base().powf(coefficient))
    }

    /// Gibbs-energy driving force `ΔG = −RT · ln Q`.
    ///
    /// Takes the reaction quotient and temperature; the sign tells the
    /// spontaneous direction at the given composition.
    #[inline]
    #[must_use]
    pub fn gibbs_driving_force(self, temperature: ThermodynamicTemperature<T>) -> MolarEnergy<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        MolarEnergy::from_base(
            T::ZERO - gas.into_base() * temperature.into_base() * self.into_base().ln(),
        )
    }
}
