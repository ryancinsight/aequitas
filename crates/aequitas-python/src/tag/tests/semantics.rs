//! Dimensionally identical quantities stay apart.
//!
//! Exponents are not identity. `SemanticTag` carries the distinction the
//! exponent vector cannot -- stress from pressure, an absolute temperature
//! from a difference, an angle from a bare scalar -- and these assert it
//! reaches Python intact.

use aequitas::systems::si::dimensions;

use crate::tag::{SemanticTag, TaggedDimension};

#[test]
fn semantics_separate_dimensionally_identical_quantities() {
    let pressure = <dimensions::Pressure as TaggedDimension>::TAG;
    let stress = <dimensions::Stress as TaggedDimension>::TAG;

    assert_eq!(pressure.exponents(), stress.exponents());
    assert_eq!(pressure.semantics(), SemanticTag::Base);
    assert_eq!(stress.semantics(), SemanticTag::Stress);
    assert_ne!(
        pressure, stress,
        "stress and pressure must not unify: the marker is the only difference"
    );
}

#[test]
fn absolute_temperature_differs_from_a_difference() {
    let absolute = <dimensions::ThermodynamicTemperature as TaggedDimension>::TAG;
    let difference = <dimensions::TemperatureDifference as TaggedDimension>::TAG;

    assert_eq!(absolute.exponents(), difference.exponents());
    assert_ne!(absolute, difference);
    assert_eq!(absolute.semantics(), SemanticTag::AbsoluteTemperature);
    assert_eq!(difference.semantics(), SemanticTag::TemperatureDifference);
}

#[test]
fn surface_tension_differs_from_energy_per_area() {
    let tension = <dimensions::SurfaceTension as TaggedDimension>::TAG;
    let per_area = <dimensions::EnergyPerArea as TaggedDimension>::TAG;

    assert_eq!(tension.exponents(), per_area.exponents());
    assert_ne!(tension, per_area);
}

#[test]
fn an_angle_is_dimensionless_but_not_a_bare_scalar() {
    let angle = <dimensions::Angle as TaggedDimension>::TAG;
    let scalar = <dimensions::Dimensionless as TaggedDimension>::TAG;

    assert!(angle.is_dimensionless());
    assert!(scalar.is_dimensionless());
    assert_ne!(angle, scalar);
    assert_eq!(angle.semantics(), SemanticTag::Angle);
}
