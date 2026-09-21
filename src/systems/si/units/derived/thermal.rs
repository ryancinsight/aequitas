use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Joule per cubic metre, the coherent SI unit of energy per volume.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerCubicMeter;
impl Sealed for JoulePerCubicMeter {}
impl crate::unit::private::Named for JoulePerCubicMeter {}
impl crate::unit::UnitDimension for JoulePerCubicMeter {
    type Dimension = dimensions::EnergyPerVolume;
}

impl LinearUnit<dimensions::EnergyPerVolume> for JoulePerCubicMeter {
    const SYMBOL: &'static str = "J/m³";
    const SCALE: f64 = 1.0;
}

/// Joule per mole, the coherent SI unit of molar energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerMole;
impl Sealed for JoulePerMole {}
impl crate::unit::private::Named for JoulePerMole {}
impl crate::unit::UnitDimension for JoulePerMole {
    type Dimension = dimensions::MolarEnergy;
}

impl LinearUnit<dimensions::MolarEnergy> for JoulePerMole {
    const SYMBOL: &'static str = "J/mol";
    const SCALE: f64 = 1.0;
}

/// Joule per mole-kelvin, the coherent SI unit of molar heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerMoleKelvin;
impl Sealed for JoulePerMoleKelvin {}
impl crate::unit::private::Named for JoulePerMoleKelvin {}
impl crate::unit::UnitDimension for JoulePerMoleKelvin {
    type Dimension = dimensions::MolarHeatCapacity;
}

impl LinearUnit<dimensions::MolarHeatCapacity> for JoulePerMoleKelvin {
    const SYMBOL: &'static str = "J/(mol·K)";
    const SCALE: f64 = 1.0;
}

/// Watt, the coherent SI unit of power.
#[derive(Clone, Copy, Debug, Default)]
pub struct Watt;
impl Sealed for Watt {}
impl crate::unit::private::Named for Watt {}
impl crate::unit::UnitDimension for Watt {
    type Dimension = dimensions::Power;
}

impl LinearUnit<dimensions::Power> for Watt {
    const SYMBOL: &'static str = "W";
    const SCALE: f64 = 1.0;
}

/// Joule per kelvin, the coherent SI unit of heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerKelvin;
impl Sealed for JoulePerKelvin {}
impl crate::unit::private::Named for JoulePerKelvin {}
impl crate::unit::UnitDimension for JoulePerKelvin {
    type Dimension = dimensions::HeatCapacity;
}

impl LinearUnit<dimensions::HeatCapacity> for JoulePerKelvin {
    const SYMBOL: &'static str = "J/K";
    const SCALE: f64 = 1.0;
}

/// Joule per kilogram-kelvin, the coherent SI unit of specific heat capacity.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerKilogramKelvin;
impl Sealed for JoulePerKilogramKelvin {}
impl crate::unit::private::Named for JoulePerKilogramKelvin {}
impl crate::unit::UnitDimension for JoulePerKilogramKelvin {
    type Dimension = dimensions::SpecificHeatCapacity;
}

impl LinearUnit<dimensions::SpecificHeatCapacity> for JoulePerKilogramKelvin {
    const SYMBOL: &'static str = "J/(kg·K)";
    const SCALE: f64 = 1.0;
}

/// Watt per metre-kelvin, the coherent SI unit of thermal conductivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerMeterKelvin;
impl Sealed for WattPerMeterKelvin {}
impl crate::unit::private::Named for WattPerMeterKelvin {}
impl crate::unit::UnitDimension for WattPerMeterKelvin {
    type Dimension = dimensions::ThermalConductivity;
}

impl LinearUnit<dimensions::ThermalConductivity> for WattPerMeterKelvin {
    const SYMBOL: &'static str = "W/(m·K)";
    const SCALE: f64 = 1.0;
}

/// Square metre per second, the coherent SI unit of thermal diffusivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeterPerSecond;
impl Sealed for SquareMeterPerSecond {}
impl crate::unit::private::Named for SquareMeterPerSecond {}
impl crate::unit::UnitDimension for SquareMeterPerSecond {
    type Dimension = dimensions::ThermalDiffusivity;
}

impl LinearUnit<dimensions::ThermalDiffusivity> for SquareMeterPerSecond {
    const SYMBOL: &'static str = "m²/s";
    const SCALE: f64 = 1.0;
}

/// Kilogram per cubic metre, the coherent SI unit of mass density.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerCubicMeter;
impl Sealed for KilogramPerCubicMeter {}
impl crate::unit::private::Named for KilogramPerCubicMeter {}
impl crate::unit::UnitDimension for KilogramPerCubicMeter {
    type Dimension = dimensions::MassDensity;
}

impl LinearUnit<dimensions::MassDensity> for KilogramPerCubicMeter {
    const SYMBOL: &'static str = "kg/m³";
    const SCALE: f64 = 1.0;
}

/// Kilogram per cubic metre-kelvin, the coherent SI unit of a mass-density
/// temperature derivative.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerCubicMeterKelvin;
impl Sealed for KilogramPerCubicMeterKelvin {}
impl crate::unit::private::Named for KilogramPerCubicMeterKelvin {}
impl crate::unit::UnitDimension for KilogramPerCubicMeterKelvin {
    type Dimension = dimensions::MassDensityPerTemperature;
}

impl LinearUnit<dimensions::MassDensityPerTemperature> for KilogramPerCubicMeterKelvin {
    const SYMBOL: &'static str = "kg/(m³·K)";
    const SCALE: f64 = 1.0;
}
