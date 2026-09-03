use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Pascal, the coherent SI unit of pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pascal;
impl Sealed for Pascal {}
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
impl LinearUnit<dimensions::PressureRate> for PascalPerSecond {
    const SYMBOL: &'static str = "Pa/s";
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

impl LinearUnit<dimensions::FlexuralRigidity> for Joule {
    const SYMBOL: &'static str = "J";
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

impl LinearUnit<dimensions::MechanicalImpedance> for KilogramPerSecond {
    const SYMBOL: &'static str = "kg/s";
    const SCALE: f64 = 1.0;
}
