//! Thermal-law values: diffusivity, Fourier flux, Pennes source and the
//! Stefan–Boltzmann exitance.

use aequitas::systems::si::{
    quantities::{
        Intensity, Length, MassDensity, ReciprocalTime, SpecificHeatCapacity,
        TemperatureDifference, ThermalConductivity, ThermalDiffusivity, ThermodynamicTemperature,
        VolumetricPowerDensity,
    },
    units::{Kelvin, KilogramPerCubicMeter, Meter, PerSecond, WattPerSquareMeter},
};

#[test]
fn thermal_diffusivity_names_the_water_value() {
    let conductivity = ThermalConductivity::from_base(0.6_f64);
    let density = MassDensity::from_base(1_000.0_f64);
    let heat_capacity = SpecificHeatCapacity::from_base(4_000.0_f64);

    let diffusivity: ThermalDiffusivity = conductivity.thermal_diffusivity(density, heat_capacity);

    assert_eq!(diffusivity.into_base().to_bits(), 1.5e-7_f64.to_bits());
}

#[test]
fn fourier_flux_matches_closed_form() {
    let conductivity = ThermalConductivity::from_base(200.0_f64);
    let drop = TemperatureDifference::from_unit::<Kelvin>(50.0_f64);
    let thickness = Length::from_unit::<Meter>(0.1_f64);

    let flux: Intensity = conductivity.fourier_flux(drop, thickness);

    assert_eq!(
        flux.in_unit::<WattPerSquareMeter>().to_bits(),
        100_000.0_f64.to_bits()
    );
}

#[test]
fn pennes_source_matches_closed_form() {
    let blood = MassDensity::from_unit::<KilogramPerCubicMeter>(1_050.0_f64);
    let heat_capacity = SpecificHeatCapacity::from_base(3_770.0_f64);
    let perfusion = ReciprocalTime::from_unit::<PerSecond>(0.01_f64);
    let delta = TemperatureDifference::from_unit::<Kelvin>(2.0_f64);

    let source: VolumetricPowerDensity =
        blood.pennes_perfusion_source(heat_capacity, perfusion, delta);
    let want: f64 = 1_050.0 * 3_770.0 * 0.01 * 2.0;

    assert!((source.into_base() - want).abs() <= 1e-6 * want.abs());
}

#[test]
fn blackbody_exitance_at_1000k() {
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(1_000.0_f64);

    let exitance: Intensity = temperature.blackbody_exitance();
    let want = 5.670_374_419_184_43e-8 * 1.0e12;

    assert!((exitance.into_base() - want).abs() <= 1e-3 * want.abs());
}
