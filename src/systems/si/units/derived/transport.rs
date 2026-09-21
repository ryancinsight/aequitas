use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Kilogram per cubic metre-second, the coherent SI unit of mass-density rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerCubicMeterSecond;
impl Sealed for KilogramPerCubicMeterSecond {}
impl crate::unit::private::Named for KilogramPerCubicMeterSecond {}
impl crate::unit::UnitDimension for KilogramPerCubicMeterSecond {
    type Dimension = dimensions::MassDensityRate;
}

impl LinearUnit<dimensions::MassDensityRate> for KilogramPerCubicMeterSecond {
    const SYMBOL: &'static str = "kg/(m³·s)";
    const SCALE: f64 = 1.0;
}

/// Pascal second, the coherent SI unit of dynamic viscosity.
#[derive(Clone, Copy, Debug, Default)]
pub struct PascalSecond;
impl Sealed for PascalSecond {}
impl crate::unit::private::Named for PascalSecond {}
impl crate::unit::UnitDimension for PascalSecond {
    type Dimension = dimensions::DynamicViscosity;
}

impl LinearUnit<dimensions::DynamicViscosity> for PascalSecond {
    const SYMBOL: &'static str = "Pa·s";
    const SCALE: f64 = 1.0;
}

/// Cubic metre per second, the coherent SI unit of volumetric flow rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct CubicMeterPerSecond;
impl Sealed for CubicMeterPerSecond {}
impl crate::unit::private::Named for CubicMeterPerSecond {}
impl crate::unit::UnitDimension for CubicMeterPerSecond {
    type Dimension = dimensions::VolumetricFlowRate;
}

impl LinearUnit<dimensions::VolumetricFlowRate> for CubicMeterPerSecond {
    const SYMBOL: &'static str = "m³/s";
    const SCALE: f64 = 1.0;
}

/// Rayl, the coherent SI unit of acoustic impedance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rayl;
impl Sealed for Rayl {}
impl crate::unit::private::Named for Rayl {}
impl crate::unit::UnitDimension for Rayl {
    type Dimension = dimensions::AcousticImpedance;
}

impl LinearUnit<dimensions::AcousticImpedance> for Rayl {
    const SYMBOL: &'static str = "Rayl";
    const SCALE: f64 = 1.0;
}

/// Watt per square metre, the coherent SI unit of intensity.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerSquareMeter;
impl Sealed for WattPerSquareMeter {}
impl crate::unit::private::Named for WattPerSquareMeter {}
impl crate::unit::UnitDimension for WattPerSquareMeter {
    type Dimension = dimensions::Intensity;
}

impl LinearUnit<dimensions::Intensity> for WattPerSquareMeter {
    const SYMBOL: &'static str = "W/m²";
    const SCALE: f64 = 1.0;
}

/// Watt per cubic metre, the coherent SI unit of volumetric power density.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerCubicMeter;
impl Sealed for WattPerCubicMeter {}
impl crate::unit::private::Named for WattPerCubicMeter {}
impl crate::unit::UnitDimension for WattPerCubicMeter {
    type Dimension = dimensions::VolumetricPowerDensity;
}

impl LinearUnit<dimensions::VolumetricPowerDensity> for WattPerCubicMeter {
    const SYMBOL: &'static str = "W/m³";
    const SCALE: f64 = 1.0;
}

/// Watt per metre to the fourth power, the coherent SI unit of a volumetric
/// power-density gradient.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerMeterFourth;
impl Sealed for WattPerMeterFourth {}
impl crate::unit::private::Named for WattPerMeterFourth {}
impl crate::unit::UnitDimension for WattPerMeterFourth {
    type Dimension = dimensions::VolumetricPowerDensityGradient;
}

impl LinearUnit<dimensions::VolumetricPowerDensityGradient> for WattPerMeterFourth {
    const SYMBOL: &'static str = "W/m⁴";
    const SCALE: f64 = 1.0;
}

/// Per kelvin, the coherent SI unit of reciprocal temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerKelvin;
impl Sealed for PerKelvin {}
impl crate::unit::private::Named for PerKelvin {}
impl crate::unit::UnitDimension for PerKelvin {
    type Dimension = dimensions::ReciprocalTemperature;
}

impl LinearUnit<dimensions::ReciprocalTemperature> for PerKelvin {
    const SYMBOL: &'static str = "K⁻¹";
    const SCALE: f64 = 1.0;
}

/// Per square kelvin, the coherent SI unit of reciprocal squared temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerSquareKelvin;
impl Sealed for PerSquareKelvin {}
impl crate::unit::private::Named for PerSquareKelvin {}
impl crate::unit::UnitDimension for PerSquareKelvin {
    type Dimension = dimensions::ReciprocalTemperatureSquared;
}

impl LinearUnit<dimensions::ReciprocalTemperatureSquared> for PerSquareKelvin {
    const SYMBOL: &'static str = "K⁻²";
    const SCALE: f64 = 1.0;
}

/// Inverse metre-kelvin, the coherent SI unit of a reciprocal-length
/// temperature derivative.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerMeterKelvin;
impl Sealed for PerMeterKelvin {}
impl crate::unit::private::Named for PerMeterKelvin {}
impl crate::unit::UnitDimension for PerMeterKelvin {
    type Dimension = dimensions::ReciprocalLengthPerTemperature;
}

impl LinearUnit<dimensions::ReciprocalLengthPerTemperature> for PerMeterKelvin {
    const SYMBOL: &'static str = "1/(m·K)";
    const SCALE: f64 = 1.0;
}
