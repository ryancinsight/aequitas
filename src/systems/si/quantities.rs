//! Named SI quantity aliases.

use crate::quantity::Quantity;

use super::dimensions;

/// Dimensionless quantity.
pub type Dimensionless<T = f64> = Quantity<T, dimensions::Dimensionless>;
/// Plane or rotational angle quantity, stored in radians.
pub type Angle<T = f64> = Quantity<T, dimensions::Angle>;
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
/// Electric-charge quantity.
pub type ElectricCharge<T = f64> = Quantity<T, dimensions::ElectricCharge>;
/// Electric-potential quantity.
pub type ElectricPotential<T = f64> = Quantity<T, dimensions::ElectricPotential>;
/// Electric-conductance quantity.
pub type ElectricConductance<T = f64> = Quantity<T, dimensions::ElectricConductance>;
/// Electrical-impedance quantity, including complex impedance phasors.
pub type ElectricalImpedance<T = f64> = Quantity<T, dimensions::ElectricalImpedance>;
/// Capacitance quantity.
pub type Capacitance<T = f64> = Quantity<T, dimensions::Capacitance>;
/// Electric polarizability quantity, including complex phasors.
pub type Polarizability<T = f64> = Quantity<T, dimensions::Polarizability>;
/// Thermodynamic-temperature quantity.
pub type ThermodynamicTemperature<T = f64> = Quantity<T, dimensions::ThermodynamicTemperature>;
/// Temperature-difference quantity.
pub type TemperatureDifference<T = f64> = Quantity<T, dimensions::TemperatureDifference>;
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
/// Number-density quantity, expressed as entities per volume.
pub type NumberDensity<T = f64> = Quantity<T, dimensions::NumberDensity>;
/// Reciprocal-volume quantity for geometric field coefficients.
pub type ReciprocalVolume<T = f64> = Quantity<T, dimensions::ReciprocalVolume>;
/// Molar-concentration quantity, expressed as amount of substance per volume.
pub type MolarConcentration<T = f64> = Quantity<T, dimensions::MolarConcentration>;
/// Velocity quantity.
pub type Velocity<T = f64> = Quantity<T, dimensions::Velocity>;
/// Frequency quantity.
pub type Frequency<T = f64> = Quantity<T, dimensions::Frequency>;
/// Reciprocal-time quantity.
pub type ReciprocalTime<T = f64> = Quantity<T, dimensions::ReciprocalTime>;
/// Reciprocal-time-squared quantity, used by vorticity-squared metrics.
pub type ReciprocalTimeSquared<T = f64> = Quantity<T, dimensions::ReciprocalTimeSquared>;
/// Pressure quantity.
pub type Pressure<T = f64> = Quantity<T, dimensions::Pressure>;
/// Force quantity.
pub type Force<T = f64> = Quantity<T, dimensions::Force>;
/// Pressure-per-electric-current quantity.
pub type PressurePerElectricCurrent<T = f64> = Quantity<T, dimensions::PressurePerElectricCurrent>;
/// Pressure-per-electric-potential quantity.
pub type PressurePerElectricPotential<T = f64> =
    Quantity<T, dimensions::PressurePerElectricPotential>;
/// Electric-potential-per-pressure quantity.
pub type ElectricPotentialPerPressure<T = f64> =
    Quantity<T, dimensions::ElectricPotentialPerPressure>;
/// Length-per-electric-potential quantity.
pub type LengthPerElectricPotential<T = f64> = Quantity<T, dimensions::LengthPerElectricPotential>;
/// Surface electric-charge density quantity.
pub type SurfaceChargeDensity<T = f64> = Quantity<T, dimensions::SurfaceChargeDensity>;
/// Mechanical spring-stiffness quantity.
pub type SpringStiffness<T = f64> = Quantity<T, dimensions::SpringStiffness>;
/// Mechanical damping-coefficient quantity.
pub type DampingCoefficient<T = f64> = Quantity<T, dimensions::DampingCoefficient>;
/// Pressure-gradient quantity.
pub type PressureGradient<T = f64> = Quantity<T, dimensions::PressureGradient>;
/// Hydraulic-resistance quantity.
pub type HydraulicResistance<T = f64> = Quantity<T, dimensions::HydraulicResistance>;
/// Hydraulic-inertance quantity.
pub type HydraulicInertance<T = f64> = Quantity<T, dimensions::HydraulicInertance>;
/// Hydraulic-compliance quantity.
pub type Compliance<T = f64> = Quantity<T, dimensions::Compliance>;
/// Quadratic-hydraulic-resistance quantity.
pub type QuadraticHydraulicResistance<T = f64> =
    Quantity<T, dimensions::QuadraticHydraulicResistance>;
/// Hydraulic-conductance quantity.
pub type HydraulicConductance<T = f64> = Quantity<T, dimensions::HydraulicConductance>;
/// Energy quantity.
pub type Energy<T = f64> = Quantity<T, dimensions::Energy>;
/// Energy-per-area quantity.
pub type EnergyPerArea<T = f64> = Quantity<T, dimensions::EnergyPerArea>;
/// Surface or interfacial tension quantity.
pub type SurfaceTension<T = f64> = Quantity<T, dimensions::SurfaceTension>;
/// Absorbed-dose quantity.
pub type AbsorbedDose<T = f64> = Quantity<T, dimensions::AbsorbedDose>;
/// Absorbed-dose-rate quantity, equivalently power per mass.
pub type AbsorbedDoseRate<T = f64> = Quantity<T, dimensions::AbsorbedDoseRate>;
/// Specific-absorption-rate quantity, the radiofrequency-dosimetry name for
/// absorbed dose rate.
pub type SpecificAbsorptionRate<T = f64> = Quantity<T, dimensions::SpecificAbsorptionRate>;
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
/// Area-per-time quantity, including planar flow rate per unit width.
pub type AreaPerTime<T = f64> = Quantity<T, dimensions::AreaPerTime>;
/// Thermal-diffusivity quantity.
pub type ThermalDiffusivity<T = f64> = Quantity<T, dimensions::ThermalDiffusivity>;
/// Mass-density quantity.
pub type MassDensity<T = f64> = Quantity<T, dimensions::MassDensity>;
/// Mass-density-rate quantity.
pub type MassDensityRate<T = f64> = Quantity<T, dimensions::MassDensityRate>;
/// Dynamic-viscosity quantity.
pub type DynamicViscosity<T = f64> = Quantity<T, dimensions::DynamicViscosity>;
/// Kinematic-viscosity quantity.
pub type KinematicViscosity<T = f64> = Quantity<T, dimensions::KinematicViscosity>;
/// Volumetric-flow-rate quantity.
pub type VolumetricFlowRate<T = f64> = Quantity<T, dimensions::VolumetricFlowRate>;
/// Acoustic-impedance quantity.
pub type AcousticImpedance<T = f64> = Quantity<T, dimensions::AcousticImpedance>;
/// Intensity quantity, expressed as power per area.
pub type Intensity<T = f64> = Quantity<T, dimensions::Intensity>;
/// Volumetric-power-density quantity.
pub type VolumetricPowerDensity<T = f64> = Quantity<T, dimensions::VolumetricPowerDensity>;
/// Energy-per-volume quantity.
pub type EnergyPerVolume<T = f64> = Quantity<T, dimensions::EnergyPerVolume>;
