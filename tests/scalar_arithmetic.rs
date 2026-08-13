//! Value-semantic checks for scalar arithmetic ergonomics on [`Quantity`].

use aequitas::quantity::{Quantity, UnitDisplay};
use aequitas::systems::si::quantities::{Energy, Length, Time, Velocity};
use aequitas::systems::si::units::{Joule, MeterPerSecond};

#[test]
fn scalar_times_quantity_is_commutative() {
    let v = Velocity::from_base(2.5);
    let left = 2.0_f64 * v;
    let right = v * 2.0_f64;
    // 2.0 * 2.5 m/s = 5.0 m/s both ways.
    assert!((*left.as_base() - 5.0_f64).abs() < f64::EPSILON);
    assert!((*right.as_base() - 5.0_f64).abs() < f64::EPSILON);
}

#[test]
fn scalar_mul_assign_scales_in_place() {
    let mut v = Velocity::from_base(2.5);
    v *= 4.0_f64;
    assert!((*v.as_base() - 10.0_f64).abs() < f64::EPSILON);
}

#[test]
fn scalar_div_assign_scales_in_place() {
    let mut e = Energy::from_base(10.0);
    e /= 2.0_f64;
    assert!((*e.as_base() - 5.0_f64).abs() < f64::EPSILON);
}

#[test]
fn scalar_ops_preserve_dimension_and_unit() {
    let mut v = Velocity::from_base(2.5);
    v *= 2.0_f64;
    // The quantity is still a velocity: displaying in m/s gives 5.
    let rendered = format!("{}", UnitDisplay::new(&v, MeterPerSecond));
    assert_eq!(rendered, "5 m/s");
}

#[test]
fn scalar_ops_compose_with_quantity_arithmetic() {
    let v = Velocity::from_base(3.0);
    let t = Time::from_base(2.0);
    let distance = 2.0_f64 * v * t;
    // 2 * 3 m/s * 2 s = 12 m.
    assert!((*distance.as_base() - 12.0_f64).abs() < f64::EPSILON);
}

#[test]
fn scalar_div_by_value_matches_compound() {
    let a = Energy::from_base(12.0);
    let b = a / 3.0_f64;
    let mut c = Energy::from_base(12.0);
    c /= 3.0_f64;
    assert!((*b.as_base() - *c.as_base()).abs() < f64::EPSILON);
}

#[test]
fn scalar_mul_assign_with_complex_quantity() {
    use eunomia::Complex64;
    let mut z = Quantity::<Complex64, Length>::from_base(Complex64::new(1.0, 2.0));
    z *= 3.0_f64;
    // (1 + 2i) * 3 = 3 + 6i.
    let value = *z.as_base();
    assert!((value.re - 3.0).abs() < f64::EPSILON);
    assert!((value.im - 6.0).abs() < f64::EPSILON);
}

#[test]
fn scalar_times_complex_quantity_is_commutative() {
    use eunomia::Complex64;
    let z = Quantity::<Complex64, Length>::from_base(Complex64::new(1.0, -1.0));
    let left = 2.0_f64 * z;
    let right = z * 2.0_f64;
    assert!((left.as_base().re - 2.0).abs() < f64::EPSILON);
    assert!((right.as_base().im - -2.0).abs() < f64::EPSILON);
}

#[test]
fn energy_displays_with_joule() {
    let e = Energy::from_base(3.0);
    let rendered = format!("{}", UnitDisplay::new(&e, Joule));
    assert_eq!(rendered, "3 J");
}

#[test]
fn complex_div_by_scalar_scales_reciprocally() {
    use eunomia::Complex64;
    let z = Quantity::<Complex64, Length>::from_base(Complex64::new(2.0, 4.0));
    let half = z / 2.0_f64;
    // (2 + 4i) / 2 = 1 + 2i.
    assert!((half.as_base().re - 1.0).abs() < f64::EPSILON);
    assert!((half.as_base().im - 2.0).abs() < f64::EPSILON);
}

#[test]
fn complex_div_assign_scales_in_place() {
    use eunomia::Complex64;
    let mut z = Quantity::<Complex64, Length>::from_base(Complex64::new(4.0, -8.0));
    z /= 2.0_f64;
    assert!((z.as_base().re - 2.0).abs() < f64::EPSILON);
    assert!((z.as_base().im - -4.0).abs() < f64::EPSILON);
}

#[test]
fn complex_div_by_scalar_matches_compound() {
    use eunomia::Complex64;
    let a = Quantity::<Complex64, Length>::from_base(Complex64::new(6.0, 3.0));
    let b = a / 3.0_f64;
    let mut c = Quantity::<Complex64, Length>::from_base(Complex64::new(6.0, 3.0));
    c /= 3.0_f64;
    assert!((b.as_base().re - c.as_base().re).abs() < f64::EPSILON);
    assert!((b.as_base().im - c.as_base().im).abs() < f64::EPSILON);
}

#[test]
fn scalar_left_mul_with_f32_is_commutative() {
    let v = Quantity::<f32, Velocity>::from_base(2.5_f32);
    let left = 2.0_f32 * v;
    let right = v * 2.0_f32;
    // 2.0 * 2.5 m/s = 5.0 m/s both ways.
    assert!((*left.as_base() - 5.0_f32).abs() < f32::EPSILON);
    assert!((*right.as_base() - 5.0_f32).abs() < f32::EPSILON);
}

#[test]
fn f32_compound_assign_ops_scale_in_place() {
    let mut v = Quantity::<f32, Velocity>::from_base(2.5_f32);
    v *= 4.0_f32;
    assert!((*v.as_base() - 10.0_f32).abs() < f32::EPSILON);
    v /= 5.0_f32;
    assert!((*v.as_base() - 2.0_f32).abs() < f32::EPSILON);
}
