//! Value-semantic tests for the Python-visible quantity.
//!
//! These run against a real interpreter so the `#[pymethods]` surface is
//! exercised as Python sees it, not as Rust would call it.

use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict};

use aequitas::systems::si::dimensions;

use super::{PyQuantity, extract_quantity};
use crate::tag::{DimensionTag, SemanticTag, TaggedDimension};
use crate::units;

/// Assert two `f64` values are bitwise identical.
///
/// The oracle here is exact equality, not a tolerance: the binding performs
/// the same `f64` operation on the same operands that Aequitas performs, so
/// evaluation order provably matches and any difference is a defect rather
/// than rounding: no reordering, no widening, no separate accumulation.
#[expect(
    clippy::float_cmp,
    reason = "bitwise equality is the correct oracle for an identical operation"
)]
#[track_caller]
fn assert_exact(actual: f64, expected: f64) {
    assert_eq!(actual, expected);
}

fn length(meters: f64) -> PyQuantity {
    PyQuantity::from_base(meters, <dimensions::Length as TaggedDimension>::TAG)
}

fn time(seconds: f64) -> PyQuantity {
    PyQuantity::from_base(seconds, <dimensions::Time as TaggedDimension>::TAG)
}

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
fn addition_requires_identical_dimensions() {
    Python::attach(|py| {
        let metre = length(2.0);
        let second = time(3.0).into_pyobject(py).expect("bind");
        let error = metre.__add__(&second).expect_err("m + s is not defined");
        assert!(error.to_string().contains("dimensions differ"), "{error}");
    });
}

#[test]
fn addition_of_like_dimensions_sums_magnitudes() {
    Python::attach(|py| {
        let sum = length(2.0)
            .__add__(&length(3.0).into_pyobject(py).expect("bind"))
            .expect("m + m is defined");
        assert_exact(sum.base_value(), 5.0);
        assert_eq!(sum.tag(), <dimensions::Length as TaggedDimension>::TAG);
    });
}

#[test]
fn multiplication_combines_dimensions() {
    Python::attach(|py| {
        let area = length(3.0)
            .__mul__(&length(4.0).into_pyobject(py).expect("bind"))
            .expect("m * m is defined");
        assert_exact(area.base_value(), 12.0);
        assert_eq!(area.tag(), <dimensions::Area as TaggedDimension>::TAG);
    });
}

#[test]
fn division_yields_the_derived_dimension() {
    Python::attach(|py| {
        let velocity = length(10.0)
            .__truediv__(&time(2.0).into_pyobject(py).expect("bind"))
            .expect("m / s is defined");
        assert_exact(velocity.base_value(), 5.0);
        assert_eq!(
            velocity.tag(),
            <dimensions::Velocity as TaggedDimension>::TAG
        );
        assert_eq!(velocity.quantity(), Some("velocity"));
    });
}

#[test]
fn a_scalar_multiplies_without_changing_the_dimension() {
    Python::attach(|py| {
        let scaled = length(3.0)
            .__mul__(&2.0_f64.into_pyobject(py).expect("bind"))
            .expect("m * scalar is defined");
        assert_exact(scaled.base_value(), 6.0);
        assert_eq!(scaled.tag(), <dimensions::Length as TaggedDimension>::TAG);
    });
}

#[test]
fn a_scalar_divided_by_a_quantity_inverts_the_dimension() {
    Python::attach(|py| {
        let inverted = time(4.0)
            .__rtruediv__(&1.0_f64.into_pyobject(py).expect("bind"))
            .expect("scalar / s is defined");
        assert_exact(inverted.base_value(), 0.25);
        assert_eq!(
            inverted.tag(),
            <dimensions::Frequency as TaggedDimension>::TAG
        );
    });
}

#[test]
fn division_by_zero_raises_rather_than_producing_infinity() {
    Python::attach(|py| {
        let error = length(1.0)
            .__truediv__(&time(0.0).into_pyobject(py).expect("bind"))
            .expect_err("division by zero");
        assert!(error.is_instance_of::<pyo3::exceptions::PyZeroDivisionError>(py));
    });
}

#[test]
fn multiplying_a_stress_by_an_area_normalizes_to_force() {
    Python::attach(|py| {
        let stress = PyQuantity::from_base(2.0, <dimensions::Stress as TaggedDimension>::TAG);
        let area = PyQuantity::from_base(3.0, <dimensions::Area as TaggedDimension>::TAG);
        let force = stress
            .__mul__(&area.into_pyobject(py).expect("bind"))
            .expect("stress * area is defined");

        assert_exact(force.base_value(), 6.0);
        assert_eq!(force.tag().semantics(), SemanticTag::Base);
        assert_eq!(force.tag(), <dimensions::Force as TaggedDimension>::TAG);
    });
}

#[test]
fn a_stress_and_a_pressure_do_not_add() {
    Python::attach(|py| {
        let stress = PyQuantity::from_base(1.0, <dimensions::Stress as TaggedDimension>::TAG);
        let pressure = PyQuantity::from_base(1.0, <dimensions::Pressure as TaggedDimension>::TAG);
        let error = stress
            .__add__(&pressure.into_pyobject(py).expect("bind"))
            .expect_err("stress and pressure are not interchangeable");
        assert!(error.to_string().contains("stress"), "{error}");
    });
}

