use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Mole per cubic metre, the coherent SI unit of molar concentration.
#[derive(Clone, Copy, Debug, Default)]
pub struct MolePerCubicMeter;
impl Sealed for MolePerCubicMeter {}
impl LinearUnit<dimensions::MolarConcentration> for MolePerCubicMeter {
    const SYMBOL: &'static str = "mol/m³";
    const SCALE: f64 = 1.0;
}

/// Mole per cubic metre per second, the coherent SI unit of volumetric
/// reaction rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct MolePerCubicMeterSecond;
impl Sealed for MolePerCubicMeterSecond {}
impl LinearUnit<dimensions::ReactionRate> for MolePerCubicMeterSecond {
    const SYMBOL: &'static str = "mol/(m³·s)";
    const SCALE: f64 = 1.0;
}

/// Mole per square metre per second, the coherent SI unit of molar flux.
#[derive(Clone, Copy, Debug, Default)]
pub struct MolePerSquareMeterSecond;
impl Sealed for MolePerSquareMeterSecond {}
impl LinearUnit<dimensions::MolarFlux> for MolePerSquareMeterSecond {
    const SYMBOL: &'static str = "mol/(m²·s)";
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

/// Metre per second-kelvin, the coherent SI unit of a velocity temperature
/// derivative.
#[derive(Clone, Copy, Debug, Default)]
pub struct MeterPerSecondKelvin;
impl Sealed for MeterPerSecondKelvin {}
impl LinearUnit<dimensions::VelocityPerTemperature> for MeterPerSecondKelvin {
    const SYMBOL: &'static str = "m/(s·K)";
    const SCALE: f64 = 1.0;
}

/// Metre per second squared, the coherent SI unit of acceleration.
#[derive(Clone, Copy, Debug, Default)]
pub struct MeterPerSecondSquared;
impl Sealed for MeterPerSecondSquared {}
impl LinearUnit<dimensions::Acceleration> for MeterPerSecondSquared {
    const SYMBOL: &'static str = "m/s²";
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
