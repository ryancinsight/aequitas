use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Degree of arc, the non-SI angle unit accepted for use with the SI.
///
/// `SCALE` is the exact defining ratio `pi / 180`: one degree is that many
/// radians, so `to_base` multiplies by it and `in_unit::<Degree>` divides.
/// `Angle` carries its own dimension semantics, so a degree value cannot be
/// mistaken for a bare scalar or for radians once it is inside a `Quantity`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Degree;
impl Sealed for Degree {}
impl LinearUnit<dimensions::Angle> for Degree {
    const SYMBOL: &'static str = "deg";
    const SCALE: f64 = core::f64::consts::PI / 180.0;
}

/// Joule per millilitre, a scaled energy-per-volume unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerMilliliter;
impl Sealed for JoulePerMilliliter {}
impl LinearUnit<dimensions::EnergyPerVolume> for JoulePerMilliliter {
    const SYMBOL: &'static str = "J/ml";
    const SCALE: f64 = 1.0e6;
}

/// Millimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct Millimeter;
impl Sealed for Millimeter {}
impl LinearUnit<dimensions::Length> for Millimeter {
    const SYMBOL: &'static str = "mm";
    const SCALE: f64 = 1.0e-3;
}

/// Nanometre.
#[derive(Clone, Copy, Debug, Default)]
pub struct Nanometer;
impl Sealed for Nanometer {}
impl LinearUnit<dimensions::Length> for Nanometer {
    const SYMBOL: &'static str = "nm";
    const SCALE: f64 = 1.0e-9;
}

/// Centimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct Centimeter;
impl Sealed for Centimeter {}
impl LinearUnit<dimensions::Length> for Centimeter {
    const SYMBOL: &'static str = "cm";
    const SCALE: f64 = 1.0e-2;
}

/// Per centimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerCentimeter;
impl Sealed for PerCentimeter {}
impl LinearUnit<dimensions::ReciprocalLength> for PerCentimeter {
    const SYMBOL: &'static str = "cm⁻¹";
    const SCALE: f64 = 1.0e2;
}

/// Kilometre.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kilometer;
impl Sealed for Kilometer {}
impl LinearUnit<dimensions::Length> for Kilometer {
    const SYMBOL: &'static str = "km";
    const SCALE: f64 = 1.0e3;
}

/// Millisecond.
#[derive(Clone, Copy, Debug, Default)]
pub struct Millisecond;
impl Sealed for Millisecond {}
impl LinearUnit<dimensions::Time> for Millisecond {
    const SYMBOL: &'static str = "ms";
    const SCALE: f64 = 1.0e-3;
}

/// Microsecond.
#[derive(Clone, Copy, Debug, Default)]
pub struct Microsecond;
impl Sealed for Microsecond {}
impl LinearUnit<dimensions::Time> for Microsecond {
    const SYMBOL: &'static str = "µs";
    const SCALE: f64 = 1.0e-6;
}

/// Micromole per litre, equivalent to millimole per cubic metre.
#[derive(Clone, Copy, Debug, Default)]
pub struct MicromolePerLiter;
impl Sealed for MicromolePerLiter {}
impl LinearUnit<dimensions::MolarConcentration> for MicromolePerLiter {
    const SYMBOL: &'static str = "µmol/L";
    const SCALE: f64 = 1.0e-3;
}

/// Mole per litre, the conventional molar concentration unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct MolePerLiter;
impl Sealed for MolePerLiter {}
impl LinearUnit<dimensions::MolarConcentration> for MolePerLiter {
    const SYMBOL: &'static str = "mol/L";
    const SCALE: f64 = 1.0e3;
}

/// Gram.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gram;
impl Sealed for Gram {}
impl LinearUnit<dimensions::Mass> for Gram {
    const SYMBOL: &'static str = "g";
    const SCALE: f64 = 1.0e-3;
}

/// Square centimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareCentimeter;
impl Sealed for SquareCentimeter {}
impl LinearUnit<dimensions::Area> for SquareCentimeter {
    const SYMBOL: &'static str = "cm²";
    const SCALE: f64 = 1.0e-4;
}

/// Square centimetre per gram.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareCentimeterPerGram;
impl Sealed for SquareCentimeterPerGram {}
impl LinearUnit<dimensions::AreaPerMass> for SquareCentimeterPerGram {
    const SYMBOL: &'static str = "cm²/g";
    const SCALE: f64 = 1.0e-1;
}

/// Cubic millimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct CubicMillimeter;
impl Sealed for CubicMillimeter {}
impl LinearUnit<dimensions::Volume> for CubicMillimeter {
    const SYMBOL: &'static str = "mm³";
    const SCALE: f64 = 1.0e-9;
}

/// Kilohertz.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kilohertz;
impl Sealed for Kilohertz {}
impl LinearUnit<dimensions::Frequency> for Kilohertz {
    const SYMBOL: &'static str = "kHz";
    const SCALE: f64 = 1.0e3;
}

/// Megahertz.
#[derive(Clone, Copy, Debug, Default)]
pub struct Megahertz;
impl Sealed for Megahertz {}
impl LinearUnit<dimensions::Frequency> for Megahertz {
    const SYMBOL: &'static str = "MHz";
    const SCALE: f64 = 1.0e6;
}

/// Kilopascal.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kilopascal;
impl Sealed for Kilopascal {}
impl LinearUnit<dimensions::Pressure> for Kilopascal {
    const SYMBOL: &'static str = "kPa";
    const SCALE: f64 = 1.0e3;
}

impl LinearUnit<dimensions::Stress> for Kilopascal {
    const SYMBOL: &'static str = "kPa";
    const SCALE: f64 = 1.0e3;
}

/// Megapascal.
#[derive(Clone, Copy, Debug, Default)]
pub struct Megapascal;
impl Sealed for Megapascal {}
impl LinearUnit<dimensions::Pressure> for Megapascal {
    const SYMBOL: &'static str = "MPa";
    const SCALE: f64 = 1.0e6;
}

impl LinearUnit<dimensions::Stress> for Megapascal {
    const SYMBOL: &'static str = "MPa";
    const SCALE: f64 = 1.0e6;
}

/// Electronvolt.
#[derive(Clone, Copy, Debug, Default)]
pub struct ElectronVolt;
impl Sealed for ElectronVolt {}
impl LinearUnit<dimensions::Energy> for ElectronVolt {
    const SYMBOL: &'static str = "eV";
    const SCALE: f64 = 1.602_176_634e-19;
}

/// Megaelectronvolt.
#[derive(Clone, Copy, Debug, Default)]
pub struct MegaElectronVolt;
impl Sealed for MegaElectronVolt {}
impl LinearUnit<dimensions::Energy> for MegaElectronVolt {
    const SYMBOL: &'static str = "MeV";
    const SCALE: f64 = 1.602_176_634e-13;
}

/// Gram per cubic centimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct GramPerCubicCentimeter;
impl Sealed for GramPerCubicCentimeter {}
impl LinearUnit<dimensions::MassDensity> for GramPerCubicCentimeter {
    const SYMBOL: &'static str = "g/cm³";
    const SCALE: f64 = 1.0e3;
}

/// Millipascal second, a common dynamic-viscosity unit.
#[derive(Clone, Copy, Debug, Default)]
pub struct MillipascalSecond;
impl Sealed for MillipascalSecond {}
impl LinearUnit<dimensions::DynamicViscosity> for MillipascalSecond {
    const SYMBOL: &'static str = "mPa·s";
    const SCALE: f64 = 1.0e-3;
}
