//! Acoustic-law values: sound speed, impedance, intensity–pressure and
//! particle velocity.

use aequitas::systems::si::{
    quantities::{AcousticImpedance, Intensity, MassDensity, Pressure, Velocity},
    units::{KilogramPerCubicMeter, MeterPerSecond, Pascal, Rayl, WattPerSquareMeter},
};

#[test]
fn sound_speed_in_water_matches_closed_form() {
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let bulk_modulus = Pressure::from_base(2.2e9_f64);

    let speed: Velocity = density.sound_speed(bulk_modulus);
    let want = (2.2e9_f64 / 1_000.0).sqrt();

    assert!((speed.in_unit::<MeterPerSecond>() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn characteristic_impedance_matches_closed_form() {
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let speed = Velocity::from_unit::<MeterPerSecond>(1_480.0_f64);

    let impedance: AcousticImpedance = density.characteristic_impedance(speed);

    assert_eq!(impedance.into_base().to_bits(), 1_480_000.0_f64.to_bits());
    let _ = Rayl;
}

#[test]
fn rms_pressure_and_intensity_round_trip() {
    let intensity = Intensity::from_unit::<WattPerSquareMeter>(1.0_f64);
    let impedance = AcousticImpedance::from_base(1.5e6_f64);

    let pressure: Pressure = intensity.rms_pressure(impedance);
    let want_p = (1.0_f64 * 1.5e6).sqrt();
    assert!((pressure.in_unit::<Pascal>() - want_p).abs() <= 1e-9 * want_p.abs());

    let back: Intensity = pressure.rms_intensity(impedance);
    assert!((back.into_base() - 1.0).abs() <= 1e-9);
}

#[test]
fn particle_velocity_matches_closed_form() {
    let pressure = Pressure::from_unit::<Pascal>(1_500.0_f64);
    let impedance = AcousticImpedance::from_base(1.5e6_f64);

    let velocity: Velocity = pressure.particle_velocity(impedance);

    assert_eq!(
        velocity.in_unit::<MeterPerSecond>().to_bits(),
        1.0e-3_f64.to_bits()
    );
}
