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

/// Gray, the coherent SI unit of absorbed dose.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gray;
impl Sealed for Gray {}
impl LinearUnit<dimensions::AbsorbedDose> for Gray {
    const SYMBOL: &'static str = "Gy";
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
