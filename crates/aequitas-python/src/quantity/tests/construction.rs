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
