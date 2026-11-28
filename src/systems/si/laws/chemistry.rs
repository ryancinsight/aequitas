//! Reaction-transport laws over typed quantities.
//!
//! Ideal-gas state, Fickian diffusion flux and Arrhenius kinetics. Scalar
//! transcendence runs through [`eunomia::FloatElement`].

use eunomia::FloatElement;

use super::super::constants::MOLAR_GAS_CONSTANT;
use super::super::quantities::{
    AmountOfSubstance, Dimensionless, Length, MassDiffusivity, MolarConcentration, MolarEnergy,
    MolarFlux, Pressure, ReciprocalTime, ThermodynamicTemperature, Volume,
};

impl<T: FloatElement> AmountOfSubstance<T> {
    /// Ideal-gas pressure `p = n·R·T / V`.
    ///
    /// Uses the exact SI molar gas constant; the amount, temperature and
    /// volume resolve dimensionally to a pressure with no raw scalar.
    #[inline]
    #[must_use]
    pub fn ideal_gas_pressure(
        self,
        temperature: ThermodynamicTemperature<T>,
        volume: Volume<T>,
    ) -> Pressure<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        self * gas * temperature / volume
    }
}

impl<T: FloatElement> MolarConcentration<T> {
    /// Fick first-law molar flux `J = −D · dc/dx`.
    ///
    /// Takes the concentration drop over a diffusion length; the sign
    /// convention (flux down the gradient) is carried by argument order —
    /// `upstream` minus `downstream` — so no negation trait bound is needed.
    #[inline]
    #[must_use]
    pub fn fick_flux(
        upstream: Self,
        downstream: Self,
        diffusivity: MassDiffusivity<T>,
        length: Length<T>,
    ) -> MolarFlux<T> {
        let drop = upstream.into_base() - downstream.into_base();
        let gradient = drop / length.into_base();
        MolarFlux::from_base(diffusivity.into_base() * gradient)
    }
}

impl<T: FloatElement> MolarEnergy<T> {
    /// Arrhenius rate constant `k = A · e^(−Ea/R·T)`.
    ///
    /// The activation energy carries its own dimension; the molar gas
    /// constant and the absolute temperature resolve the exponent
    /// dimensionlessly with no raw scalar.
    #[inline]
    #[must_use]
    pub fn arrhenius_rate(
        self,
        pre_exponential: ReciprocalTime<T>,
        temperature: ThermodynamicTemperature<T>,
    ) -> ReciprocalTime<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        let exponent = self.into_base() / (gas.into_base() * temperature.into_base());
        ReciprocalTime::from_base(pre_exponential.into_base() * (T::ZERO - exponent).exp())
    }
}

impl<T: FloatElement> ThermodynamicTemperature<T> {
    /// Van 't Hoff temperature coefficient `Q10 = k(T+10K) / k(T)`.
    ///
    /// Takes the absolute baseline temperature; resolves through the
    /// Arrhenius form with the molar gas constant, and the 10 K interval is
    /// exact, not a rounded scalar. Positive activation energy gives
    /// `Q10 > 1`: the rate at the warmer temperature is higher.
    #[inline]
    #[must_use]
    pub fn q10_factor(self, activation_energy: MolarEnergy<T>) -> Dimensionless<T> {
        let gas = MOLAR_GAS_CONSTANT.cast::<T>();
        let t = self.into_base();
        let dt = T::from_f64(10.0);
        let exponent =
            activation_energy.into_base() / gas.into_base() * (T::ONE / t - T::ONE / (t + dt));
        Dimensionless::from_base(exponent.exp())
    }
}
