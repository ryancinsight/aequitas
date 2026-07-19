use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Metre, the SI base unit of length.
#[derive(Clone, Copy, Debug, Default)]
pub struct Meter;
impl Sealed for Meter {}
impl LinearUnit<dimensions::Length> for Meter {
    const SYMBOL: &'static str = "m";
    const SCALE: f64 = 1.0;
}

/// Kilogram, the SI base unit of mass.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kilogram;
impl Sealed for Kilogram {}
impl LinearUnit<dimensions::Mass> for Kilogram {
    const SYMBOL: &'static str = "kg";
    const SCALE: f64 = 1.0;
}

/// Second, the SI base unit of time.
#[derive(Clone, Copy, Debug, Default)]
pub struct Second;
impl Sealed for Second {}
impl LinearUnit<dimensions::Time> for Second {
    const SYMBOL: &'static str = "s";
    const SCALE: f64 = 1.0;
}

/// Ampere, the SI base unit of electric current.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ampere;
impl Sealed for Ampere {}
impl LinearUnit<dimensions::ElectricCurrent> for Ampere {
    const SYMBOL: &'static str = "A";
    const SCALE: f64 = 1.0;
}

/// Kelvin, the SI base unit of thermodynamic temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kelvin;
impl Sealed for Kelvin {}
impl LinearUnit<dimensions::ThermodynamicTemperature> for Kelvin {
    const SYMBOL: &'static str = "K";
    const SCALE: f64 = 1.0;
}

/// Mole, the SI base unit of amount of substance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mole;
impl Sealed for Mole {}
impl LinearUnit<dimensions::AmountOfSubstance> for Mole {
    const SYMBOL: &'static str = "mol";
    const SCALE: f64 = 1.0;
}

/// Candela, the SI base unit of luminous intensity.
#[derive(Clone, Copy, Debug, Default)]
pub struct Candela;
impl Sealed for Candela {}
impl LinearUnit<dimensions::LuminousIntensity> for Candela {
    const SYMBOL: &'static str = "cd";
    const SCALE: f64 = 1.0;
}