#[test]
fn an_absolute_temperature_and_a_difference_do_not_add() {
    Python::attach(|py| {
        let absolute = PyQuantity::from_base(
            310.0,
            <dimensions::ThermodynamicTemperature as TaggedDimension>::TAG,
        );
        let difference = PyQuantity::from_base(
            2.0,
            <dimensions::TemperatureDifference as TaggedDimension>::TAG,
        );
        assert!(
            absolute
                .__add__(&difference.into_pyobject(py).expect("bind"))
                .is_err(),
            "an offset scale and an interval scale are not the same dimension"
        );
    });
}

#[test]
fn sqrt_halves_the_dimension_and_rejects_an_odd_one() {
    let area = PyQuantity::from_base(9.0, <dimensions::Area as TaggedDimension>::TAG);
    let side = area.sqrt().expect("area halves exactly");
    assert_exact(side.base_value(), 3.0);
    assert_eq!(side.tag(), <dimensions::Length as TaggedDimension>::TAG);

    assert!(length(4.0).sqrt().is_err(), "L^1 does not halve");
}

#[test]
fn cbrt_preserves_sign() {
    let volume = PyQuantity::from_base(-8.0, <dimensions::Volume as TaggedDimension>::TAG);
    let side = volume.cbrt().expect("volume thirds exactly");
    assert_exact(side.base_value(), -2.0);
    assert_eq!(side.tag(), <dimensions::Length as TaggedDimension>::TAG);
}

#[test]
fn an_integer_power_multiplies_the_dimension() {
    Python::attach(|py| {
        let cube = length(2.0)
            .__pow__(&3_i32.into_pyobject(py).expect("bind"), None)
            .expect("integer powers are defined");
        assert_exact(cube.base_value(), 8.0);
        assert_eq!(cube.tag(), <dimensions::Volume as TaggedDimension>::TAG);
    });
}

#[test]
fn a_fractional_power_is_rejected() {
    Python::attach(|py| {
        let error = length(2.0)
            .__pow__(&0.5_f64.into_pyobject(py).expect("bind"), None)
            .expect_err("fractional powers have no exponent vector");
        assert!(error.to_string().contains("sqrt"), "{error}");
    });
}

#[test]
fn comparison_orders_within_a_dimension_and_refuses_across_one() {
    Python::attach(|py| {
        let shorter = length(1.0);
        let longer = length(2.0).into_pyobject(py).expect("bind");
        let ordered = shorter
            .__richcmp__(&longer, pyo3::basic::CompareOp::Lt)
            .expect("lengths order");
        assert!(ordered.extract::<bool>(py).expect("bool"));

        let second = time(2.0).into_pyobject(py).expect("bind");
        assert!(
            shorter
                .__richcmp__(&second, pyo3::basic::CompareOp::Lt)
                .is_err(),
            "a length does not order against a time"
        );
    });
}

#[test]
fn equality_across_dimensions_is_false_not_an_error() {
    Python::attach(|py| {
        let metre = length(1.0);
        let second = time(1.0).into_pyobject(py).expect("bind");
        let equal = metre
            .__richcmp__(&second, pyo3::basic::CompareOp::Eq)
            .expect("equality is total");
        assert!(!equal.extract::<bool>(py).expect("bool"));
    });
}

#[test]
fn equality_against_a_foreign_object_is_false() {
    Python::attach(|py| {
        let metre = length(1.0);
        let text = "not a quantity".into_pyobject(py).expect("bind");
        let equal = metre
            .__richcmp__(&text, pyo3::basic::CompareOp::Eq)
            .expect("equality is total");
        assert!(!equal.extract::<bool>(py).expect("bool"));
    });
}

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

#[test]
fn equal_quantities_hash_equally() {
    let one = length(1.5);
    let other = length(1.5);
    assert_eq!(one.__hash__(), other.__hash__());

    // Signed zero compares equal, so it must hash equally too.
    assert_eq!(length(0.0).__hash__(), length(-0.0).__hash__());

    // A different dimension with the same magnitude should not collide.
    assert_ne!(length(1.5).__hash__(), time(1.5).__hash__());
}

#[test]
fn repr_names_the_quantity_and_str_names_the_dimension() {
    let metre = length(2.0);
    assert_eq!(metre.__repr__(), "Quantity(2, 'length')");
    assert_eq!(metre.__str__(), "2 m");
}

#[test]
fn an_unnamed_derived_dimension_still_renders() {
    // L^5 has no alias; the repr falls back to the unit algebra rather than
    // failing or claiming a name it does not have.
    let odd = PyQuantity::from_base(
        1.0,
        DimensionTag::new([5, 0, 0, 0, 0, 0, 0], SemanticTag::Base),
    );
    assert_eq!(odd.quantity(), None);
    assert!(odd.__repr__().contains("m^5"), "{}", odd.__repr__());
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
