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

linear_units! {
    /// `Becquerel`, expressed in `Bq`.
    Becquerel, Activity, "Bq", 1.0;
    /// `Kilobecquerel`, expressed in `kBq`.
    Kilobecquerel, Activity, "kBq", 1.0e3;
    /// `Megabecquerel`, expressed in `MBq`.
    Megabecquerel, Activity, "MBq", 1.0e6;
    /// `Gigabecquerel`, expressed in `GBq`.
    Gigabecquerel, Activity, "GBq", 1.0e9;
    /// `Curie`, expressed in `Ci`.
    Curie, Activity, "Ci", 3.7e10;
    /// `Millicurie`, expressed in `mCi`.
    Millicurie, Activity, "mCi", 3.7e7;
    /// `Microcurie`, expressed in `µCi`.
    Microcurie, Activity, "µCi", 3.7e4;
    /// `BecquerelPerCubicMeter`, expressed in `Bq/m³`.
    BecquerelPerCubicMeter, ActivityConcentration, "Bq/m³", 1.0;
    /// `BecquerelPerMilliliter`, expressed in `Bq/mL`.
    BecquerelPerMilliliter, ActivityConcentration, "Bq/mL", 1.0e6;
    /// `KilobecquerelPerMilliliter`, expressed in `kBq/mL`.
    KilobecquerelPerMilliliter, ActivityConcentration, "kBq/mL", 1.0e9;
    /// `Sievert`, expressed in `Sv`.
    Sievert, DoseEquivalent, "Sv", 1.0;
    /// `Millisievert`, expressed in `mSv`.
    Millisievert, DoseEquivalent, "mSv", 1.0e-3;
    /// `Microsievert`, expressed in `µSv`.
    Microsievert, DoseEquivalent, "µSv", 1.0e-6;
    /// `Rem`, expressed in `rem`.
    Rem, DoseEquivalent, "rem", 1.0e-2;
    /// `SievertPerSecond`, expressed in `Sv/s`.
    SievertPerSecond, DoseEquivalentRate, "Sv/s", 1.0;
    /// `MicrosievertPerHour`, expressed in `µSv/h`.
    MicrosievertPerHour, DoseEquivalentRate, "µSv/h", 1.0e-6 / 3600.0;
    /// `CoulombPerKilogram`, expressed in `C/kg`.
    CoulombPerKilogram, RadiationExposure, "C/kg", 1.0;
    /// `Roentgen`, expressed in `R`.
    Roentgen, RadiationExposure, "R", 2.58e-4;
    /// `Milligray`, expressed in `mGy`.
    Milligray, AbsorbedDose, "mGy", 1.0e-3;
    /// `Microgray`, expressed in `µGy`.
    Microgray, AbsorbedDose, "µGy", 1.0e-6;
    /// `Centigray`, expressed in `cGy`.
    Centigray, AbsorbedDose, "cGy", 1.0e-2;
    /// `Rad`, expressed in `rad`.
    Rad, AbsorbedDose, "rad", 1.0e-2;
}
