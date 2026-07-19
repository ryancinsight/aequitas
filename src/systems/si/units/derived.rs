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

/// Watt per metre-kelvin, the coherent SI unit of thermal conductivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerMeterKelvin;
impl Sealed for WattPerMeterKelvin {}
impl LinearUnit<dimensions::ThermalConductivity> for WattPerMeterKelvin {
    const SYMBOL: &'static str = "W/(m·K)";
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
