//! Decimal power units carry the nearest binary64 to their exact decimal scale.
//!
//! The oracle is Rust's float parser, which rounds a decimal literal to
//! nearest and shares no code with the scale computation under test.

use aequitas::{
    systems::si::{
        dimensions::{Area, TemperatureDifference, ThermodynamicTemperature, Volume},
        units::{
            Are, Centimeter, CubicCentimeter, CubicDecimeter, CubicMillimeter, Degree,
            DegreeFahrenheit, Fahrenheit, Hectare, Kilometer, Liter, Meter, Millimeter,
            SquareCentimeter, SquareDecameter, SquareDecimeter, SquareHectometer, SquareKilometer,
            SquareMillimeter,
        },
    },
    unit::{
        AffineUnit, LinearUnit, Power, Prefixed, Unit,
        prefixes::{Deci, Milli},
    },
};
use typenum::{N2, P2, P3};

fn nearest(decimal: &str) -> u64 {
    decimal
        .parse::<f64>()
        .expect("invariant: the literal is a valid float")
        .to_bits()
}

fn scale<U: LinearUnit<D>, D>() -> u64 {
    U::SCALE.to_bits()
}

#[test]
fn every_decimal_power_unit_is_the_nearest_double_to_its_decimal_value() {
    let cases = [
        (scale::<SquareMillimeter, Area>(), "1e-6", "mm²"),
        (scale::<SquareCentimeter, Area>(), "1e-4", "cm²"),
        (scale::<SquareDecimeter, Area>(), "1e-2", "dm²"),
        (scale::<SquareDecameter, Area>(), "1e2", "dam²"),
        (scale::<SquareHectometer, Area>(), "1e4", "hm²"),
        (scale::<SquareKilometer, Area>(), "1e6", "km²"),
        (scale::<CubicMillimeter, Volume>(), "1e-9", "mm³"),
        (scale::<CubicCentimeter, Volume>(), "1e-6", "cm³"),
        (scale::<CubicDecimeter, Volume>(), "1e-3", "dm³"),
    ];
    for (observed, decimal, symbol) in cases {
        assert_eq!(observed, nearest(decimal), "{symbol}");
    }
}

#[test]
fn units_that_predate_the_shared_mechanism_keep_their_literal_values() {
    assert_eq!(scale::<SquareCentimeter, Area>(), 1.0e-4_f64.to_bits());
    assert_eq!(scale::<CubicMillimeter, Volume>(), 1.0e-9_f64.to_bits());
}

#[test]
fn metric_power_spellings_denote_the_unit_they_alias() {
    assert_eq!(scale::<SquareDecameter, Area>(), scale::<Are, Area>());
    assert_eq!(scale::<SquareHectometer, Area>(), scale::<Hectare, Area>());
    assert_eq!(scale::<CubicDecimeter, Volume>(), scale::<Liter, Volume>());
    assert_eq!(
        scale::<CubicCentimeter, Volume>(),
        scale::<Prefixed<Milli, Liter>, Volume>()
    );
}

#[test]
fn a_power_of_a_decimal_unit_is_rounded_once() {
    assert_eq!(
        <Power<Prefixed<Deci, Meter>, P2> as Unit<Area>>::SCALE.to_bits(),
        nearest("1e-2")
    );
    assert_eq!(
        <Power<Centimeter, P3> as Unit<Volume>>::SCALE.to_bits(),
        nearest("1e-6")
    );
    assert_eq!(
        <Power<Kilometer, P3> as Unit<Volume>>::SCALE.to_bits(),
        nearest("1e9")
    );
    assert_eq!(
        <Power<Millimeter, N2> as Unit<_>>::SCALE.to_bits(),
        nearest("1e6")
    );
}

#[test]
fn a_power_of_a_non_decimal_unit_keeps_repeated_multiplication() {
    let degree = scale::<Degree, aequitas::systems::si::dimensions::Angle>();
    let degree = f64::from_bits(degree);
    assert_eq!(
        <Power<Degree, P2> as Unit<_>>::SCALE.to_bits(),
        (degree * degree).to_bits()
    );
}

#[test]
fn the_unicode_fahrenheit_marker_is_the_ascii_marker_under_another_symbol() {
    assert_eq!(
        <Fahrenheit as LinearUnit<TemperatureDifference>>::SYMBOL,
        "°F"
    );
    assert_eq!(
        scale::<Fahrenheit, TemperatureDifference>(),
        scale::<DegreeFahrenheit, TemperatureDifference>()
    );
    assert_eq!(
        <Fahrenheit as AffineUnit<ThermodynamicTemperature>>::SYMBOL,
        "°F"
    );
    assert_eq!(
        <Fahrenheit as AffineUnit<ThermodynamicTemperature>>::SCALE.to_bits(),
        <DegreeFahrenheit as AffineUnit<ThermodynamicTemperature>>::SCALE.to_bits()
    );
    assert_eq!(
        <Fahrenheit as AffineUnit<ThermodynamicTemperature>>::OFFSET.to_bits(),
        <DegreeFahrenheit as AffineUnit<ThermodynamicTemperature>>::OFFSET.to_bits()
    );
}
