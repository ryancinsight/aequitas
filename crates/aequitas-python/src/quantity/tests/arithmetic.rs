//! Dimensional algebra through the `#[pymethods]` operators.
//!
//! Each operation has a type-level counterpart in Aequitas; what `rustc`
//! rejects at compile time these assert at call time, value-semantically.

use super::fixtures::{assert_exact, length, time};
use crate::quantity::PyQuantity;
use crate::tag::{DimensionTag, SemanticTag, TaggedDimension};
use aequitas::systems::si::dimensions;
use aequitas::systems::si::quantities::{Angle, Dimensionless, Pressure, Stress};
use pyo3::prelude::*;

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

/// Scaling by a bare number keeps the dimension, marker included.
///
/// The `let _: Stress<f64>` lines are the oracle: they compile only because
/// the law crate's `Mul<T>`, reflected `Mul`, and `Div<T>` for `Quantity<T, D>`
/// return the same `D`. The binding normalized the marker instead, so a stress
/// scaled by two came back a pressure and no longer added to a stress.
#[test]
fn scaling_by_a_number_keeps_the_semantic_marker() {
    let _: Stress<f64> = Stress::<f64>::from_base(1.0) * 2.0;
    let _: Stress<f64> = 2.0 * Stress::<f64>::from_base(1.0);
    let _: Stress<f64> = Stress::<f64>::from_base(1.0) / 2.0;
    let _: Angle<f64> = Angle::<f64>::from_base(1.0) * 2.0;

    let stress = <dimensions::Stress as TaggedDimension>::TAG;
    let angle = <dimensions::Angle as TaggedDimension>::TAG;
    Python::attach(|py| {
        let two = 2.0_f64.into_pyobject(py).expect("bind");

        let scaled = PyQuantity::from_base(1.0, stress)
            .__mul__(&two)
            .expect("stress * 2 is defined");
        assert_eq!(scaled.tag(), stress);
        assert_exact(scaled.base_value(), 2.0);

        let reflected = PyQuantity::from_base(1.0, stress)
            .__rmul__(&two)
            .expect("2 * stress is defined");
        assert_eq!(reflected.tag(), stress);

        let halved = PyQuantity::from_base(1.0, stress)
            .__truediv__(&two)
            .expect("stress / 2 is defined");
        assert_eq!(halved.tag(), stress);
        assert_exact(halved.base_value(), 0.5);

        let turned = PyQuantity::from_base(1.0, angle)
            .__mul__(&two)
            .expect("angle * 2 is defined");
        assert_eq!(turned.tag(), angle);
    });
}

/// A dimensionless *quantity* is not a bare number: multiplying by one combines
/// dimensions and normalizes the marker, as `MultiplyDimension` does.
#[test]
fn a_dimensionless_quantity_operand_normalizes_the_marker() {
    let _: Pressure<f64> = Stress::<f64>::from_base(1.0) * Dimensionless::<f64>::from_base(2.0);

    Python::attach(|py| {
        let factor = PyQuantity::from_base(2.0, DimensionTag::DIMENSIONLESS)
            .into_pyobject(py)
            .expect("bind");
        let product = PyQuantity::from_base(1.0, <dimensions::Stress as TaggedDimension>::TAG)
            .__mul__(&factor)
            .expect("stress * dimensionless is defined");
        assert_eq!(
            product.tag(),
            <dimensions::Pressure as TaggedDimension>::TAG
        );
    });
}
