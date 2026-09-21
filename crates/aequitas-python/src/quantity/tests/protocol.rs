//! The duck-typed cross-extension protocol.
//!
//! A consumer built against another `aequitas-python` version interoperates
//! through the tag tuple rather than a downcast, so these exercise foreign
//! objects: conforming, malformed, and absent.

use super::fixtures::{assert_exact, length};
use crate::quantity::{PyQuantity, extract_quantity};
use crate::tag::TaggedDimension;
use aequitas::systems::si::dimensions;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[test]
fn the_protocol_tuple_carries_exponents_and_semantics() {
    Python::attach(|py| {
        let stress = PyQuantity::from_base(1.0, <dimensions::Stress as TaggedDimension>::TAG);
        let tuple = stress.dimension(py).expect("tuple renders");
        let (exponents, semantics): (Vec<i64>, String) = tuple.extract().expect("shape");

        assert_eq!(exponents, vec![-1, 1, -2, 0, 0, 0, 0]);
        assert_eq!(semantics, "stress");
    });
}

#[test]
fn a_duck_typed_object_is_accepted_without_the_pyclass() {
    // The interop path: an object from another extension module, or plain
    // Python, that merely exposes the protocol attributes.
    let setup = r#"
class Foreign:
    __aequitas_base__ = 2.5
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), "base")

result = Foreign()
"#;
    Python::attach(|py| {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(setup).expect("no interior nul"),
            Some(&globals),
            None,
        )
        .expect("class defines");
        let foreign = globals.get_item("result").expect("lookup").expect("bound");

        let extracted = extract_quantity(&foreign).expect("the protocol is satisfied");
        assert_exact(extracted.base_value(), 2.5);
        assert_eq!(
            extracted.tag(),
            <dimensions::Length as TaggedDimension>::TAG
        );
    });
}

#[test]
fn a_duck_typed_object_participates_in_arithmetic() {
    let setup = r#"
class Foreign:
    __aequitas_base__ = 4.0
    __aequitas_dimension__ = ((0, 0, 1, 0, 0, 0, 0), "base")

result = Foreign()
"#;
    Python::attach(|py| {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(setup).expect("no interior nul"),
            Some(&globals),
            None,
        )
        .expect("class defines");
        let foreign = globals.get_item("result").expect("lookup").expect("bound");

        let velocity = length(8.0)
            .__truediv__(&foreign)
            .expect("a foreign time divides a native length");
        assert_exact(velocity.base_value(), 2.0);
        assert_eq!(
            velocity.tag(),
            <dimensions::Velocity as TaggedDimension>::TAG
        );
    });
}

#[test]
fn a_malformed_tag_is_rejected_with_its_reason() {
    let setup = r#"
class TooShort:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((1, 0, 0), "base")

class UnknownSemantics:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), "vibes")

result = (TooShort(), UnknownSemantics())
"#;
    Python::attach(|py| {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(setup).expect("no interior nul"),
            Some(&globals),
            None,
        )
        .expect("classes define");
        let pair = globals.get_item("result").expect("lookup").expect("bound");

        let short = pair.get_item(0).expect("first");
        let short_error = extract_quantity(&short).expect_err("seven exponents required");
        assert!(
            short_error.to_string().contains("exactly 7"),
            "{short_error}"
        );

        let unknown = pair.get_item(1).expect("second");
        let unknown_error = extract_quantity(&unknown).expect_err("unknown marker");
        assert!(
            unknown_error.to_string().contains("vibes"),
            "{unknown_error}"
        );
    });
}

#[test]
fn a_non_conforming_object_names_the_missing_attributes() {
    Python::attach(|py| {
        let text = "not a quantity".into_pyobject(py).expect("bind");
        let error = extract_quantity(&text).expect_err("a string is not a quantity");
        assert!(error.to_string().contains("__aequitas_base__"), "{error}");
        assert!(error.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
    });
}

/// A protocol object that also defines `__float__` keeps its dimension.
///
/// Reading it as a scalar kept the magnitude and dropped the dimension, so
/// `length * time` came back a length with no error raised.
#[test]
fn a_protocol_object_with_float_keeps_its_dimension_in_arithmetic() {
    let setup = r#"
class ForeignTime:
    __aequitas_base__ = 4.0
    __aequitas_dimension__ = ((0, 0, 1, 0, 0, 0, 0), "base")

    def __float__(self):
        return 4.0

result = ForeignTime()
"#;
    Python::attach(|py| {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(setup).expect("no interior nul"),
            Some(&globals),
            None,
        )
        .expect("class defines");
        let foreign = globals.get_item("result").expect("lookup").expect("bound");
        let length_times_time = <dimensions::Length as TaggedDimension>::TAG
            .multiply(<dimensions::Time as TaggedDimension>::TAG)
            .expect("in range");

        let product = length(2.0)
            .__mul__(&foreign)
            .expect("length * time is defined");
        assert_eq!(product.tag(), length_times_time);
        assert_exact(product.base_value(), 8.0);

        let reflected = length(2.0)
            .__rmul__(&foreign)
            .expect("time * length is defined");
        assert_eq!(reflected.tag(), length_times_time);

        let speed = length(2.0)
            .__truediv__(&foreign)
            .expect("length / time is defined");
        assert_eq!(speed.tag(), <dimensions::Velocity as TaggedDimension>::TAG);
        assert_exact(speed.base_value(), 0.5);
    });
}
