//! The exponent algebra, and its refusals.
//!
//! Every operation here reproduces a law-crate derivative -- `MultiplyDimension`,
//! `DivideDimension`, the root impls, `powi` -- so the arithmetic and the
//! errors it can raise are one contract: an exponent vector is only useful if
//! it is exact or rejected, never truncated.

use aequitas::systems::si::dimensions;

use crate::tag::{DimensionTag, SemanticTag, TaggedDimension};

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
