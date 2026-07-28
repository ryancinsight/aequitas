use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Square metre, the coherent SI unit of area.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeter;
impl Sealed for SquareMeter {}
impl LinearUnit<dimensions::Area> for SquareMeter {
    const SYMBOL: &'static str = "m²";
    const SCALE: f64 = 1.0;
}

/// Per metre, the coherent SI unit of reciprocal length.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerMeter;
impl Sealed for PerMeter {}
impl LinearUnit<dimensions::ReciprocalLength> for PerMeter {
    const SYMBOL: &'static str = "m⁻¹";
    const SCALE: f64 = 1.0;
}

/// Square metre per kilogram, the coherent SI unit of area per mass.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeterPerKilogram;
impl Sealed for SquareMeterPerKilogram {}
impl LinearUnit<dimensions::AreaPerMass> for SquareMeterPerKilogram {
    const SYMBOL: &'static str = "m²/kg";
    const SCALE: f64 = 1.0;
}

/// Cubic metre, the coherent SI unit of volume.
#[derive(Clone, Copy, Debug, Default)]
pub struct CubicMeter;
impl Sealed for CubicMeter {}
impl LinearUnit<dimensions::Volume> for CubicMeter {
    const SYMBOL: &'static str = "m³";
    const SCALE: f64 = 1.0;
}

/// Per cubic metre, the coherent SI unit of number density.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerCubicMeter;
impl Sealed for PerCubicMeter {}
impl LinearUnit<dimensions::NumberDensity> for PerCubicMeter {
    const SYMBOL: &'static str = "m⁻³";
    const SCALE: f64 = 1.0;
}

/// Metre per second, the coherent SI unit of velocity.
#[derive(Clone, Copy, Debug, Default)]
pub struct MeterPerSecond;
impl Sealed for MeterPerSecond {}
impl LinearUnit<dimensions::Velocity> for MeterPerSecond {
    const SYMBOL: &'static str = "m/s";
    const SCALE: f64 = 1.0;
}

/// Hertz, the coherent SI unit of frequency.
#[derive(Clone, Copy, Debug, Default)]
pub struct Hertz;
impl Sealed for Hertz {}
impl LinearUnit<dimensions::Frequency> for Hertz {
    const SYMBOL: &'static str = "Hz";
    const SCALE: f64 = 1.0;
}

/// Per second, the coherent SI unit of reciprocal time.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerSecond;
impl Sealed for PerSecond {}
impl LinearUnit<dimensions::ReciprocalTime> for PerSecond {
    const SYMBOL: &'static str = "s⁻¹";
    const SCALE: f64 = 1.0;
}

/// Pascal, the coherent SI unit of pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pascal;
impl Sealed for Pascal {}
impl LinearUnit<dimensions::Pressure> for Pascal {
    const SYMBOL: &'static str = "Pa";
    const SCALE: f64 = 1.0;
}

/// Newton, the coherent SI unit of force.
#[derive(Clone, Copy, Debug, Default)]
pub struct Newton;
impl Sealed for Newton {}
impl LinearUnit<dimensions::Force> for Newton {
    const SYMBOL: &'static str = "N";
    const SCALE: f64 = 1.0;
}

/// Coulomb, the coherent SI unit of electric charge.
#[derive(Clone, Copy, Debug, Default)]
pub struct Coulomb;
impl Sealed for Coulomb {}
impl LinearUnit<dimensions::ElectricCharge> for Coulomb {
    const SYMBOL: &'static str = "C";
    const SCALE: f64 = 1.0;
}

/// Volt, the coherent SI unit of electric potential.
#[derive(Clone, Copy, Debug, Default)]
pub struct Volt;
impl Sealed for Volt {}
impl LinearUnit<dimensions::ElectricPotential> for Volt {
    const SYMBOL: &'static str = "V";
    const SCALE: f64 = 1.0;
}

/// Siemens, the coherent SI unit of electric conductance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Siemens;
impl Sealed for Siemens {}
impl LinearUnit<dimensions::ElectricConductance> for Siemens {
    const SYMBOL: &'static str = "S";
    const SCALE: f64 = 1.0;
}

/// Ohm, the coherent SI unit of electrical impedance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ohm;
impl Sealed for Ohm {}
impl LinearUnit<dimensions::ElectricalImpedance> for Ohm {
    const SYMBOL: &'static str = "Ω";
    const SCALE: f64 = 1.0;
}

/// Farad, the coherent SI unit of capacitance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Farad;
impl Sealed for Farad {}
impl LinearUnit<dimensions::Capacitance> for Farad {
    const SYMBOL: &'static str = "F";
    const SCALE: f64 = 1.0;
}

/// Joule, the coherent SI unit of energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct Joule;
impl Sealed for Joule {}
impl LinearUnit<dimensions::Energy> for Joule {
    const SYMBOL: &'static str = "J";
    const SCALE: f64 = 1.0;
}

