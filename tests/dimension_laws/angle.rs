//! Angle-unit identities for the SI dimension system.

use aequitas::systems::si::{quantities::Angle, units::Degree, units::Radian};

#[test]
fn angle_has_a_distinct_radian_semantic_contract() {
    let angle = Angle::from_unit::<Radian>(core::f64::consts::FRAC_PI_2);

    assert_eq!(
        angle.in_unit::<Radian>().to_bits(),
        core::f64::consts::FRAC_PI_2.to_bits()
    );
}

/// Degree and radian are two units of one dimension, so a quantity built in
/// either reads back correctly in the other. The reference values are the
/// defining ratio, not a previous run: a straight angle is exactly `pi`
/// radians and a right angle exactly `FRAC_PI_2`.
#[test]
fn degree_and_radian_are_the_same_angle_in_two_units() {
    let straight = Angle::from_unit::<Degree>(180.0);
    assert!(
        (straight.in_unit::<Radian>() - core::f64::consts::PI).abs() <= 4.0 * f64::EPSILON,
        "180 deg read back as {} rad",
        straight.in_unit::<Radian>()
    );

    let right = Angle::from_unit::<Radian>(core::f64::consts::FRAC_PI_2);
    assert!(
        (right.in_unit::<Degree>() - 90.0).abs() <= 4.0 * f64::EPSILON * 90.0,
        "FRAC_PI_2 read back as {} deg",
        right.in_unit::<Degree>()
    );
}

/// The conversion is a pure scale, so it preserves sign and zero exactly.
///
/// `Angle` is deliberately not additive — `Add` is bound on
/// `BaseAdditiveDimension`, which `AngleSemantics` does not implement — so this
/// checks the conversion itself rather than arithmetic the dimension withholds.
#[test]
fn degree_conversion_preserves_sign_and_zero() {
    let negative = Angle::from_unit::<Degree>(-45.0);
    assert!(
        (negative.in_unit::<Radian>() + core::f64::consts::FRAC_PI_4).abs() <= 4.0 * f64::EPSILON,
        "-45 deg must be -pi/4 rad, got {}",
        negative.in_unit::<Radian>()
    );

    assert_eq!(
        Angle::from_unit::<Degree>(0.0_f64)
            .in_unit::<Radian>()
            .to_bits(),
        0.0_f64.to_bits(),
        "zero must convert without introducing a signed zero"
    );
}
