//! Value-semantic tests for the runtime dimension tag.

use aequitas::systems::si::dimensions;

use super::{DimensionTag, SemanticTag, TaggedDimension};

#[test]
fn base_dimensions_carry_their_own_axis() {
    assert_eq!(
        <dimensions::Length as TaggedDimension>::TAG.exponents(),
        [1, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Mass as TaggedDimension>::TAG.exponents(),
        [0, 1, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Time as TaggedDimension>::TAG.exponents(),
        [0, 0, 1, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::LuminousIntensity as TaggedDimension>::TAG.exponents(),
        [0, 0, 0, 0, 0, 0, 1]
    );
}

#[test]
fn negative_exponents_survive_derivation() {
    // Pressure is M L^-1 T^-2; a sign lost in `typenum`'s `I8` would show here.
    assert_eq!(
        <dimensions::Pressure as TaggedDimension>::TAG.exponents(),
        [-1, 1, -2, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Capacitance as TaggedDimension>::TAG.exponents(),
        [-2, -1, 4, 2, 0, 0, 0]
    );
}

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

#[test]
fn multiplication_adds_exponents_and_normalizes_semantics() {
    let stress = <dimensions::Stress as TaggedDimension>::TAG;
    let area = <dimensions::Area as TaggedDimension>::TAG;
    let force = <dimensions::Force as TaggedDimension>::TAG;

    let product = stress
        .multiply(area)
        .expect("invariant: exponents are small");
    assert_eq!(product.exponents(), force.exponents());
    assert_eq!(
        product.semantics(),
        SemanticTag::Base,
        "a product normalizes the marker, as `MultiplyDimension` does"
    );
    assert_eq!(product, force);
}

#[test]
fn division_subtracts_exponents() {
    let conductivity = <dimensions::ThermalConductivity as TaggedDimension>::TAG;
    let density = <dimensions::MassDensity as TaggedDimension>::TAG;
    let heat_capacity = <dimensions::SpecificHeatCapacity as TaggedDimension>::TAG;
    let diffusivity = <dimensions::ThermalDiffusivity as TaggedDimension>::TAG;

    let denominator = density
        .multiply(heat_capacity)
        .expect("invariant: exponents are small");
    let derived = conductivity
        .divide(denominator)
        .expect("invariant: exponents are small");

    assert_eq!(derived, diffusivity);
}

#[test]
fn velocity_is_length_over_time() {
    let length = <dimensions::Length as TaggedDimension>::TAG;
    let time = <dimensions::Time as TaggedDimension>::TAG;
    let velocity = <dimensions::Velocity as TaggedDimension>::TAG;

    assert_eq!(
        length.divide(time).expect("invariant: exponents are small"),
        velocity
    );
}

#[test]
fn a_square_root_halves_every_exponent() {
    let area = <dimensions::Area as TaggedDimension>::TAG;
    let length = <dimensions::Length as TaggedDimension>::TAG;

    assert_eq!(area.root(2).expect("area halves exactly"), length);
}

#[test]
fn a_cube_root_thirds_every_exponent() {
    let volume = <dimensions::Volume as TaggedDimension>::TAG;
    let length = <dimensions::Length as TaggedDimension>::TAG;

    assert_eq!(volume.root(3).expect("volume thirds exactly"), length);
}

#[test]
fn an_inexact_root_is_rejected_rather_than_truncated() {
    let length = <dimensions::Length as TaggedDimension>::TAG;
    let error = length.root(2).expect_err("L^1 does not halve");

    assert_eq!(error.root, 2);
    assert_eq!(error.axis, 0);
    // The message names the offending axis, not just the failure.
    assert!(error.to_string().contains("not a multiple of 2"), "{error}");
}

#[test]
fn a_zero_root_is_rejected() {
    let length = <dimensions::Length as TaggedDimension>::TAG;
    assert!(length.root(0).is_err());
}

#[test]
fn powers_multiply_exponents() {
    let length = <dimensions::Length as TaggedDimension>::TAG;
    let volume = <dimensions::Volume as TaggedDimension>::TAG;

    assert_eq!(length.powi(3).expect("invariant: 1 * 3 fits i8"), volume);
}

#[test]
fn exponent_overflow_is_reported_not_wrapped() {
    let extreme = DimensionTag::new([100, 0, 0, 0, 0, 0, 0], SemanticTag::Base);
    assert!(extreme.multiply(extreme).is_err());
    assert!(extreme.powi(2).is_err());
}

#[test]
fn reciprocal_negates_exponents() {
    let time = <dimensions::Time as TaggedDimension>::TAG;
    let frequency = <dimensions::Frequency as TaggedDimension>::TAG;

    assert_eq!(
        time.reciprocal().expect("invariant: exponents are small"),
        frequency
    );
}

#[test]
fn semantic_names_are_unique_and_round_trip() {
    let mut names: Vec<&str> = SemanticTag::ALL.iter().map(|tag| tag.name()).collect();
    let count = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), count, "two markers share a wire name");

    for tag in SemanticTag::ALL {
        let found = SemanticTag::ALL
            .into_iter()
            .find(|candidate| candidate.name() == tag.name());
        assert_eq!(found, Some(tag));
    }
}

#[test]
fn display_renders_unit_algebra_and_marks_semantics() {
    let pressure = <dimensions::Pressure as TaggedDimension>::TAG;
    assert_eq!(pressure.to_string(), "m^-1 kg s^-2");

    let stress = <dimensions::Stress as TaggedDimension>::TAG;
    assert_eq!(stress.to_string(), "m^-1 kg s^-2 [stress]");

    let scalar = <dimensions::Dimensionless as TaggedDimension>::TAG;
    assert_eq!(scalar.to_string(), "dimensionless");

    let length = <dimensions::Length as TaggedDimension>::TAG;
    assert_eq!(length.to_string(), "m");
}
