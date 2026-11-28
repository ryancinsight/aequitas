//! Electromagnetism-law values: Ohm, capacitance, Lorentz forces and the
//! vacuum wave speed.

use aequitas::systems::si::{
    quantities::{
        Capacitance, ElectricCharge, ElectricCurrent, ElectricPotential, Force, Length, Velocity,
    },
    units::{Ampere, Coulomb, Farad, Meter, Newton, Volt},
};

#[test]
fn ohm_current_matches_closed_form() {
    use aequitas::systems::si::quantities::ElectricalImpedance;
    let potential = ElectricPotential::from_unit::<Volt>(12.0_f64);
    let resistance = ElectricalImpedance::from_base(4.0_f64);

    let current: ElectricCurrent = potential.ohm_current(resistance);

    assert_eq!(current.in_unit::<Ampere>().to_bits(), 3.0_f64.to_bits());
}

#[test]
fn capacitance_for_charge_matches_closed_form() {
    let potential = ElectricPotential::from_unit::<Volt>(5.0_f64);
    let charge = ElectricCharge::from_unit::<Coulomb>(6.0_f64);

    let capacitance: Capacitance = potential.capacitance_for_charge(charge);

    assert_eq!(capacitance.in_unit::<Farad>().to_bits(), 1.2_f64.to_bits());
}

#[test]
fn lorentz_forces_match_closed_forms() {
    use aequitas::systems::si::{
        quantities::MagneticFluxDensity,
        units::{MeterPerSecond, Tesla, VoltPerMeter},
    };
    let charge = ElectricCharge::from_unit::<Coulomb>(2.0_f64);
    let field = aequitas::systems::si::quantities::ElectricFieldStrength::from_unit::<VoltPerMeter>(
        3.0_f64,
    );

    let electric: Force = charge.electric_force(field);
    assert_eq!(electric.in_unit::<Newton>().to_bits(), 6.0_f64.to_bits());

    let flux = MagneticFluxDensity::from_unit::<Tesla>(0.5_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(4.0_f64);
    let magnetic: Force = flux.magnetic_force(charge, velocity);
    assert_eq!(magnetic.in_unit::<Newton>().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn vacuum_wave_speed_recovers_c() {
    use aequitas::systems::si::constants::{VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY};

    let speed: Velocity = VACUUM_PERMITTIVITY.vacuum_wave_speed(VACUUM_PERMEABILITY);

    assert!((speed.into_base() - 299_792_458.0).abs() <= 1.0);
}

#[test]
fn wavelength_is_speed_over_frequency() {
    use aequitas::systems::si::{
        quantities::Frequency,
        units::{Hertz, MeterPerSecond},
    };
    let speed = Velocity::from_unit::<MeterPerSecond>(343.0_f64);
    let frequency = Frequency::from_unit::<Hertz>(440.0_f64);

    let wavelength: Length = Length::wavelength_for_frequency(speed, frequency);
    let want = 343.0 / 440.0;

    assert!((wavelength.in_unit::<Meter>() - want).abs() <= 1e-12);
}
