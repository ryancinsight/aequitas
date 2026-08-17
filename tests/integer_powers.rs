//! Value-semantic checks for type-level integer powers (`Quantity::powi`).
//!
//! These verify the beyond-`uom` capability: `powi::<P>()` raises every SI
//! dimension exponent at the type level while applying the scalar power, so
//! `Length::powi::<P2>()` is an `Area` and `Time::powi::<N1>()` is a
//! `ReciprocalTime`. `uom`'s `powi` only scales the value at runtime with a
//! hardcoded per-quantity dimension, so this is a strict capability
//! extension. The semantics-marked normalization — `Angle::powi` yields a
//! plain dimensionless result, not `Angle` — is locked by a `compile_fail`
//! doctest on `Quantity::powi` in `src/quantity/pow.rs`.

use aequitas::dimension::Dimension;
use aequitas::quantity::{Quantity, UnitDisplay};
use aequitas::systems::si::dimensions::Dimensionless;
use aequitas::systems::si::quantities::{
    Acceleration, Area as AreaQuantity, Energy, Length as LengthQuantity, ReciprocalLength,
    ReciprocalTime as RecipTimeQuantity, Time as TimeQuantity, Velocity as VelocityQuantity,
    Volume as VolumeQuantity,
};
use aequitas::systems::si::units::SquareMeter;
use typenum::{N1, N2, N3, N4, P1, P2, P3, Z0};

// Raw dimension shapes used by the shape-checking tests. These factor the
// seven-axis `Dimension` type so the tests read the exponent pattern directly.
type ReciprocalVelocityShape = Dimension<N1, Z0, P1, Z0, Z0, Z0, Z0>;
type CubedVelocityShape = Dimension<P3, Z0, N3, Z0, Z0, Z0, Z0>;
type ReciprocalAreaShape = Dimension<N2, Z0, Z0, Z0, Z0, Z0, Z0>;
type SquaredVelocityShape = Dimension<P2, Z0, N2, Z0, Z0, Z0, Z0>;
type SquaredAccelerationShape = Dimension<P2, Z0, N4, Z0, Z0, Z0, Z0>;

#[test]
fn square_of_length_is_area() {
    // (3 m)² = 9 m².
    let length = LengthQuantity::from_base(3.0);
    let area = length.powi::<P2>();
    let area: AreaQuantity = area;
    assert!((*area.as_base() - 9.0).abs() < f64::EPSILON);
}

#[test]
fn cube_of_length_is_volume() {
    // (3 m)³ = 27 m³.
    let length = LengthQuantity::from_base(3.0);
    let volume = length.powi::<P3>();
    let volume: VolumeQuantity = volume;
    assert!((*volume.as_base() - 27.0).abs() < f64::EPSILON);
}

