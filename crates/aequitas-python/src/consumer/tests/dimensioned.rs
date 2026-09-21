//! `Dimensioned<D>` as a consumer's parameter type.
//!
//! The whole point of the extractor is that a call site changes from `f64` to
//! `Dimensioned<D>` and nothing else, so these cover both halves of that
//! promise: what was always accepted still is, and what arrives as a quantity
//! is checked against `D` before any computation sees it.

use aequitas::systems::si::dimensions;
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;

use super::fixtures::{A_LENGTH, A_PRESSURE, A_STRESS, A_TIME, build};
use crate::consumer::Dimensioned;
use crate::tag::TaggedDimension;

#[test]
fn a_bare_float_keeps_its_base_unit_meaning() {
    // The already-published contract: an `f64` parameter was canonical SI base
    // units, and stays so. No consumer call site breaks.
    Python::attach(|py| {
        let value = 0.0025_f64.into_pyobject(py).expect("bind");
        let radius: Dimensioned<dimensions::Length> = value.extract().expect("a float extracts");
        assert!((radius.base() - 0.0025).abs() < f64::EPSILON);
    });
}

#[test]
fn a_bare_int_is_accepted_as_a_float() {
    Python::attach(|py| {
        let value = 3_i64.into_pyobject(py).expect("bind");
        let count: Dimensioned<dimensions::Length> = value.extract().expect("an int extracts");
        assert!((count.base() - 3.0).abs() < f64::EPSILON);
    });
}

#[test]
fn a_matching_quantity_is_accepted() {
    Python::attach(|py| {
        let object = build(py, A_LENGTH);
        let radius: Dimensioned<dimensions::Length> =
            object.extract().expect("a length extracts as a length");
        assert!((radius.base() - 0.0025).abs() < f64::EPSILON);
    });
}

#[test]
fn a_mismatched_quantity_is_rejected_with_both_dimensions() {
    Python::attach(|py| {
        let object = build(py, A_TIME);
        let error = object
            .extract::<Dimensioned<dimensions::Length>>()
            .expect_err("a time is not a length");
        let message = error.to_string();
        assert!(message.contains("expected a quantity of `m`"), "{message}");
        assert!(message.contains("got `s`"), "{message}");
    });
}

#[test]
fn a_stress_is_not_accepted_where_a_pressure_belongs() {
    // Identical exponents; the semantic marker is the whole difference, and it
    // must survive the trip through the protocol.
    Python::attach(|py| {
        let stress = build(py, A_STRESS);
        assert!(
            stress
                .extract::<Dimensioned<dimensions::Pressure>>()
                .is_err(),
            "a stress must not satisfy a pressure parameter"
        );

        let pressure = build(py, A_PRESSURE);
        assert!(
            pressure
                .extract::<Dimensioned<dimensions::Pressure>>()
                .is_ok(),
            "a pressure must satisfy a pressure parameter"
        );
        assert!(
            pressure
                .extract::<Dimensioned<dimensions::Stress>>()
                .is_err(),
            "a pressure must not satisfy a stress parameter"
        );
    });
}

#[test]
fn a_native_quantity_is_accepted_through_the_same_path() {
    Python::attach(|py| {
        let native =
            crate::PyQuantity::from_base(0.5, <dimensions::Length as TaggedDimension>::TAG)
                .into_pyobject(py)
                .expect("bind");

        let radius: Dimensioned<dimensions::Length> =
            native.extract().expect("the native class conforms too");
        assert!((radius.base() - 0.5).abs() < f64::EPSILON);
    });
}

#[test]
fn the_extractor_yields_a_typed_quantity() {
    Python::attach(|py| {
        let object = build(py, A_LENGTH);
        let radius: Dimensioned<dimensions::Length> = object.extract().expect("extracts");
        let typed: aequitas::systems::si::quantities::Length<f64> = radius.quantity();
        // Round-trips through the law crate's own unit conversion.
        assert!((typed.in_unit::<aequitas::systems::si::units::Millimeter>() - 2.5).abs() < 1e-12);
    });
}
