//! Kinematic identities: length, velocity and pressure dividing into rates.

use aequitas::systems::si::{
    quantities::{Acceleration, Length, Pressure, PressureRate, Time, Velocity},
    units::{Meter, MeterPerSecond, MeterPerSecondSquared, Pascal, PascalPerSecond, Second},
};

#[test]
fn length_divided_by_time_is_velocity() {
    let length = Length::from_unit::<Meter>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let velocity: Velocity = length / time;

    assert_eq!(velocity.into_base().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn velocity_divided_by_time_is_acceleration() {
    let velocity = Velocity::from_unit::<MeterPerSecond>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let acceleration: Acceleration = velocity / time;

    assert_eq!(
        acceleration.in_unit::<MeterPerSecondSquared>().to_bits(),
        4.0_f64.to_bits()
    );
}

#[test]
fn pressure_divided_by_time_is_pressure_rate() {
    let pressure = Pressure::from_unit::<Pascal>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let rate: PressureRate = pressure / time;

    assert_eq!(
        rate.in_unit::<PascalPerSecond>().to_bits(),
        4.0_f64.to_bits()
    );
}