#[test]
fn inverse_of_time_is_reciprocal_time() {
    // (2 s)⁻¹ = 0.5 s⁻¹.
    let time = TimeQuantity::from_base(2.0);
    let reciprocal = time.powi::<N1>();
    let reciprocal: RecipTimeQuantity = reciprocal;
    assert!((*reciprocal.as_base() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn inverse_square_of_time_is_reciprocal_time_squared() {
    let time = TimeQuantity::from_base(2.0);
    let reciprocal_squared = time.powi::<N2>();
    let reciprocal_squared: Quantity<
        f64,
        aequitas::systems::si::dimensions::ReciprocalTimeSquared,
    > = reciprocal_squared;
    assert!((*reciprocal_squared.as_base() - 0.25).abs() < f64::EPSILON);
}

#[test]
fn powi_composes_with_quantity_arithmetic() {
    // Kinetic energy: E = ½ m v². v = 3 m/s, m = 4 kg -> 18 J.
    let velocity = VelocityQuantity::from_base(3.0);
    let v2 = velocity.powi::<P2>();
    let mass = aequitas::systems::si::quantities::Mass::from_base(4.0);
    let ke = 0.5_f64 * mass * v2;
    let ke: Energy = ke;
    assert!((*ke.as_base() - 18.0).abs() < f64::EPSILON);
}

#[test]
fn powi_zero_is_dimensionless() {
    // Any quantity to the 0th power is dimensionless with value 1.
    let length = LengthQuantity::from_base(42.0);
    let one = length.powi::<Z0>();
    let one: Quantity<f64, Dimensionless> = one;
    assert!((*one.as_base() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn powi_roundtrips_through_quantity_arithmetic() {
    // (3 m)² = 9 m², then sqrt -> 3 m.
    let length = LengthQuantity::from_base(3.0);
    let area: AreaQuantity = length.powi::<P2>();
    let back: LengthQuantity = area.sqrt();
    assert!((*back.as_base() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn powi_negative_power_inverts_dimension() {
    // velocity⁻¹ is reciprocal velocity; the time/velocity shape inverts.
    let velocity = VelocityQuantity::from_base(4.0);
    let reciprocal = velocity.powi::<N1>();
    // velocity = length·time⁻¹, so velocity⁻¹ = time·length⁻¹.
    let reciprocal: Quantity<f64, ReciprocalVelocityShape> = reciprocal;
    assert!((*reciprocal.as_base() - 0.25).abs() < f64::EPSILON);
}

#[test]
fn powi_cubes_velocity_shape() {
    // (2 m/s)³ = 8 m³/s³.
    let velocity = VelocityQuantity::from_base(2.0);
    let cubed = velocity.powi::<P3>();
    let cubed: Quantity<f64, CubedVelocityShape> = cubed;
    assert!((*cubed.as_base() - 8.0).abs() < f64::EPSILON);
}

#[test]
fn powi_two_is_multiplication_with_self() {
    // length² == length * length.
    let length = LengthQuantity::from_base(5.0);
    let squared = length.powi::<P2>();
    let multiplied: AreaQuantity = length * length;
    assert!((*squared.as_base() - *multiplied.as_base()).abs() < f64::EPSILON);
}

#[test]
fn powi_preserves_value_semantics() {
    let length = LengthQuantity::from_base(7.0);
    let squared: AreaQuantity = length.powi::<P2>();
    assert!((*squared.as_base() - 7.0_f64.powi(2)).abs() < f64::EPSILON);
}

#[test]
fn powi_squared_area_displays_in_square_metres() {
    let length = LengthQuantity::from_base(3.0);
    let area: AreaQuantity = length.powi::<P2>();
    let rendered = format!("{}", UnitDisplay::new(&area, SquareMeter));
    assert_eq!(rendered, "9 m²");
}

#[test]
fn powi_one_is_identity() {
    let length = LengthQuantity::from_base(2.5);
    let same = length.powi::<P1>();
    let same: LengthQuantity = same;
    assert!((*same.as_base() - 2.5).abs() < f64::EPSILON);
}

#[test]
fn powi_normalizes_semantics_marker() {
    use aequitas::systems::si::quantities::{Angle, Dimensionless as DimensionlessQuantity};
    // angle² is a plain dimensionless result (marker normalized), not an Angle.
    let angle = Angle::from_base(3.0);
    let squared: DimensionlessQuantity = angle.powi::<P2>();
    assert!((*squared.as_base() - 9.0).abs() < f64::EPSILON);
}

#[test]
fn powi_of_reciprocal_length_is_reciprocal_area() {
    // (2 m⁻¹)² = 4 m⁻².
    let reciprocal_length = ReciprocalLength::from_base(2.0);
    let reciprocal_area = reciprocal_length.powi::<P2>();
    let reciprocal_area: Quantity<f64, ReciprocalAreaShape> = reciprocal_area;
    assert!((*reciprocal_area.as_base() - 4.0).abs() < f64::EPSILON);
}

#[test]
fn powi_squared_velocity_is_speed_squared_shape() {
    // velocity² has length²·time⁻², which sqrt reduces back to velocity.
    let velocity = VelocityQuantity::from_base(4.0);
    let v2 = velocity.powi::<P2>();
    let v2: Quantity<f64, SquaredVelocityShape> = v2;
    let v: VelocityQuantity = v2.sqrt();
    assert!((*v.as_base() - 4.0).abs() < f64::EPSILON);
}

#[test]
fn powi_acceleration_shape() {
    // acceleration = m/s². Squaring gives length²·time⁻⁴.
    let acceleration = Acceleration::from_base(2.0);
    let squared = acceleration.powi::<P2>();
    let squared: Quantity<f64, SquaredAccelerationShape> = squared;
    assert!((*squared.as_base() - 4.0).abs() < f64::EPSILON);
    // And sqrt restores the acceleration shape.
    let back: Acceleration = squared.sqrt();
    assert!((*back.as_base() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn reciprocal_of_time_is_reciprocal_time() {
    // (2 s)⁻¹ = 0.5 s⁻¹.
    let time = TimeQuantity::from_base(2.0);
    let reciprocal = time.reciprocal();
    let reciprocal: RecipTimeQuantity = reciprocal;
    assert!((*reciprocal.as_base() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn reciprocal_of_length_is_reciprocal_length() {
    // (4 m)⁻¹ = 0.25 m⁻¹.
    let length = LengthQuantity::from_base(4.0);
    let reciprocal = length.reciprocal();
    let reciprocal: ReciprocalLength = reciprocal;
    assert!((*reciprocal.as_base() - 0.25).abs() < f64::EPSILON);
}

#[test]
fn reciprocal_inverts_velocity_shape() {
    // velocity = length·time⁻¹, so velocity⁻¹ = time·length⁻¹.
    let velocity = VelocityQuantity::from_base(4.0);
    let reciprocal = velocity.reciprocal();
    let reciprocal: Quantity<f64, ReciprocalVelocityShape> = reciprocal;
    assert!((*reciprocal.as_base() - 0.25).abs() < f64::EPSILON);
}

#[test]
fn reciprocal_agrees_with_powi_n1() {
    // reciprocal() and powi::<N1>() are the same dimension and value.
    let time = TimeQuantity::from_base(5.0);
    let named: RecipTimeQuantity = time.reciprocal();
    let powered: RecipTimeQuantity = time.powi::<N1>();
    assert!((*named.as_base() - *powered.as_base()).abs() < f64::EPSILON);
}

#[test]
fn reciprocal_of_reciprocal_is_identity() {
    // (4 m)⁻¹⁻¹ = 4 m; inverting twice restores the original quantity.
    let length = LengthQuantity::from_base(4.0);
    let back: LengthQuantity = length.reciprocal().reciprocal();
    assert!((*back.as_base() - 4.0).abs() < f64::EPSILON);
}
