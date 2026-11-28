//! Mechanics-law values: Newton II, weight, Hooke, elastic energy,
//! hydrostatic and stagnation pressure.

use aequitas::systems::si::{
    quantities::{
        Acceleration, Energy, Force, Length, Mass, MassDensity, Pressure, SpringStiffness, Velocity,
    },
    units::{
        Joule, Kilogram, KilogramPerCubicMeter, Meter, MeterPerSecond, MeterPerSecondSquared,
        Newton, Pascal,
    },
};

#[test]
fn newton2_matches_closed_form() {
    let mass = Mass::from_unit::<Kilogram>(2.0_f64);
    let acceleration = Acceleration::from_unit::<MeterPerSecondSquared>(3.0_f64);

    let force: Force = mass.newton2(acceleration);

    assert_eq!(force.in_unit::<Newton>().to_bits(), 6.0_f64.to_bits());
}

#[test]
fn weight_uses_standard_gravity() {
    let mass = Mass::from_unit::<Kilogram>(1.0_f64);

    let weight: Force = mass.weight();

    assert_eq!(weight.in_unit::<Newton>().to_bits(), 9.806_65_f64.to_bits());
}

#[test]
fn kinetic_energy_matches_closed_form() {
    let mass = Mass::from_unit::<Kilogram>(2.0_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(3.0_f64);

    let energy: Energy = mass.kinetic_energy(velocity);

    assert_eq!(energy.in_unit::<Joule>().to_bits(), 9.0_f64.to_bits());
}

#[test]
fn hooke_force_and_energy_match_closed_forms() {
    use aequitas::systems::si::units::NewtonPerMeter;
    let stiffness = SpringStiffness::from_unit::<NewtonPerMeter>(100.0_f64);
    let displacement = Length::from_unit::<Meter>(0.05_f64);

    let force: Force = stiffness.hooke_force(displacement);
    assert_eq!(force.in_unit::<Newton>().to_bits(), 5.0_f64.to_bits());

    let energy: Energy = stiffness.elastic_energy(displacement);
    assert_eq!(energy.in_unit::<Joule>().to_bits(), 0.125_f64.to_bits());
}

#[test]
fn hydrostatic_pressure_matches_closed_form() {
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let height = Length::from_unit::<Meter>(10.0_f64);

    let pressure: Pressure = Pressure::hydrostatic(density, height);
    let want: f64 = 1_000.0 * 9.806_65 * 10.0;

    assert_eq!(pressure.in_unit::<Pascal>().to_bits(), want.to_bits());
}

#[test]
fn stagnation_pressure_adds_dynamic_pressure() {
    let static_pressure = Pressure::from_unit::<Pascal>(100_000.0_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1.225_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(50.0_f64);

    let stagnation = static_pressure.stagnation(density, velocity);
    let want = 100_000.0 + 1.225 * 2_500.0 / 2.0;

    assert!((stagnation.into_base() - want).abs() <= 1e-9 * want.abs());
}
