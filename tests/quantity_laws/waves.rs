//! Wave-law values: skin depth, plasma cutoff, Poynting magnitude,
//! dielectric speed and free-space wavelength.

use aequitas::systems::si::{
    quantities::{
        AngularVelocity, Dimensionless, ElectricFieldStrength, ElectricalConductivity, Frequency,
        Intensity, Length, MagneticPermeability, NumberDensity,
    },
    units::{Hertz, SiemensPerMeter, VoltPerMeter, WattPerSquareMeter},
};

#[test]
fn skin_depth_of_copper_at_50_hz() {
    let omega = AngularVelocity::from_frequency(Frequency::from_unit::<Hertz>(50.0_f64));
    let mu = MagneticPermeability::from_base(4.0 * core::f64::consts::PI * 1.0e-7);
    let sigma = ElectricalConductivity::from_unit::<SiemensPerMeter>(5.96e7_f64);

    let depth: Length = omega.skin_depth(mu, sigma);
    let want = (2.0
        / (2.0 * core::f64::consts::PI * 50.0 * 4.0e-7 * core::f64::consts::PI * 5.96e7))
        .sqrt();

    assert!((depth.into_base() - want).abs() <= 1e-6 * want.abs());
}

#[test]
fn plasma_frequency_matches_closed_form() {
    let density = NumberDensity::from_base(1.0e18_f64);

    let omega: AngularVelocity = density.plasma_frequency();
    let want = (1.0e18 * (1.602_176_634e-19_f64).powi(2)
        / (8.854_187_818_8e-12 * 9.109_383_713_9e-31))
        .sqrt();

    assert!((omega.into_base() - want).abs() <= 1e-6 * want.abs());
}

#[test]
fn poynting_magnitude_matches_closed_form() {
    use aequitas::systems::si::{quantities::ElectricalImpedance, units::Ohm};
    let field = ElectricFieldStrength::from_unit::<VoltPerMeter>(100.0_f64);
    let impedance = ElectricalImpedance::from_unit::<Ohm>(377.0_f64);

    let flux: Intensity = field.poynting_magnitude(impedance);
    let want = 10_000.0 / 377.0;

    assert!((flux.in_unit::<WattPerSquareMeter>() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn dielectric_speed_reduces_by_index() {
    use aequitas::systems::si::quantities::Permittivity;
    let eps_r = Dimensionless::from_base(4.0_f64);

    let speed = Permittivity::dielectric_wave_speed(eps_r);

    assert!((speed.into_base() - 299_792_458.0 / 2.0).abs() <= 1.0);
}

#[test]
fn free_space_wavelength_matches_c_over_f() {
    let frequency = Frequency::from_unit::<Hertz>(2.45e9_f64);

    let wavelength: Length = frequency.free_space_wavelength();
    let want = 299_792_458.0 / 2.45e9;

    assert!((wavelength.into_base() - want).abs() <= 1e-9 * want.abs());
}
