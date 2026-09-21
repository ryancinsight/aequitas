use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Gray, the coherent SI unit of absorbed dose.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gray;
impl Sealed for Gray {}
impl crate::unit::private::Named for Gray {}
impl crate::unit::UnitDimension for Gray {
    type Dimension = dimensions::AbsorbedDose;
}

impl LinearUnit<dimensions::AbsorbedDose> for Gray {
    const SYMBOL: &'static str = "Gy";
    const SCALE: f64 = 1.0;
}

/// Joule per kilogram, the coherent SI unit of specific energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoulePerKilogram;
impl Sealed for JoulePerKilogram {}
impl crate::unit::private::Named for JoulePerKilogram {}
impl crate::unit::UnitDimension for JoulePerKilogram {
    type Dimension = dimensions::SpecificEnergy;
}

impl LinearUnit<dimensions::SpecificEnergy> for JoulePerKilogram {
    const SYMBOL: &'static str = "J/kg";
    const SCALE: f64 = 1.0;
}

/// Gray per second, the coherent SI unit of absorbed dose rate.
#[derive(Clone, Copy, Debug, Default)]
pub struct GrayPerSecond;
impl Sealed for GrayPerSecond {}
impl crate::unit::private::Named for GrayPerSecond {}
impl crate::unit::UnitDimension for GrayPerSecond {
    type Dimension = dimensions::AbsorbedDoseRate;
}

impl LinearUnit<dimensions::AbsorbedDoseRate> for GrayPerSecond {
    const SYMBOL: &'static str = "Gy/s";
    const SCALE: f64 = 1.0;
}

/// Watt per kilogram, the specific-absorption-rate spelling of the coherent SI
/// absorbed-dose-rate unit. Numerically identical to [`GrayPerSecond`]; the two
/// exist so radiofrequency dosimetry and radiation dosimetry each read in their
/// own vocabulary without a conversion.
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerKilogram;
impl Sealed for WattPerKilogram {}
impl crate::unit::private::Named for WattPerKilogram {}
impl crate::unit::UnitDimension for WattPerKilogram {
    type Dimension = dimensions::SpecificAbsorptionRate;
}

impl LinearUnit<dimensions::SpecificAbsorptionRate> for WattPerKilogram {
    const SYMBOL: &'static str = "W/kg";
    const SCALE: f64 = 1.0;
}
