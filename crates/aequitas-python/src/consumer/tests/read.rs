//! The structural read, and the order it reads in.
//!
//! A foreign object is accepted on what it *carries*, never on what it is, so
//! the read has to hold three properties at once: a malformed tag is a typed
//! error rather than a fallback, the protocol outranks `__float__`, and an
//! object that is not a quantity at all names what it is missing. The last two
//! are regressions this leaf exists to keep closed.

use aequitas::systems::si::dimensions;
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;

use super::fixtures::build;
use crate::consumer::{Dimensioned, read};

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
fn read_rejects_a_malformed_tag() {
    let cases = [
        ("((1, 0, 0), 'base')", "exactly 7"),
        ("((1, 0, 0, 0, 0, 0, 0), 'vibes')", "vibes"),
        ("'not a tuple'", "must be"),
        // The exponent count is read by hand rather than by extracting a
        // vector, so both directions of the arity check are pinned: the count
        // reported has to be the one the tag actually carried.
        (
            "((1, 0, 0, 0, 0, 0, 0, 0, 0), 'base')",
            "exactly 7 exponents, got 9",
        ),
        ("((1, 0, 0, 0, 0, 0, 'x'), 'base')", "must be"),
        ("((200, 0, 0, 0, 0, 0, 0), 'base')", "out of range"),
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
