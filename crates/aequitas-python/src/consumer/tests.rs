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

#[test]
fn a_quantity_that_is_also_a_number_is_still_dimension_checked() {
    // The regression this guards: `f64` extraction honours `__float__`, and
    // quantity types define one (pint's does). Reading the number first let
    // such an object through as a bare magnitude with its dimension never
    // examined -- a time accepted where a length belonged, silently.
    let source = r"
class Sneaky:
    __aequitas_base__ = 2.0
    __aequitas_dimension__ = ((0, 0, 1, 0, 0, 0, 0), 'base')

    def __float__(self):
        return 2.0

result = Sneaky()
";
    Python::attach(|py| {
        let object = build(py, source);
        assert!(
            object.extract::<f64>().is_ok(),
            "the fixture must be float-convertible, or it does not test the hole"
        );
        let error = object
            .extract::<Dimensioned<dimensions::Length>>()
            .expect_err("a float-convertible time is still not a length");
        assert!(error.to_string().contains("expected a quantity"), "{error}");
    });
}

#[test]
fn a_float_convertible_quantity_of_the_right_dimension_is_accepted() {
    let source = r"
class Convertible:
    __aequitas_base__ = 0.25
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), 'base')

    def __float__(self):
        return 999.0

result = Convertible()
";
    Python::attach(|py| {
        let object = build(py, source);
        let length: Dimensioned<dimensions::Length> = object.extract().expect("extracts");
        // The protocol value, not `__float__`'s answer.
        assert!((length.base() - 0.25).abs() < f64::EPSILON);
    });
}

#[test]
fn a_malformed_tag_surfaces_rather_than_falling_back_to_the_number() {
    let source = r"
class Broken:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((1, 0, 0), 'base')

    def __float__(self):
        return 1.0

result = Broken()
";
    Python::attach(|py| {
        let object = build(py, source);
        let error = object
            .extract::<Dimensioned<dimensions::Length>>()
            .expect_err("a malformed tag is an error, not a bare number");
        assert!(error.to_string().contains("exactly 7"), "{error}");
    });
}

/// A `NaN` float never reaches a consumer's computation.
///
/// The float arm was documented as unchecked because "there is nothing in a
/// bare number to check". There is: kwavers' `pmut_self_heating` forwarded a
/// drive voltage straight into a multiply and returned a `NaN` power.
#[test]
fn a_nan_float_is_rejected() {
    Python::attach(|py| {
        let value = py
            .eval(
                &std::ffi::CString::new("float('nan')")
                    .expect("invariant: the literal has no interior nul"),
                None,
                None,
            )
            .expect("nan");
        let extracted = value.extract::<Dimensioned<dimensions::Length>>();
        let message = extracted
            .expect_err("a NaN magnitude must not extract")
            .to_string();
        assert!(
            message.contains("finite") && message.contains("NaN"),
            "the rejection must name what is admitted and the offending value, got {message}"
        );
    });
}

/// An infinite float is rejected where the parameter did not ask for it.
#[test]
fn an_infinite_float_is_rejected_by_default() {
    Python::attach(|py| {
        let value = py
            .eval(
                &std::ffi::CString::new("float('inf')")
                    .expect("invariant: the literal has no interior nul"),
                None,
                None,
            )
            .expect("inf");
        assert!(
            value.extract::<Dimensioned<dimensions::Length>>().is_err(),
            "infinity must not extract into a finite-only parameter"
        );
    });
}

/// A parameter that publishes infinity as a sentinel still accepts it.
///
/// kwavers' `set_focus_distance` documents `INF` for "no focusing"; the strict
/// default must not break that contract.
#[test]
fn an_infinite_float_extracts_where_the_sentinel_is_declared() {
    Python::attach(|py| {
        let value = py
            .eval(
                &std::ffi::CString::new("float('inf')")
                    .expect("invariant: the literal has no interior nul"),
                None,
                None,
            )
            .expect("inf");
        let extracted = value
            .extract::<Dimensioned<dimensions::Length, super::MayBeInfinite>>()
            .expect("the sentinel is admitted");
        assert!(
            extracted.base().is_infinite() && extracted.base().is_sign_positive(),
            "the sentinel arrives unchanged, got {}",
            extracted.base()
        );
    });
}

/// `NaN` is not a sentinel, so declaring the sentinel does not admit it.
#[test]
fn a_nan_is_rejected_even_where_infinity_is_admitted() {
    Python::attach(|py| {
        let value = py
            .eval(
                &std::ffi::CString::new("float('nan')")
                    .expect("invariant: the literal has no interior nul"),
                None,
                None,
            )
            .expect("nan");
        assert!(
            value
                .extract::<Dimensioned<dimensions::Length, super::MayBeInfinite>>()
                .is_err(),
            "NaN is the absence of a value, not a sentinel"
        );
    });
}

/// The protocol arm checks the magnitude too, not just the float arm.
#[test]
fn a_non_finite_quantity_is_rejected() {
    Python::attach(|py| {
        let value = build(
            py,
            r"
class Foreign:
    __aequitas_base__ = float('nan')
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), 'base')

result = Foreign()
",
        );
        assert!(
            value.extract::<Dimensioned<dimensions::Length>>().is_err(),
            "a quantity carrying NaN must be rejected like a bare NaN"
        );
    });
}

/// An ordinary magnitude is unaffected.
#[test]
fn a_finite_float_still_extracts() {
    Python::attach(|py| {
        let value = py
            .eval(
                &std::ffi::CString::new("0.0025")
                    .expect("invariant: the literal has no interior nul"),
                None,
                None,
            )
            .expect("float");
        let extracted = value
            .extract::<Dimensioned<dimensions::Length>>()
            .expect("a finite magnitude extracts");
        assert!((extracted.base() - 0.0025).abs() < f64::EPSILON);
    });
}
