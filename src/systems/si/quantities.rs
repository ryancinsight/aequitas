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
/// Electrical-conductivity quantity.
pub type ElectricalConductivity<T = f64> = Quantity<T, dimensions::ElectricalConductivity>;
/// Electrical-impedance quantity, including complex impedance phasors.
pub type ElectricalImpedance<T = f64> = Quantity<T, dimensions::ElectricalImpedance>;
/// Capacitance quantity.
pub type Capacitance<T = f64> = Quantity<T, dimensions::Capacitance>;
/// Electric polarizability quantity, including complex phasors.
pub type Polarizability<T = f64> = Quantity<T, dimensions::Polarizability>;
/// Magnetic flux density quantity, the `B` field of the Lorentz force law.
pub type MagneticFluxDensity<T = f64> = Quantity<T, dimensions::MagneticFluxDensity>;
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
/// Volumetric reaction rate.
pub type ReactionRate<T = f64> = Quantity<T, dimensions::ReactionRate>;
/// Molar flux across a surface.
pub type MolarFlux<T = f64> = Quantity<T, dimensions::MolarFlux>;
/// Velocity quantity.
pub type Velocity<T = f64> = Quantity<T, dimensions::Velocity>;
/// Temperature derivative of velocity, expressed in m/(s·K).
pub type VelocityPerTemperature<T = f64> = Quantity<T, dimensions::VelocityPerTemperature>;
/// Acceleration quantity.
pub type Acceleration<T = f64> = Quantity<T, dimensions::Acceleration>;
/// Frequency quantity.
pub type Frequency<T = f64> = Quantity<T, dimensions::Frequency>;
/// Reciprocal-time quantity.
pub type ReciprocalTime<T = f64> = Quantity<T, dimensions::ReciprocalTime>;
/// Reciprocal-time-squared quantity, used by vorticity-squared metrics.
pub type ReciprocalTimeSquared<T = f64> = Quantity<T, dimensions::ReciprocalTimeSquared>;
/// Pressure quantity.
pub type Pressure<T = f64> = Quantity<T, dimensions::Pressure>;
/// Mechanical stress, dimensionally pressure with distinct semantics.
pub type Stress<T = f64> = Quantity<T, dimensions::Stress>;
/// Pressure-rate quantity, expressed as pascals per second.
pub type PressureRate<T = f64> = Quantity<T, dimensions::PressureRate>;
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
/// Volume electric-charge density quantity.
pub type VolumeChargeDensity<T = f64> = Quantity<T, dimensions::VolumeChargeDensity>;
/// Mechanical spring-stiffness quantity.
pub type SpringStiffness<T = f64> = Quantity<T, dimensions::SpringStiffness>;
/// Mechanical damping-coefficient quantity.
pub type DampingCoefficient<T = f64> = Quantity<T, dimensions::DampingCoefficient>;
/// Mechanical impedance quantity, including complex force-per-velocity phasors.
pub type MechanicalImpedance<T = f64> = Quantity<T, dimensions::MechanicalImpedance>;
/// Flexural-rigidity quantity.
pub type FlexuralRigidity<T = f64> = Quantity<T, dimensions::FlexuralRigidity>;
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
/// Specific-energy quantity, expressed in joules per kilogram.
pub type SpecificEnergy<T = f64> = Quantity<T, dimensions::SpecificEnergy>;
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
/// Temperature derivative of mass density, expressed in kg/(m³·K).
pub type MassDensityPerTemperature<T = f64> = Quantity<T, dimensions::MassDensityPerTemperature>;
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
/// Spatial gradient of volumetric power density, expressed in W/m⁴.
pub type VolumetricPowerDensityGradient<T = f64> =
    Quantity<T, dimensions::VolumetricPowerDensityGradient>;
/// Energy-per-volume quantity.
pub type EnergyPerVolume<T = f64> = Quantity<T, dimensions::EnergyPerVolume>;
/// Temperature derivative of reciprocal length, expressed in 1/(m·K).
pub type ReciprocalLengthPerTemperature<T = f64> =
    Quantity<T, dimensions::ReciprocalLengthPerTemperature>;

/// Action, the product of energy and time.
pub type Action<T = f64> = Quantity<T, dimensions::Action>;
/// Reciprocal amount of substance.
pub type ReciprocalAmountOfSubstance<T = f64> =
    Quantity<T, dimensions::ReciprocalAmountOfSubstance>;
/// Electric permittivity, capacitance per length.
pub type Permittivity<T = f64> = Quantity<T, dimensions::Permittivity>;
/// Magnetic permeability, inductance per length.
pub type MagneticPermeability<T = f64> = Quantity<T, dimensions::MagneticPermeability>;

/// Activity of a radionuclide in decays per second.
///
/// ```compile_fail
/// use aequitas::systems::si::{quantities::Frequency, units::Becquerel};
/// let _ = Frequency::from_unit::<Becquerel>(800.0_f64);
/// ```
pub type Activity<T = f64> = Quantity<T, dimensions::Activity>;

