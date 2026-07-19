//! Compile-time dimensional identities with exact binary values.

use aequitas::systems::si::{
    quantities::{Area, Energy, Length, Power, Pressure, Time, Velocity, Volume},
    units::{Meter, Pascal, Second, SquareMeter, Watt},
};

#[test]
fn length_divided_by_time_is_velocity() {
    let length = Length::from_unit::<Meter>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let velocity: Velocity = length / time;

    assert_eq!(velocity.into_base().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn pressure_times_volume_is_energy() {
    let pressure = Pressure::from_unit::<Pascal>(6.0_f64);
    let area = Area::from_unit::<SquareMeter>(2.0_f64);
    let length = Length::from_unit::<Meter>(4.0_f64);
    let volume: Volume = area * length;
    let energy: Energy = pressure * volume;

    assert_eq!(energy.into_base().to_bits(), 48.0_f64.to_bits());
}

#[test]
fn power_times_time_is_energy() {
    let power = Power::from_unit::<Watt>(7.0_f64);
    let time = Time::from_unit::<Second>(5.0_f64);
    let energy: Energy = power * time;

    assert_eq!(energy.into_base().to_bits(), 35.0_f64.to_bits());
}
