use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Pascal, the coherent SI unit of pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pascal;
impl Sealed for Pascal {}
impl crate::unit::private::Named for Pascal {}
impl crate::unit::UnitDimension for Pascal {
    type Dimension = dimensions::Pressure;
}

impl LinearUnit<dimensions::Pressure> for Pascal {
    const SYMBOL: &'static str = "Pa";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::Stress> for Pascal {
    const SYMBOL: &'static str = "Pa";
    const SCALE: f64 = 1.0;
}

/// Pascal per second, the coherent SI unit of pressure rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct PascalPerSecond;
impl Sealed for PascalPerSecond {}
impl crate::unit::private::Named for PascalPerSecond {}
impl crate::unit::UnitDimension for PascalPerSecond {
    type Dimension = dimensions::PressureRate;
}

impl LinearUnit<dimensions::PressureRate> for PascalPerSecond {
    const SYMBOL: &'static str = "Pa/s";
    const SCALE: f64 = 1.0;
}

/// Newton, the coherent SI unit of force.
#[derive(Clone, Copy, Debug, Default)]
pub struct Newton;
impl Sealed for Newton {}
impl crate::unit::private::Named for Newton {}
impl crate::unit::UnitDimension for Newton {
    type Dimension = dimensions::Force;
}

impl LinearUnit<dimensions::Force> for Newton {
    const SYMBOL: &'static str = "N";
    const SCALE: f64 = 1.0;
}

/// Joule, the coherent SI unit of energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct Joule;
impl Sealed for Joule {}
impl crate::unit::private::Named for Joule {}
impl crate::unit::UnitDimension for Joule {
    type Dimension = dimensions::Energy;
}

impl LinearUnit<dimensions::Energy> for Joule {
    const SYMBOL: &'static str = "J";
    const SCALE: f64 = 1.0;
}

/// Joule per square metre, the coherent SI unit of energy per area.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerSquareMeter;
impl Sealed for JoulePerSquareMeter {}
impl crate::unit::private::Named for JoulePerSquareMeter {}
impl crate::unit::UnitDimension for JoulePerSquareMeter {
    type Dimension = dimensions::EnergyPerArea;
}

impl LinearUnit<dimensions::EnergyPerArea> for JoulePerSquareMeter {
    const SYMBOL: &'static str = "J/m²";
    const SCALE: f64 = 1.0;
}

/// Newton per metre, the coherent SI unit of surface tension.
#[derive(Clone, Copy, Debug, Default)]
pub struct NewtonPerMeter;
impl Sealed for NewtonPerMeter {}
impl crate::unit::private::Named for NewtonPerMeter {}
impl crate::unit::UnitDimension for NewtonPerMeter {
    type Dimension = dimensions::SurfaceTension;
}

impl LinearUnit<dimensions::SurfaceTension> for NewtonPerMeter {
    const SYMBOL: &'static str = "N/m";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::SpringStiffness> for NewtonPerMeter {
    const SYMBOL: &'static str = "N/m";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::FlexuralRigidity> for Joule {
    const SYMBOL: &'static str = "J";
    const SCALE: f64 = 1.0;
}

/// Kilogram per second, the coherent SI unit of mechanical damping coefficient.
#[derive(Clone, Copy, Debug, Default)]
pub struct KilogramPerSecond;
impl Sealed for KilogramPerSecond {}
impl crate::unit::private::Named for KilogramPerSecond {}
impl crate::unit::UnitDimension for KilogramPerSecond {
    type Dimension = dimensions::DampingCoefficient;
}

impl LinearUnit<dimensions::DampingCoefficient> for KilogramPerSecond {
    const SYMBOL: &'static str = "kg/s";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::MechanicalImpedance> for KilogramPerSecond {
    const SYMBOL: &'static str = "kg/s";
    const SCALE: f64 = 1.0;
}
