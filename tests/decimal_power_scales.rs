//! A power of a decimal unit carries the nearest binary64 to its exact decimal scale.
//!
//! The oracle is Rust's float parser, which rounds a decimal literal to
//! nearest and shares no code with the scale computation under test.

use aequitas::{
    systems::si::{
        dimensions::{Angle, Area, Volume},
        units::{
            Centimeter, CubicMillimeter, Degree, Kilometer, Meter, Millimeter, SquareCentimeter,
        },
    },
    unit::{LinearUnit, Power, Prefixed, Unit, prefixes::Deci},
};
use typenum::{N2, P2, P3};

fn nearest(decimal: &str) -> u64 {
    decimal
        .parse::<f64>()
        .expect("invariant: the literal is a valid float")
        .to_bits()
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
fn units_that_predate_the_shared_mechanism_keep_their_literal_values() {
    assert_eq!(
        <SquareCentimeter as LinearUnit<Area>>::SCALE.to_bits(),
        1.0e-4_f64.to_bits()
    );
    assert_eq!(
        <CubicMillimeter as LinearUnit<Volume>>::SCALE.to_bits(),
        1.0e-9_f64.to_bits()
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
