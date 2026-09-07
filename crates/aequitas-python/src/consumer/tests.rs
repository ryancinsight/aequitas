//! The consumer extractor, exercised as a consumer would meet it.

use aequitas::systems::si::dimensions;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict};

use super::{Dimensioned, read};
use crate::tag::TaggedDimension;

/// Bind a Python object built by `source`, whose last statement assigns
/// `result`.
fn build<'py>(py: Python<'py>, source: &str) -> Bound<'py, PyAny> {
    let globals = PyDict::new(py);
    py.run(
        &std::ffi::CString::new(source).expect("no interior nul"),
        Some(&globals),
        None,
    )
    .expect("the fixture defines");
    globals
        .get_item("result")
        .expect("lookup")
        .expect("`result` is assigned")
}

const A_LENGTH: &str = r"
class Foreign:
    __aequitas_base__ = 0.0025
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), 'base')

result = Foreign()
";

const A_TIME: &str = r"
class Foreign:
    __aequitas_base__ = 2.0
    __aequitas_dimension__ = ((0, 0, 1, 0, 0, 0, 0), 'base')

result = Foreign()
";

const A_STRESS: &str = r"
class Foreign:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((-1, 1, -2, 0, 0, 0, 0), 'stress')

result = Foreign()
";

const A_PRESSURE: &str = r"
class Foreign:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((-1, 1, -2, 0, 0, 0, 0), 'base')

result = Foreign()
";

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
fn a_non_quantity_names_the_attributes_it_lacks() {
    Python::attach(|py| {
        let text = "not a quantity".into_pyobject(py).expect("bind");
        let error = text
            .extract::<Dimensioned<dimensions::Length>>()
            .expect_err("a string is not a quantity");
        assert!(error.to_string().contains("__aequitas_base__"), "{error}");
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

#[test]
fn read_rejects_a_malformed_tag() {
    let cases = [
        ("((1, 0, 0), 'base')", "exactly 7"),
        ("((1, 0, 0, 0, 0, 0, 0), 'vibes')", "vibes"),
        ("'not a tuple'", "must be"),
    ];
    Python::attach(|py| {
        for (tag, expected) in cases {
            let source = format!(
                "class Foreign:\n    __aequitas_base__ = 1.0\n    __aequitas_dimension__ = {tag}\n\nresult = Foreign()\n"
            );
            let object = build(py, &source);
            let error = read(&object).expect_err("the tag is malformed");
            assert!(error.to_string().contains(expected), "tag {tag}: {error}");
        }
    });
}
