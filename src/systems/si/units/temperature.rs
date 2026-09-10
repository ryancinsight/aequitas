//! Affine temperature units.
//!
//! Each unit here appears twice, and the pair is the point. A degree Celsius
//! naming a *temperature* is affine: its zero is 273.15 K. A degree Celsius
//! naming a *temperature difference* is linear: its zero is no change at all,
//! and ten of them are ten kelvin. Aequitas keeps those as separate
//! dimensions, so the same marker implements
//! [`AffineUnit`](crate::unit::AffineUnit) for
//! [`ThermodynamicTemperature`](dimensions::ThermodynamicTemperature) and
//! [`LinearUnit`] for
//! [`TemperatureDifference`](dimensions::TemperatureDifference). The quantity's
//! dimension selects the conversion, so a caller cannot pick the wrong one.
//!
//! This is the classic affine-unit defect made unrepresentable: subtracting
//! two Celsius temperatures and reading the result as a Celsius temperature
//! double-counts the offset by 273.15 K.

use crate::{
    systems::si::dimensions,
    unit::{AffineUnit, LinearUnit, private::Sealed},
};

/// Degree Celsius.
///
/// `t/°C = T/K − 273.15`, exactly, by the definition of the Celsius scale
/// against the kelvin (SI Brochure, 9th edition, section 2.3.1). The degree is
/// the same size as the kelvin, so [`SCALE`](AffineUnit::SCALE) is one and the
/// whole difference between the scales is the offset.
#[derive(Clone, Copy, Debug, Default)]
pub struct DegreeCelsius;
impl Sealed for DegreeCelsius {}

impl AffineUnit<dimensions::ThermodynamicTemperature> for DegreeCelsius {
    const SYMBOL: &'static str = "degC";
    const SCALE: f64 = 1.0;
    const OFFSET: f64 = 273.15;
}

/// A Celsius *difference* is a kelvin difference, offset and all absent.
impl LinearUnit<dimensions::TemperatureDifference> for DegreeCelsius {
    const SYMBOL: &'static str = "degC";
    const SCALE: f64 = 1.0;
}

/// Degree Fahrenheit.
///
/// `t/°F = T/K × 9/5 − 459.67`, so the degree is 5/9 kelvin and the scale
/// reads zero at 459.67 °F below the kelvin zero — that is, at
/// `273.15 − 32 × 5/9` kelvin.
#[derive(Clone, Copy, Debug, Default)]
pub struct DegreeFahrenheit;
impl Sealed for DegreeFahrenheit {}

impl AffineUnit<dimensions::ThermodynamicTemperature> for DegreeFahrenheit {
    const SYMBOL: &'static str = "degF";
    const SCALE: f64 = 5.0 / 9.0;
    /// The kelvin reading at 0 °F: `273.15 − 32 × 5/9`.
    const OFFSET: f64 = 273.15 - 32.0 * (5.0 / 9.0);
}

/// A Fahrenheit *difference* carries the degree size and none of the offset.
impl LinearUnit<dimensions::TemperatureDifference> for DegreeFahrenheit {
    const SYMBOL: &'static str = "degF";
    const SCALE: f64 = 5.0 / 9.0;
}
