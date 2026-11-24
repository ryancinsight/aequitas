//! A power of a decimal unit carries the nearest binary64 to its exact decimal scale.
//!
//! The oracle is Rust's float parser, which rounds a decimal literal to
//! nearest and shares no code with the scale computation under test.

use aequitas::{
    systems::si::{
        dimensions::{
            Acceleration, Angle, Area, Mass, Pressure, TemperatureDifference,
            ThermodynamicTemperature, Volume,
        },
        units::{
            Are, Bar, Barn, Centimeter, CubicCentimeter, CubicDecimeter, CubicMillimeter, Degree,
            DegreeFahrenheit, Fahrenheit, Gal, Gram, Hectare, Kilometer, Liter, LiterLowercase,
            Meter, Millimeter, SquareCentimeter, SquareDecameter, SquareDecimeter,
            SquareHectometer, SquareKilometer, SquareMillimeter, Tonne,
        },
    },
    unit::{
        AffineUnit, LinearUnit, Power, Prefixed, Unit,
        prefixes::{Deci, Micro, Milli, Nano},
    },
};
use typenum::{N1, N2, N3, P2, P3, P4};

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

/// The result range the `Prefixed` and `Power` rustdoc guarantees.
const GUARANTEED_DECADE: i32 = 22;

fn nearest_decade(exponent: i32) -> u64 {
    nearest(&format!("1e{exponent}"))
}

#[test]
fn a_power_of_a_decimal_unit_is_rounded_once() {
    // 0.1 * 0.1 is one binary64 step above the nearest double to 0.01.
    assert_ne!((0.1_f64 * 0.1).to_bits(), nearest("1e-2"));
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
    let degree = <Degree as LinearUnit<Angle>>::SCALE;
    assert_eq!(
        <Power<Degree, P2> as Unit<_>>::SCALE.to_bits(),
        (degree * degree).to_bits()
    );
}

#[test]
fn a_prefixed_decimal_unit_is_the_nearest_double_to_its_decade() {
    // 1e5 * 1e-6 and 1e-3 * 1e-9 each land one step off the exact decade.
    assert_ne!((1.0e5_f64 * 1.0e-6).to_bits(), nearest("1e-1"));
    assert_ne!((1.0e-3_f64 * 1.0e-9).to_bits(), nearest("1e-12"));
    assert_eq!(
        <Prefixed<Micro, Bar> as LinearUnit<Pressure>>::SCALE.to_bits(),
        nearest("1e-1")
    );
    assert_eq!(
        <Prefixed<Nano, Liter> as LinearUnit<Volume>>::SCALE.to_bits(),
        nearest("1e-12")
    );
}

#[test]
fn a_power_of_a_prefixed_decimal_unit_is_rounded_once() {
    assert_eq!(
        <Power<Prefixed<Micro, Bar>, P2> as Unit<_>>::SCALE.to_bits(),
        nearest("1e-2")
    );
    assert_eq!(
        <Power<Prefixed<Nano, Liter>, N1> as Unit<_>>::SCALE.to_bits(),
        nearest("1e12")
    );
    assert_eq!(
        <Power<Prefixed<Micro, Bar>, P4> as Unit<_>>::SCALE.to_bits(),
        nearest("1e-4")
    );
    assert_eq!(
        <Power<Prefixed<Nano, Tonne>, N3> as Unit<_>>::SCALE.to_bits(),
        nearest("1e18")
    );
}

/// A prefixed scale is the nearest double to its decade wherever the decade is
/// within the guaranteed range, whatever the operand exponents.
fn assert_prefixed_scale<
    Dimension,
    Root: LinearUnit<Dimension>,
    Composite: LinearUnit<Dimension>,
>(
    root_exponent: i32,
    prefix_exponent: i32,
) {
    assert_eq!(
        Root::SCALE.to_bits(),
        nearest_decade(root_exponent),
        "the root is 1e{root_exponent}"
    );
    let exponent = root_exponent + prefix_exponent;
    if exponent.abs() <= GUARANTEED_DECADE {
        assert_eq!(
            Composite::SCALE.to_bits(),
            nearest_decade(exponent),
            "1e{root_exponent} prefixed by 1e{prefix_exponent}"
        );
    }
}

macro_rules! assert_every_decimal_prefix {
    ($dimension:ty, $root:ty, $root_exponent:expr) => {
        assert_every_decimal_prefix!(@each $dimension, $root, $root_exponent;
            Quetta 30, Ronna 27, Yotta 24, Zetta 21, Exa 18, Peta 15, Tera 12, Giga 9, Mega 6,
            Kilo 3, Hecto 2, Deca 1, Deci -1, Centi -2, Milli -3, Micro -6, Nano -9, Pico -12,
            Femto -15, Atto -18, Zepto -21, Yocto -24, Ronto -27, Quecto -30);
    };
    (@each $dimension:ty, $root:ty, $root_exponent:expr; $($prefix:ident $exponent:literal),+) => {
        $(
            assert_prefixed_scale::<
                $dimension,
                $root,
                Prefixed<aequitas::unit::prefixes::$prefix, $root>,
            >($root_exponent, $exponent);
        )+
    };
}

#[test]
fn every_prefix_of_a_decimal_root_is_the_nearest_double_to_its_decade() {
    assert_every_decimal_prefix!(Mass, Gram, -3);
    assert_every_decimal_prefix!(Mass, Tonne, 3);
    assert_every_decimal_prefix!(Volume, Liter, -3);
    assert_every_decimal_prefix!(Volume, LiterLowercase, -3);
    assert_every_decimal_prefix!(Pressure, Bar, 5);
    assert_every_decimal_prefix!(Acceleration, Gal, -2);
    assert_every_decimal_prefix!(Area, Barn, -28);
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
