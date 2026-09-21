//! The finiteness policy at the consumer boundary.
//!
//! Strict by default, with an explicit opt-in where a parameter publishes
//! infinity as a sentinel. `NaN` is the absence of a value and is never
//! admitted -- including where infinity is -- and the check applies to the
//! protocol arm as well as the float arm, so a quantity carrying a `NaN`
//! cannot slip past a rule its bare-number twin would fail.

use aequitas::systems::si::dimensions;
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;

use super::fixtures::build;
use crate::consumer::{Dimensioned, MayBeInfinite};

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
            .extract::<Dimensioned<dimensions::Length, MayBeInfinite>>()
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
                .extract::<Dimensioned<dimensions::Length, MayBeInfinite>>()
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
