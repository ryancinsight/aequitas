//! Generic-instantiation and scalar special-value contracts.

use aequitas::systems::si::{
    quantities::{Length, Time, Velocity},
    units::{Meter, MeterPerSecond, Second},
};
use eunomia::{Bf4, Bf8, Bf16, F4, F8, F16, F32, F64, FloatElement};

fn assert_velocity_law<T: FloatElement>() {
    // Two and one are exactly representable even in Eunomia's finite E2M1
    // format, so this checks quantity arithmetic rather than saturation.
    let length = Length::from_unit::<Meter>(T::from_f64(2.0));
    let time = Time::from_unit::<Second>(T::from_f64(1.0));
    let velocity: Velocity<T> = length / time;

    assert_eq!(velocity.in_unit::<MeterPerSecond>(), T::from_f64(2.0));
}

#[test]
fn dimension_law_monomorphizes_for_every_eunomia_float() {
    assert_velocity_law::<f32>();
    assert_velocity_law::<f64>();
    assert_velocity_law::<F16>();
    assert_velocity_law::<F32>();
    assert_velocity_law::<F64>();
    assert_velocity_law::<Bf16>();
    assert_velocity_law::<Bf8>();
    assert_velocity_law::<Bf4>();
    assert_velocity_law::<F8>();
    assert_velocity_law::<F4>();
}

#[test]
fn special_values_follow_the_scalar_contract() {
    let nan = Length::from_unit::<Meter>(f64::NAN);
    let infinity = Length::from_unit::<Meter>(f64::INFINITY);
    let negative_zero = Length::from_unit::<Meter>(-0.0_f64);

    assert!(nan.into_base().is_nan());
    assert!(infinity.into_base().is_infinite());
    assert!(infinity.into_base().is_sign_positive());
    assert!(negative_zero.into_base().is_sign_negative());
}