/// Activity per volume.
pub type ActivityConcentration<T = f64> = Quantity<T, dimensions::ActivityConcentration>;

/// Radiation-weighted dose, dimensionally energy per mass.
///
/// ```compile_fail
/// use aequitas::systems::si::{quantities::AbsorbedDose, units::Sievert};
/// let _ = AbsorbedDose::from_unit::<Sievert>(1.0_f64);
/// ```
pub type DoseEquivalent<T = f64> = Quantity<T, dimensions::DoseEquivalent>;

/// Radiation-weighted dose per time.
pub type DoseEquivalentRate<T = f64> = Quantity<T, dimensions::DoseEquivalentRate>;

/// Ionizing radiation exposure in charge of one sign produced per mass of air.
pub type RadiationExposure<T = f64> = Quantity<T, dimensions::RadiationExposure>;

/// Solid angle, stored in steradians.
pub type SolidAngle<T = f64> = Quantity<T, dimensions::SolidAngle>;
/// Magnetic flux, electric potential times time.
pub type MagneticFlux<T = f64> = Quantity<T, dimensions::MagneticFlux>;
/// Inductance, magnetic flux per electric current.
pub type Inductance<T = f64> = Quantity<T, dimensions::Inductance>;
/// Luminous flux, luminous intensity integrated over solid angle.
pub type LuminousFlux<T = f64> = Quantity<T, dimensions::LuminousFlux>;
/// Incident luminous flux per surface area.
pub type Illuminance<T = f64> = Quantity<T, dimensions::Illuminance>;
/// Catalytic activity, amount transformed per unit time.
pub type CatalyticActivity<T = f64> = Quantity<T, dimensions::CatalyticActivity>;
/// Surface mass density, mass per area.
pub type SurfaceDensity<T = f64> = Quantity<T, dimensions::SurfaceDensity>;
/// Specific volume, volume per mass.
pub type SpecificVolume<T = f64> = Quantity<T, dimensions::SpecificVolume>;
/// Electric current per cross-sectional area.
pub type CurrentDensity<T = f64> = Quantity<T, dimensions::CurrentDensity>;
/// Magnetic field strength, current per length.
pub type MagneticFieldStrength<T = f64> = Quantity<T, dimensions::MagneticFieldStrength>;
/// Luminance, luminous intensity per projected area.
pub type Luminance<T = f64> = Quantity<T, dimensions::Luminance>;
/// Moment of force, expressed in newton metres.
pub type Torque<T = f64> = Quantity<T, dimensions::Torque>;
/// Angular velocity or angular frequency, expressed in radians per second.
pub type AngularVelocity<T = f64> = Quantity<T, dimensions::AngularVelocity>;
/// Angular acceleration, change of angular velocity per unit time.
pub type AngularAcceleration<T = f64> = Quantity<T, dimensions::AngularAcceleration>;
/// Electric potential difference per length.
pub type ElectricFieldStrength<T = f64> = Quantity<T, dimensions::ElectricFieldStrength>;
/// Radiant flux per solid angle.
pub type RadiantIntensity<T = f64> = Quantity<T, dimensions::RadiantIntensity>;
/// Radiant flux per projected area per solid angle.
pub type Radiance<T = f64> = Quantity<T, dimensions::Radiance>;
/// Catalytic activity per volume.
pub type CatalyticActivityConcentration<T = f64> =
    Quantity<T, dimensions::CatalyticActivityConcentration>;
/// Reactive power, expressed in var or volt amperes.
pub type ReactivePower<T = f64> = Quantity<T, dimensions::ReactivePower>;
/// Luminous flux per radiant power.
pub type LuminousEfficacy<T = f64> = Quantity<T, dimensions::LuminousEfficacy>;
/// Amount of information, stored in bits.
pub type Information<T = f64> = Quantity<T, dimensions::Information>;
/// Wavenumber, inverse wavelength.
pub type Wavenumber<T = f64> = Quantity<T, dimensions::ReciprocalLength>;
/// Mass of a constituent per volume of a mixture.
pub type MassConcentration<T = f64> = Quantity<T, dimensions::MassDensity>;
/// Angular frequency of a periodic phenomenon.
pub type AngularFrequency<T = f64> = Quantity<T, dimensions::AngularVelocity>;
/// Heat flow per unit area.
pub type HeatFluxDensity<T = f64> = Quantity<T, dimensions::Intensity>;
/// Incident radiant flux per unit area.
pub type Irradiance<T = f64> = Quantity<T, dimensions::Intensity>;
/// Thermodynamic entropy.
pub type Entropy<T = f64> = Quantity<T, dimensions::HeatCapacity>;
/// Entropy per mass.
pub type SpecificEntropy<T = f64> = Quantity<T, dimensions::SpecificHeatCapacity>;
/// Entropy per amount of substance.
pub type MolarEntropy<T = f64> = Quantity<T, dimensions::MolarHeatCapacity>;
/// Electric displacement, electric flux per area.
pub type ElectricDisplacement<T = f64> = Quantity<T, dimensions::SurfaceChargeDensity>;