/// Joule per square metre, the coherent SI unit of energy per area.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerSquareMeter;
impl Sealed for JoulePerSquareMeter {}
impl LinearUnit<dimensions::EnergyPerArea> for JoulePerSquareMeter {
    const SYMBOL: &'static str = "J/m²";
    const SCALE: f64 = 1.0;
}

/// Newton per metre, the coherent SI unit of surface tension.
#[derive(Clone, Copy, Debug, Default)]
pub struct NewtonPerMeter;
impl Sealed for NewtonPerMeter {}
impl LinearUnit<dimensions::SurfaceTension> for NewtonPerMeter {
    const SYMBOL: &'static str = "N/m";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::SpringStiffness> for NewtonPerMeter {
    const SYMBOL: &'static str = "N/m";
    const SCALE: f64 = 1.0;
}

/// Kilogram per second, the coherent SI unit of mechanical damping coefficient.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerSecond;
impl Sealed for KilogramPerSecond {}
impl LinearUnit<dimensions::DampingCoefficient> for KilogramPerSecond {
    const SYMBOL: &'static str = "kg/s";
    const SCALE: f64 = 1.0;
}

/// Pascal per volt, the coherent SI unit of voltage-driven pressure gain.
#[derive(Clone, Copy, Debug, Default)]
pub struct PascalPerVolt;
impl Sealed for PascalPerVolt {}
impl LinearUnit<dimensions::PressurePerElectricPotential> for PascalPerVolt {
    const SYMBOL: &'static str = "Pa/V";
    const SCALE: f64 = 1.0;
}

/// Volt per pascal, the coherent SI unit of pressure-to-voltage sensitivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct VoltPerPascal;
impl Sealed for VoltPerPascal {}
impl LinearUnit<dimensions::ElectricPotentialPerPressure> for VoltPerPascal {
    const SYMBOL: &'static str = "V/Pa";
    const SCALE: f64 = 1.0;
}

/// Metre per volt, the coherent SI unit of voltage-driven displacement.
#[derive(Clone, Copy, Debug, Default)]
pub struct MeterPerVolt;
impl Sealed for MeterPerVolt {}
impl LinearUnit<dimensions::LengthPerElectricPotential> for MeterPerVolt {
    const SYMBOL: &'static str = "m/V";
    const SCALE: f64 = 1.0;
}

/// Coulomb per square metre, the coherent SI unit of surface charge density.
#[derive(Clone, Copy, Debug, Default)]
pub struct CoulombPerSquareMeter;
impl Sealed for CoulombPerSquareMeter {}
impl LinearUnit<dimensions::SurfaceChargeDensity> for CoulombPerSquareMeter {
    const SYMBOL: &'static str = "C/m²";
    const SCALE: f64 = 1.0;
}

/// Joule per cubic metre, the coherent SI unit of energy per volume.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerCubicMeter;
impl Sealed for JoulePerCubicMeter {}
impl LinearUnit<dimensions::EnergyPerVolume> for JoulePerCubicMeter {
    const SYMBOL: &'static str = "J/m³";
    const SCALE: f64 = 1.0;
}

/// Gray, the coherent SI unit of absorbed dose.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gray;
impl Sealed for Gray {}
impl LinearUnit<dimensions::AbsorbedDose> for Gray {
    const SYMBOL: &'static str = "Gy";
    const SCALE: f64 = 1.0;
}

/// Gray per second, the coherent SI unit of absorbed dose rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct GrayPerSecond;
impl Sealed for GrayPerSecond {}
impl LinearUnit<dimensions::AbsorbedDoseRate> for GrayPerSecond {
    const SYMBOL: &'static str = "Gy/s";
    const SCALE: f64 = 1.0;
}

/// Watt per kilogram, the specific-absorption-rate spelling of the coherent SI
/// absorbed-dose-rate unit. Numerically identical to [`GrayPerSecond`]; the two
/// exist so radiofrequency dosimetry and radiation dosimetry each read in their
/// own vocabulary without a conversion.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerKilogram;
impl Sealed for WattPerKilogram {}
impl LinearUnit<dimensions::SpecificAbsorptionRate> for WattPerKilogram {
    const SYMBOL: &'static str = "W/kg";
    const SCALE: f64 = 1.0;
}

/// Joule per mole, the coherent SI unit of molar energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerMole;
impl Sealed for JoulePerMole {}
impl LinearUnit<dimensions::MolarEnergy> for JoulePerMole {
    const SYMBOL: &'static str = "J/mol";
    const SCALE: f64 = 1.0;
}

/// Joule per mole-kelvin, the coherent SI unit of molar heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerMoleKelvin;
impl Sealed for JoulePerMoleKelvin {}
impl LinearUnit<dimensions::MolarHeatCapacity> for JoulePerMoleKelvin {
    const SYMBOL: &'static str = "J/(mol·K)";
    const SCALE: f64 = 1.0;
}

