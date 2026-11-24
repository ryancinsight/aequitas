//! Constructing a quantity from a unit, and reading it back.
//!
//! Unit resolution is scoped to the named quantity, so these also cover
//! the refusals: an unknown quantity, an unknown unit, and a unit that
//! belongs to another dimension.

use super::fixtures::assert_exact;
use crate::quantity::PyQuantity;
use crate::tag::TaggedDimension;
use crate::units;
use aequitas::systems::si::dimensions;
use pyo3::prelude::*;

#[test]
fn from_unit_scales_into_base_units() {
    let millimeters = PyQuantity::from_unit(12.5, "mm", "length").expect("mm resolves");
    assert_exact(millimeters.base_value(), 0.012_5);
    assert_eq!(
        millimeters.tag(),
        <dimensions::Length as TaggedDimension>::TAG
    );
}

#[test]
fn in_unit_inverts_from_unit() {
    let value = 12.5;
    let quantity = PyQuantity::from_unit(value, "mm", "length").expect("mm resolves");
    let round_tripped = quantity.in_unit("mm").expect("mm resolves");
    assert_exact(round_tripped, value);
}

#[test]
fn an_unknown_quantity_is_a_key_error() {
    let error = PyQuantity::from_unit(1.0, "m", "wingspan").expect_err("no such quantity");
    Python::attach(|py| {
        assert!(error.is_instance_of::<pyo3::exceptions::PyKeyError>(py));
    });
}

#[test]
fn an_unknown_unit_lists_the_known_ones() {
    let error = PyQuantity::from_unit(1.0, "furlong", "length").expect_err("no such unit");
    let message = error.to_string();
    assert!(message.contains("furlong"), "{message}");
    assert!(message.contains("mm"), "{message}");
}

#[test]
fn a_unit_of_another_dimension_does_not_resolve() {
    let error = PyQuantity::from_unit(1.0, "Pa", "length").expect_err("Pa is not a length");
    assert!(error.to_string().contains("Pa"));
}

#[test]
fn the_module_exposes_every_registered_quantity() {
    let names = crate::units::all();
    assert!(
        names
            .iter()
            .any(|quantity| quantity.name == "acoustic_impedance")
    );
    assert!(units::by_name("mass_density").is_some());
}

/// `degC` resolves per quantity through the `#[pymethods]` surface.
#[test]
fn celsius_resolves_per_quantity_through_the_python_surface() {
    let temperature = PyQuantity::from_unit(0.0, "degC", "thermodynamic_temperature")
        .expect("affine degC resolves for a temperature");
    let difference = PyQuantity::from_unit(10.0, "degC", "temperature_difference")
        .expect("linear degC resolves for a difference");

    assert_exact(temperature.base_value(), 273.15);
    assert_exact(difference.base_value(), 10.0);
    assert_exact(temperature.in_unit("degC").expect("degC reads back"), 0.0);
}

#[test]
fn metric_power_notation_resolves_in_both_directions() {
    let cases = [
        ("area", "mm²", "m²", 1e-6),
        ("area", "dm²", "m²", 1e-2),
        ("area", "dam²", "a", 1.0),
        ("area", "hm²", "ha", 1.0),
        ("area", "km²", "m²", 1e6),
        ("volume", "cm³", "mL", 1.0),
        ("volume", "dm³", "L", 1.0),
    ];
    for (quantity_name, symbol, reference, expected) in cases {
        let q = PyQuantity::from_unit(1.0, symbol, quantity_name)
            .unwrap_or_else(|error| panic!("{quantity_name}/{symbol} must construct: {error}"));
        assert_exact(
            q.in_unit(reference).expect("reference unit resolves"),
            expected,
        );
        let q = PyQuantity::from_unit(expected, reference, quantity_name)
            .expect("reference unit constructs");
        assert_exact(
            q.in_unit(symbol).expect("metric power symbol reads back"),
            1.0,
        );
    }
}

/// Assert `symbol` is refused for `quantity` with a `ValueError` naming both.
#[track_caller]
fn assert_unknown_unit(symbol: &str, quantity: &str) {
    let error = PyQuantity::from_unit(1.0, symbol, quantity).expect_err("spelling is refused");
    Python::attach(|py| {
        assert!(error.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
    let message = error.to_string();
    let prefix = format!("ValueError: unknown unit '{symbol}' for {quantity};");
    assert!(message.starts_with(&prefix), "{message}");
}

#[test]
fn metric_power_symbols_belong_only_to_their_dimension() {
    assert_unknown_unit("cm³", "area");
    assert_unknown_unit("dm²", "volume");
}

#[test]
fn spellings_outside_the_accepted_set_are_refused() {
    assert_unknown_unit("km³", "volume");
    assert_unknown_unit("dam³", "volume");
    assert_unknown_unit("hm³", "volume");
    assert_unknown_unit("cc", "volume");
    assert_unknown_unit("\u{2109}", "thermodynamic_temperature");
    assert_unknown_unit("\u{3bc}m", "length");
}

/// Absolute error bound for reading a temperature back through kelvin.
///
/// Reading `v` degrees Fahrenheit computes `base = fl(fl(v·s) + o)` and
/// `v' = fl(fl(base − o) / s)` with the stored scale `s = 5/9` and offset
/// `o`, the same stored values in both directions, so their representation
/// errors cancel. To first order in `ε = f64::EPSILON`, for `v = 32`:
/// `v·s ≈ 17.8` lies in `[16, 32)`, whose half-ulp is `8ε`; `base ≈ 273.15`
/// lies in `[256, 512)`, whose half-ulp is `128ε`; `base − o ≈ 17.8` is exact
/// by Sterbenz's lemma, since `o ≈ 255.4` is within a factor of two of `base`.
/// Division by `s` amplifies the base-side error by `1/s = 9/5`, and the
/// result `≈ 32` lies in `[32, 64)`, whose half-ulp is `16ε`.
const FAHRENHEIT_ROUND_TRIP_BOUND: f64 = ((128.0 + 8.0) * (9.0 / 5.0) + 16.0) * f64::EPSILON;

#[test]
fn unicode_fahrenheit_retains_absolute_and_interval_behavior() {
    let freezing = PyQuantity::from_unit(32.0, "°F", "thermodynamic_temperature")
        .expect("Unicode Fahrenheit resolves for an absolute temperature");
    assert_exact(freezing.in_unit("°C").expect("Celsius reads back"), 0.0);
    let read_back = freezing
        .in_unit("°F")
        .expect("Unicode Fahrenheit reads back");
    assert!(
        (read_back - 32.0).abs() <= FAHRENHEIT_ROUND_TRIP_BOUND,
        "{read_back}"
    );
    let interval = PyQuantity::from_unit(18.0, "°F", "temperature_difference")
        .expect("Unicode Fahrenheit resolves for an interval");
    assert_exact(interval.in_unit("K").expect("kelvin reads back"), 10.0);
    assert_exact(
        interval
            .in_unit("degF")
            .expect("ASCII Fahrenheit still resolves"),
        18.0,
    );
    assert_exact(
        interval
            .in_unit("°F")
            .expect("Unicode Fahrenheit reads back"),
        18.0,
    );
}
