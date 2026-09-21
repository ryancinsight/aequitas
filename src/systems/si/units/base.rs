use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Radian, the coherent SI unit for plane and rotational angles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Radian;
impl Sealed for Radian {}
impl crate::unit::private::Named for Radian {}
impl crate::unit::UnitDimension for Radian {
    type Dimension = dimensions::Angle;
}

impl LinearUnit<dimensions::Angle> for Radian {
    const SYMBOL: &'static str = "rad";
    const SCALE: f64 = 1.0;
}

/// Metre, the SI base unit of length.
#[derive(Clone, Copy, Debug, Default)]
pub struct Meter;
impl Sealed for Meter {}
impl crate::unit::private::Named for Meter {}
impl crate::unit::UnitDimension for Meter {
    type Dimension = dimensions::Length;
}

impl LinearUnit<dimensions::Length> for Meter {
    const SYMBOL: &'static str = "m";
    const SCALE: f64 = 1.0;
}

/// Kilogram, the SI base unit of mass.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kilogram;
impl Sealed for Kilogram {}
impl crate::unit::private::Named for Kilogram {}
impl crate::unit::UnitDimension for Kilogram {
    type Dimension = dimensions::Mass;
}

impl LinearUnit<dimensions::Mass> for Kilogram {
    const SYMBOL: &'static str = "kg";
    const SCALE: f64 = 1.0;
}

/// Second, the SI base unit of time.
#[derive(Clone, Copy, Debug, Default)]
pub struct Second;
impl Sealed for Second {}
impl crate::unit::private::Named for Second {}
impl crate::unit::UnitDimension for Second {
    type Dimension = dimensions::Time;
}

impl LinearUnit<dimensions::Time> for Second {
    const SYMBOL: &'static str = "s";
    const SCALE: f64 = 1.0;
}

/// Ampere, the SI base unit of electric current.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ampere;
impl Sealed for Ampere {}
impl crate::unit::private::Named for Ampere {}
impl crate::unit::UnitDimension for Ampere {
    type Dimension = dimensions::ElectricCurrent;
}

impl LinearUnit<dimensions::ElectricCurrent> for Ampere {
    const SYMBOL: &'static str = "A";
    const SCALE: f64 = 1.0;
}

/// Kelvin, the SI base unit of thermodynamic temperature.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kelvin;
impl Sealed for Kelvin {}
impl crate::unit::private::Named for Kelvin {}
impl crate::unit::UnitDimension for Kelvin {
    type Dimension = dimensions::TemperatureDifference;
}

impl LinearUnit<dimensions::ThermodynamicTemperature> for Kelvin {
    const SYMBOL: &'static str = "K";
    const SCALE: f64 = 1.0;
}

impl LinearUnit<dimensions::TemperatureDifference> for Kelvin {
    const SYMBOL: &'static str = "K";
    const SCALE: f64 = 1.0;
}

/// Mole, the SI base unit of amount of substance.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mole;
impl Sealed for Mole {}
impl crate::unit::private::Named for Mole {}
impl crate::unit::UnitDimension for Mole {
    type Dimension = dimensions::AmountOfSubstance;
}

impl LinearUnit<dimensions::AmountOfSubstance> for Mole {
    const SYMBOL: &'static str = "mol";
    const SCALE: f64 = 1.0;
}

/// Candela, the SI base unit of luminous intensity.
#[derive(Clone, Copy, Debug, Default)]
pub struct Candela;
impl Sealed for Candela {}
impl crate::unit::private::Named for Candela {}
impl crate::unit::UnitDimension for Candela {
    type Dimension = dimensions::LuminousIntensity;
}

impl LinearUnit<dimensions::LuminousIntensity> for Candela {
    const SYMBOL: &'static str = "cd";
    const SCALE: f64 = 1.0;
}
