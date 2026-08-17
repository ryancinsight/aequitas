//! Value-semantic checks for dimension-preserving value-only operations
//! (`Quantity::abs` / `Quantity::min` / `Quantity::max`).
//!
//! These verify the zero-cost, trait-free contract of `src/quantity/absminmax.rs`:
//! the dimension type is carried through unchanged (so the result is the same
//! `Quantity<T, D>` type as the operand and can be assigned straight back to
//! the original quantity type), only the scalar value changes, and the scalar
//! behavior matches the native `f64` reference. The same-scalar-type
//! requirement — `min`/`max` accept only a quantity of identical type — is
//! guaranteed by the signature itself, so no `compile_fail` doctests are
//! needed (unlike `powi`/`sqrt`/`cbrt`, which change the dimension).

use aequitas::quantity::{Quantity, UnitDisplay};
use aequitas::systems::si::dimensions::Dimensionless;
use aequitas::systems::si::quantities::{
    Acceleration, Angle, Length as LengthQuantity, Mass as MassQuantity, Time as TimeQuantity,
};
use aequitas::systems::si::units::Meter;

#[test]
fn abs_of_negative_length_is_positive() {
    // |−3 m| = 3 m.
    let depth = LengthQuantity::from_base(-3.0);
    let magnitude = depth.abs();
    let magnitude: LengthQuantity = magnitude;
    assert!((*magnitude.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn abs_of_positive_length_is_identity() {
    // |3 m| = 3 m — the value is unchanged.
    let length = LengthQuantity::from_base(3.0);
    let magnitude: LengthQuantity = length.abs();
    assert!((*magnitude.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn abs_matches_native_f64_abs() {
    // The scalar path is `FloatElement::abs`, which for `f64` is the native
    // intrinsic; -0.0 flips to +0.0 just like `f64::abs`, and NaN stays NaN.
    // Bit-exact comparison: `abs` is a sign-bit operation, so a difference
    // from `f64::abs` would be a real mismatch, and `-0.0`/`NaN` cannot be
    // compared with epsilon margins.
    for value in [-3.5_f64, -0.0, 0.0, 2.5] {
        let quantity = Quantity::<f64, Dimensionless>::from_base(value);
        let abs = quantity.abs();
        assert_eq!(
            abs.as_base().to_bits(),
            value.abs().to_bits(),
            "abs({value}) mismatch"
        );
    }
    let nan = Quantity::<f64, Dimensionless>::from_base(f64::NAN);
    assert!(nan.abs().as_base().is_nan(), "abs(NaN) must be NaN");
}

#[test]
fn abs_preserves_dimension_type() {
    // The result type is identical to the operand type — no dimension change.
    let time = TimeQuantity::from_base(-2.0);
    let magnitude: TimeQuantity = time.abs();
    assert!((*magnitude.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn abs_preserves_semantics_marker() {
    // `Angle::abs` keeps the angle marker (the dimension is untouched), so
    // unlike `sqrt`/`cbrt`/`reciprocal` the result assigns back to an `Angle`.
    let angle = Angle::from_base(-1.5);
    let magnitude: Angle = angle.abs();
    assert!((*magnitude.as_base() - 1.5).abs() < f64::EPSILON);
}

#[test]
fn abs_displays_in_original_unit() {
    // The dimension (and thus the unit display) is unchanged by `abs`.
    let depth = LengthQuantity::from_base(-4.0);
    let magnitude: LengthQuantity = depth.abs();
    let rendered = format!("{}", UnitDisplay::new(&magnitude, Meter));
    assert_eq!(rendered, "4 m");
}

#[test]
fn min_of_same_dimension_quantities() {
    // min(2 m, 5 m) = 2 m.
    let short = LengthQuantity::from_base(2.0);
    let long = LengthQuantity::from_base(5.0);
    let min = short.min(long);
    let min: LengthQuantity = min;
    assert!((*min.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn max_of_same_dimension_quantities() {
    // max(2 m, 5 m) = 5 m.
    let short = LengthQuantity::from_base(2.0);
    let long = LengthQuantity::from_base(5.0);
    let max = short.max(long);
    let max: LengthQuantity = max;
    assert!((*max.as_base() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn min_max_are_commutative() {
    // min/max are symmetric in their operands.
    let a = LengthQuantity::<f64>::from_base(2.0);
    let b = LengthQuantity::<f64>::from_base(5.0);
    assert!((*a.min(b).as_base() - *b.min(a).as_base()).abs() < f64::EPSILON);
    assert!((*a.max(b).as_base() - *b.max(a).as_base()).abs() < f64::EPSILON);
}

#[test]
fn min_max_match_native_f64_reference() {
    let a = LengthQuantity::from_base(2.5);
    let b = LengthQuantity::from_base(-1.5);
    assert!((*a.min(b).as_base() - 2.5_f64.min(-1.5)).abs() < f64::EPSILON);
    assert!((*a.max(b).as_base() - 2.5_f64.max(-1.5)).abs() < f64::EPSILON);
}

#[test]
fn min_max_preserve_dimension_type() {
    // The result type is identical to the operand type — no dimension change.
    let a = TimeQuantity::from_base(1.0);
    let b = TimeQuantity::from_base(2.0);
    let min: TimeQuantity = a.min(b);
    let max: TimeQuantity = a.max(b);
    assert!((*min.as_base() - 1.0).abs() < f64::EPSILON);
    assert!((*max.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn min_max_work_on_composite_dimensions() {
    // The ops are dimension-agnostic: any `Quantity<T, D>` works, here an
    // acceleration. min(−2 m/s², 4 m/s²) = −2 m/s².
    let braking = Acceleration::from_base(-2.0);
    let speeding = Acceleration::from_base(4.0);
    let min: Acceleration = braking.min(speeding);
    let max: Acceleration = braking.max(speeding);
    assert!((*min.as_base() + 2.0).abs() < f64::EPSILON);
    assert!((*max.as_base() - 4.0).abs() < f64::EPSILON);
}

#[test]
fn abs_min_max_roundtrip_through_quantity_arithmetic() {
    // The value-only ops compose with quantity arithmetic: the energy
    // difference |E2 − E1| = |8 J − 3 J| = 5 J, then doubling gives 10 J.
    let e1 = aequitas::systems::si::quantities::Energy::from_base(3.0);
    let e2 = aequitas::systems::si::quantities::Energy::from_base(8.0);
    let delta = (e2 - e1).abs();
    let doubled = 2.0_f64 * delta;
    let doubled: aequitas::systems::si::quantities::Energy = doubled;
    assert!((*doubled.as_base() - 10.0).abs() < f64::EPSILON);
}

#[test]
fn min_max_work_on_dimensionless_quantities() {
    let a = Quantity::<f64, Dimensionless>::from_base(1.0);
    let b = Quantity::<f64, Dimensionless>::from_base(2.0);
    let min: Quantity<f64, Dimensionless> = a.min(b);
    let max: Quantity<f64, Dimensionless> = a.max(b);
    assert!((*min.as_base() - 1.0).abs() < f64::EPSILON);
    assert!((*max.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn min_max_preserve_semantics_marker() {
    // `Angle::min`/`Angle::max` keep the angle marker (the dimension is
    // untouched), so unlike `sqrt`/`cbrt`/`reciprocal` the result assigns
    // back to an `Angle`.
    let a = Angle::from_base(1.0);
    let b = Angle::from_base(2.0);
    let min: Angle = a.min(b);
    let max: Angle = a.max(b);
    assert!((*min.as_base() - 1.0).abs() < f64::EPSILON);
    assert!((*max.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn abs_of_negative_mass_used_in_comparison() {
    // |−2 kg| < 3 kg: min/max operate on the absolute magnitudes.
    let m1 = MassQuantity::from_base(-2.0);
    let m2 = MassQuantity::from_base(3.0);
    let smallest: MassQuantity = m1.abs().min(m2);
    let largest: MassQuantity = m1.abs().max(m2);
    assert!((*smallest.as_base() - 2.0).abs() < f64::EPSILON);
    assert!((*largest.as_base() - 3.0).abs() < f64::EPSILON);
}