/// Watt, the coherent SI unit of power.
#[derive(Clone, Copy, Debug, Default)]
pub struct Watt;
impl Sealed for Watt {}
impl LinearUnit<dimensions::Power> for Watt {
    const SYMBOL: &'static str = "W";
    const SCALE: f64 = 1.0;
}

/// Joule per kelvin, the coherent SI unit of heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerKelvin;
impl Sealed for JoulePerKelvin {}
impl LinearUnit<dimensions::HeatCapacity> for JoulePerKelvin {
    const SYMBOL: &'static str = "J/K";
    const SCALE: f64 = 1.0;
}

/// Joule per kilogram-kelvin, the coherent SI unit of specific heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerKilogramKelvin;
impl Sealed for JoulePerKilogramKelvin {}
impl LinearUnit<dimensions::SpecificHeatCapacity> for JoulePerKilogramKelvin {
    const SYMBOL: &'static str = "J/(kg·K)";
    const SCALE: f64 = 1.0;
}

/// Watt per metre-kelvin, the coherent SI unit of thermal conductivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerMeterKelvin;
impl Sealed for WattPerMeterKelvin {}
impl LinearUnit<dimensions::ThermalConductivity> for WattPerMeterKelvin {
    const SYMBOL: &'static str = "W/(m·K)";
    const SCALE: f64 = 1.0;
}

/// Square metre per second, the coherent SI unit of thermal diffusivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeterPerSecond;
impl Sealed for SquareMeterPerSecond {}
impl LinearUnit<dimensions::ThermalDiffusivity> for SquareMeterPerSecond {
    const SYMBOL: &'static str = "m²/s";
    const SCALE: f64 = 1.0;
}

/// Kilogram per cubic metre, the coherent SI unit of mass density.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerCubicMeter;
impl Sealed for KilogramPerCubicMeter {}
impl LinearUnit<dimensions::MassDensity> for KilogramPerCubicMeter {
    const SYMBOL: &'static str = "kg/m³";
    const SCALE: f64 = 1.0;
}

/// Kilogram per cubic metre-second, the coherent SI unit of mass-density rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerCubicMeterSecond;
impl Sealed for KilogramPerCubicMeterSecond {}
impl LinearUnit<dimensions::MassDensityRate> for KilogramPerCubicMeterSecond {
    const SYMBOL: &'static str = "kg/(m³·s)";
    const SCALE: f64 = 1.0;
}

/// Pascal second, the coherent SI unit of dynamic viscosity.
#[derive(Clone, Copy, Debug, Default)]
pub struct PascalSecond;
impl Sealed for PascalSecond {}
impl LinearUnit<dimensions::DynamicViscosity> for PascalSecond {
    const SYMBOL: &'static str = "Pa·s";
    const SCALE: f64 = 1.0;
}

/// Cubic metre per second, the coherent SI unit of volumetric flow rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct CubicMeterPerSecond;
impl Sealed for CubicMeterPerSecond {}
impl LinearUnit<dimensions::VolumetricFlowRate> for CubicMeterPerSecond {
    const SYMBOL: &'static str = "m³/s";
    const SCALE: f64 = 1.0;
}

/// Rayl, the coherent SI unit of acoustic impedance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rayl;
impl Sealed for Rayl {}
impl LinearUnit<dimensions::AcousticImpedance> for Rayl {
    const SYMBOL: &'static str = "Rayl";
    const SCALE: f64 = 1.0;
}

/// Watt per square metre, the coherent SI unit of intensity.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerSquareMeter;
impl Sealed for WattPerSquareMeter {}
impl LinearUnit<dimensions::Intensity> for WattPerSquareMeter {
    const SYMBOL: &'static str = "W/m²";
    const SCALE: f64 = 1.0;
}

/// Watt per cubic metre, the coherent SI unit of volumetric power density.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerCubicMeter;
impl Sealed for WattPerCubicMeter {}
impl LinearUnit<dimensions::VolumetricPowerDensity> for WattPerCubicMeter {
    const SYMBOL: &'static str = "W/m³";
    const SCALE: f64 = 1.0;
}

/// Per kelvin, the coherent SI unit of reciprocal temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerKelvin;
impl Sealed for PerKelvin {}
impl LinearUnit<dimensions::ReciprocalTemperature> for PerKelvin {
    const SYMBOL: &'static str = "K⁻¹";
    const SCALE: f64 = 1.0;
}

/// Per square kelvin, the coherent SI unit of reciprocal squared temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerSquareKelvin;
impl Sealed for PerSquareKelvin {}
impl LinearUnit<dimensions::ReciprocalTemperatureSquared> for PerSquareKelvin {
    const SYMBOL: &'static str = "K⁻²";
    const SCALE: f64 = 1.0;
}
