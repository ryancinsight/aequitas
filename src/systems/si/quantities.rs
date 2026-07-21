//! Named SI quantity aliases.

use crate::quantity::Quantity;

use super::dimensions;

/// Dimensionless quantity.
pub type Dimensionless<T = f64> = Quantity<T, dimensions::Dimensionless>;
/// Length quantity.
pub type Length<T = f64> = Quantity<T, dimensions::Length>;
/// Reciprocal-length quantity.
pub type ReciprocalLength<T = f64> = Quantity<T, dimensions::ReciprocalLength>;
/// Mass quantity.
pub type Mass<T = f64> = Quantity<T, dimensions::Mass>;
/// Time quantity.
pub type Time<T = f64> = Quantity<T, dimensions::Time>;
/// Electric-current quantity.
pub type ElectricCurrent<T = f64> = Quantity<T, dimensions::ElectricCurrent>;
/// Thermodynamic-temperature quantity.
pub type ThermodynamicTemperature<T = f64> = Quantity<T, dimensions::ThermodynamicTemperature>;
/// Reciprocal-temperature quantity.
pub type ReciprocalTemperature<T = f64> = Quantity<T, dimensions::ReciprocalTemperature>;
/// Reciprocal-squared-temperature quantity.
pub type ReciprocalTemperatureSquared<T = f64> =
    Quantity<T, dimensions::ReciprocalTemperatureSquared>;
/// Amount-of-substance quantity.
pub type AmountOfSubstance<T = f64> = Quantity<T, dimensions::AmountOfSubstance>;
/// Luminous-intensity quantity.
pub type LuminousIntensity<T = f64> = Quantity<T, dimensions::LuminousIntensity>;
/// Area quantity.
pub type Area<T = f64> = Quantity<T, dimensions::Area>;
/// Area-per-mass quantity.
pub type AreaPerMass<T = f64> = Quantity<T, dimensions::AreaPerMass>;
/// Volume quantity.
pub type Volume<T = f64> = Quantity<T, dimensions::Volume>;
/// Velocity quantity.
pub type Velocity<T = f64> = Quantity<T, dimensions::Velocity>;
/// Frequency quantity.
pub type Frequency<T = f64> = Quantity<T, dimensions::Frequency>;
/// Reciprocal-time quantity.
pub type ReciprocalTime<T = f64> = Quantity<T, dimensions::ReciprocalTime>;
/// Pressure quantity.
pub type Pressure<T = f64> = Quantity<T, dimensions::Pressure>;
/// Energy quantity.
pub type Energy<T = f64> = Quantity<T, dimensions::Energy>;
/// Energy-per-area quantity.
pub type EnergyPerArea<T = f64> = Quantity<T, dimensions::EnergyPerArea>;
/// Absorbed-dose quantity.
pub type AbsorbedDose<T = f64> = Quantity<T, dimensions::AbsorbedDose>;
/// Molar-energy quantity.
pub type MolarEnergy<T = f64> = Quantity<T, dimensions::MolarEnergy>;
/// Molar-heat-capacity quantity.
pub type MolarHeatCapacity<T = f64> = Quantity<T, dimensions::MolarHeatCapacity>;
/// Power quantity.
pub type Power<T = f64> = Quantity<T, dimensions::Power>;
/// Heat-capacity quantity.
pub type HeatCapacity<T = f64> = Quantity<T, dimensions::HeatCapacity>;
/// Specific-heat-capacity quantity.
pub type SpecificHeatCapacity<T = f64> = Quantity<T, dimensions::SpecificHeatCapacity>;
/// Thermal-conductivity quantity.
pub type ThermalConductivity<T = f64> = Quantity<T, dimensions::ThermalConductivity>;
/// Thermal-diffusivity quantity.
pub type ThermalDiffusivity<T = f64> = Quantity<T, dimensions::ThermalDiffusivity>;
/// Mass-density quantity.
pub type MassDensity<T = f64> = Quantity<T, dimensions::MassDensity>;
