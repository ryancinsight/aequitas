//! Mechanical identities: the products and ratios that reach force and energy.

use aequitas::systems::si::{
    quantities::{Area, Energy, Force, Length, Power, Pressure, SurfaceTension, Time, Volume},
    units::{Meter, Newton, NewtonPerMeter, Pascal, Second, SquareMeter, Watt},
};

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
fn pressure_times_area_is_force() {
    let pressure = Pressure::from_unit::<Pascal>(6.0_f64);
    let area = Area::from_unit::<SquareMeter>(2.0_f64);
    let force: Force = pressure * area;

    assert_eq!(force.in_unit::<Newton>().to_bits(), 12.0_f64.to_bits());
}

#[test]
fn surface_tension_is_a_distinct_force_per_length_quantity() {
    let tension = SurfaceTension::from_unit::<NewtonPerMeter>(0.072_f64);
    let radius = Length::from_unit::<Meter>(2.0e-3_f64);
    let pressure: Pressure = tension / radius;

    assert_eq!(
        tension.in_unit::<NewtonPerMeter>().to_bits(),
        0.072_f64.to_bits()
    );
    assert_eq!(pressure.in_unit::<Pascal>().to_bits(), 36.0_f64.to_bits());
}

#[test]
fn power_times_time_is_energy() {
    let power = Power::from_unit::<Watt>(7.0_f64);
    let time = Time::from_unit::<Second>(5.0_f64);
    let energy: Energy = power * time;

    assert_eq!(energy.into_base().to_bits(), 35.0_f64.to_bits());
}
