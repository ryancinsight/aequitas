use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Millimetre.
#[derive(Clone, Copy, Debug, Default)]
pub struct Millimeter;
impl Sealed for Millimeter {}
impl LinearUnit<dimensions::Length> for Millimeter {
    const SYMBOL: &'static str = "mm";
    const SCALE: f64 = 1.0e-3;
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

/// Megapascal.
#[derive(Clone, Copy, Debug, Default)]
pub struct Megapascal;
impl Sealed for Megapascal {}
impl LinearUnit<dimensions::Pressure> for Megapascal {
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
