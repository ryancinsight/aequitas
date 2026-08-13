//! Value-semantic checks for rational-dimension operations (sqrt/cbrt).
//!
//! These verify the beyond-`uom` capability: `sqrt` halves and `cbrt` thirds
//! the type-level dimension while applying the scalar root. Coverage locks
//! the value behavior, unit-aware display, round-trips through quantity
//! arithmetic, and the sign-preserving `cbrt` of negative operands. The
//! negative *type-level* case — odd-exponent dimensions such as `Length::sqrt`
//! and `Time::cbrt` must not compile — is locked by `compile_fail` doctests on
//! `Quantity::sqrt` / `Quantity::cbrt` in `src/quantity/root.rs`.

use aequitas::quantity::{Quantity, UnitDisplay};
use aequitas::systems::si::dimensions::{
    Dimensionless, ReciprocalTemperature, ReciprocalTemperatureSquared, Velocity,
};
use aequitas::systems::si::quantities::{
    Area as AreaQuantity, Energy, Length as LengthQuantity, Mass,
    ReciprocalTime as RecipTimeQuantity, Volume as VolQuantity,
};
use aequitas::systems::si::units::Meter;

#[test]
fn sqrt_of_area_is_length() {
    // sqrt(16 m²) = 4 m.
    let area = AreaQuantity::from_base(16.0);
    let side = area.sqrt();
    // The result must be a length-typed quantity.
    let side: LengthQuantity = side;
    assert!((*side.as_base() - 4.0).abs() < f64::EPSILON);
}

#[test]
fn cbrt_of_volume_is_length() {
    // cbrt(27 m³) = 3 m.
    let volume = VolQuantity::from_base(27.0);
    let side = volume.cbrt();
    let side: LengthQuantity = side;
    assert!((*side.as_base() - 3.0).abs() < 1e-12);
}

#[test]
fn sqrt_composes_with_quantity_arithmetic() {
    // speed = sqrt(2 * kinetic_energy / mass): 2 * 8 J / 4 kg = 4 m²/s² -> 2 m/s.
    let ke = Energy::from_base(8.0);
    let mass = Mass::from_base(4.0);
    let v2 = (2.0_f64 * ke) / mass;
    let v = v2.sqrt();
    let v: Quantity<f64, Velocity> = v;
    assert!((*v.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn sqrt_preserves_value_semantics() {
    // The scalar root matches f64::sqrt exactly.
    let area = AreaQuantity::from_base(25.0);
    let side = area.sqrt();
    assert!((*side.as_base() - 25.0_f64.sqrt()).abs() < f64::EPSILON);
}

#[test]
fn dimension_aliases_are_sqrt_compatible() {
    // ReciprocalTimeSquared -> ReciprocalTime via sqrt.
    let rts =
        Quantity::<f64, aequitas::systems::si::dimensions::ReciprocalTimeSquared>::from_base(4.0);
    let rt = rts.sqrt();
    let rt: RecipTimeQuantity = rt;
    assert!((*rt.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn sqrt_of_acceleration_squared_is_acceleration() {
    // (m/s²)² has length 2 and time -4; sqrt restores the acceleration shape.
    let a2 = Quantity::<
        f64,
        aequitas::dimension::Dimension<
            typenum::P2,
            typenum::Z0,
            typenum::N4,
            typenum::Z0,
            typenum::Z0,
            typenum::Z0,
            typenum::Z0,
        >,
    >::from_base(9.0);
    let acceleration: Quantity<f64, aequitas::systems::si::dimensions::Acceleration> = a2.sqrt();
    assert!((*acceleration.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn sqrt_preserves_unit_display() {
    // The rooted quantity is a length and displays in metres.
    let area = AreaQuantity::from_base(9.0);
    let side: LengthQuantity = area.sqrt();
    let rendered = format!("{}", UnitDisplay::new(&side, Meter));
    assert_eq!(rendered, "3 m");
}

#[test]
fn sqrt_roundtrips_through_quantity_multiplication() {
    let area = AreaQuantity::from_base(16.0);
    let side: LengthQuantity = area.sqrt();
    let area_back: AreaQuantity = side * side;
    assert!((*area_back.as_base() - 16.0).abs() < f64::EPSILON);
}

#[test]
fn cbrt_roundtrips_through_quantity_multiplication() {
    let volume = VolQuantity::from_base(27.0);
    let side: LengthQuantity = volume.cbrt();
    let volume_back: VolQuantity = side * side * side;
    assert!((*volume_back.as_base() - 27.0).abs() < f64::EPSILON);
}

#[test]
fn sqrt_of_reciprocal_temperature_squared() {
    let squared = Quantity::<f64, ReciprocalTemperatureSquared>::from_base(4.0);
    let reciprocal: Quantity<f64, ReciprocalTemperature> = squared.sqrt();
    assert!((*reciprocal.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn dimensionless_sqrt_and_cbrt_preserve_value() {
    let sqrt_value = Quantity::<f64, Dimensionless>::from_base(9.0);
    let rooted: Quantity<f64, Dimensionless> = sqrt_value.sqrt();
    assert!((*rooted.as_base() - 3.0).abs() < f64::EPSILON);

    let cbrt_value = Quantity::<f64, Dimensionless>::from_base(27.0);
    let cubed: Quantity<f64, Dimensionless> = cbrt_value.cbrt();
    assert!((*cubed.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn cbrt_preserves_sign_of_negative_operands() {
    // cbrt(-8 m³) = -2 m via the sign-preserving `FloatElement::cbrt`. The
    // double-precision root is one ulp off -2.0, so assert the sign and
    // magnitude with a loose tolerance rather than exact equality.
    let volume = VolQuantity::from_base(-8.0);
    let side: LengthQuantity = volume.cbrt();
    let value = *side.as_base();
    assert!(value.is_sign_negative(), "cbrt(-8) must be negative");
    assert!((value.abs() - 2.0).abs() < 1e-12, "|cbrt(-8)| must be 2");
}

#[test]
fn sqrt_of_angle_normalizes_to_dimensionless() {
    use aequitas::systems::si::quantities::{Angle, Dimensionless as DimensionlessQuantity};
    // sqrt(9 rad) = 3 — the angle semantic marker normalizes away, so the
    // result is a plain dimensionless quantity, not an angle.
    let angle = Angle::from_base(9.0);
    let rooted: DimensionlessQuantity = angle.sqrt();
    assert!((*rooted.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn cbrt_of_reciprocal_volume_is_reciprocal_length() {
    use aequitas::systems::si::quantities::{ReciprocalLength, ReciprocalVolume};
    // cbrt(27 m⁻³) = 3 m⁻¹ — the reciprocal-volume marker normalizes away,
    // so the result is a plain reciprocal length.
    let reciprocal_volume = ReciprocalVolume::from_base(27.0);
    let reciprocal_length: ReciprocalLength = reciprocal_volume.cbrt();
    assert!((*reciprocal_length.as_base() - 3.0).abs() < f64::EPSILON);
}
