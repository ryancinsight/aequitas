//! Thermal laws over typed quantities.
//!
//! Conduction, storage, perfusion and blackbody emission. Every law takes
//! quantities in and returns a quantity out; scalar transcendence runs
//! through [`eunomia::FloatElement`].

use eunomia::FloatElement;
use typenum::P4;

use super::super::constants::STEFAN_BOLTZMANN_CONSTANT;
use super::super::quantities::{
    Dimensionless, Intensity, Length, MassDensity, ReciprocalTime, SpecificHeatCapacity,
    TemperatureDifference, ThermalConductivity, ThermalDiffusivity, ThermodynamicTemperature,
    VolumetricPowerDensity,
};
use crate::dimension::PowDimension;

impl<T: FloatElement> ThermalConductivity<T> {
    /// Thermal diffusivity `alpha = k / (rho * c)`.
    ///
    /// The typed form of the `ThermalPropertyData::thermal_diffusivity`
    /// computation that consumers currently perform in raw `f64`.
    #[inline]
    #[must_use]
    pub fn thermal_diffusivity(
        self,
        density: MassDensity<T>,
        heat_capacity: SpecificHeatCapacity<T>,
    ) -> ThermalDiffusivity<T> {
        self / (density * heat_capacity)
    }

    /// Fourier heat-flux density `q = k * dT / dx`.
    ///
    /// The gradient is carried as a temperature difference per unit length;
    /// the product dimension-resolves to a heat-flux density with no
    /// intermediate raw scalar.
    #[inline]
    #[must_use]
    pub fn fourier_flux(
        self,
        temperature_drop: TemperatureDifference<T>,
        thickness: Length<T>,
    ) -> Intensity<T> {
        self * (temperature_drop / thickness)
    }
}

impl<T: FloatElement> MassDensity<T> {
    /// Pennes bioheat perfusion source `Q = rho * c * w * (T_a − T)`.
    ///
    /// Called on the blood density; takes blood specific heat, perfusion rate
    /// and arterial–tissue temperature difference. The typed form of the
    /// `blood_perfusion` term consumers currently carry as `Option<f64>`.
    #[inline]
    #[must_use]
    pub fn pennes_perfusion_source(
        self,
        heat_capacity: SpecificHeatCapacity<T>,
        perfusion_rate: ReciprocalTime<T>,
        arterial_minus_tissue: TemperatureDifference<T>,
    ) -> VolumetricPowerDensity<T> {
        let capacity = self * heat_capacity;
        let rate = capacity * perfusion_rate;
        rate * arterial_minus_tissue
    }
}

impl<T: FloatElement> ThermodynamicTemperature<T>
where
    super::super::dimensions::ThermodynamicTemperature: PowDimension<P4>,
{
    /// Stefan–Boltzmann blackbody exitance `M = sigma * T^4`.
    ///
    /// Uses the exact SI [`STEFAN_BOLTZMANN_CONSTANT`]; the `T^4` power runs
    /// through the type-level [`PowDimension`], so the product resolves to
    /// an [`Intensity`] with no raw scalar.
    #[inline]
    #[must_use]
    pub fn blackbody_exitance(self) -> Intensity<T> {
        let t4 = self.powi::<P4>();
        let sigma = STEFAN_BOLTZMANN_CONSTANT.cast::<T>();
        sigma * t4
    }
}

impl<T: FloatElement> TemperatureDifference<T> {
    /// Dimensionless temperature ratio `theta = dT / T_ref`.
    #[inline]
    #[must_use]
    pub fn over(self, reference: ThermodynamicTemperature<T>) -> Dimensionless<T> {
        self / reference
    }
}
