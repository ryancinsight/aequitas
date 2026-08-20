use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Coulomb, the coherent SI unit of electric charge.
#[derive(Clone, Copy, Debug, Default)]
pub struct Coulomb;
impl Sealed for Coulomb {}
impl LinearUnit<dimensions::ElectricCharge> for Coulomb {
    const SYMBOL: &'static str = "C";
    const SCALE: f64 = 1.0;
}

/// Coulomb per cubic metre, the coherent SI unit of volume charge density.
#[derive(Clone, Copy, Debug, Default)]
pub struct CoulombPerCubicMeter;
impl Sealed for CoulombPerCubicMeter {}
impl LinearUnit<dimensions::VolumeChargeDensity> for CoulombPerCubicMeter {
    const SYMBOL: &'static str = "C/m³";
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

/// Siemens per metre, the coherent SI unit of electrical conductivity.
#[derive(Clone, Copy, Debug, Default)]
pub struct SiemensPerMeter;
impl Sealed for SiemensPerMeter {}
impl LinearUnit<dimensions::ElectricalConductivity> for SiemensPerMeter {
    const SYMBOL: &'static str = "S/m";
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

/// Farad square metre, the coherent SI unit of electric polarizability.
#[derive(Clone, Copy, Debug, Default)]
pub struct FaradSquareMeter;
impl Sealed for FaradSquareMeter {}
impl LinearUnit<dimensions::Polarizability> for FaradSquareMeter {
    const SYMBOL: &'static str = "F·m²";
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
