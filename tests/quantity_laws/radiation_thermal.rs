//! Thermal-radiation law values: Wien peaks, photon energy, radiation
//! pressure and molar photon flux.

use aequitas::systems::si::{
    quantities::{
        Energy, Frequency, Intensity, Length, MolarFlux, Pressure, ThermodynamicTemperature,
    },
    units::{Hertz, Joule, Kelvin, Meter, MolePerSquareMeterSecond, Pascal, WattPerSquareMeter},
};

#[test]
fn wien_peak_of_solar_temperature() {
    let sun = ThermodynamicTemperature::from_unit::<Kelvin>(5_772.0_f64);

    let frequency: Frequency = sun.wien_peak_frequency();
    let want_nu = 2.821_439_372_122_078_7 * 1.380_649e-23 * 5_772.0 / 6.626_070_15e-34;
    assert!((frequency.in_unit::<Hertz>() - want_nu).abs() <= 1e-6 * want_nu);

    let wavelength: Length = sun.wien_peak_wavelength();
    let want_lambda =
        6.626_070_15e-34 * 299_792_458.0 / (4.965_114_231_744_276 * 1.380_649e-23 * 5_772.0);
    assert!((wavelength.into_base() - want_lambda).abs() <= 1e-9 * want_lambda);
    assert!((wavelength.in_unit::<Meter>() - 5.02e-7).abs() < 1.0e-8);
}

#[test]
fn photon_energy_matches_planck_relation() {
    let green = Frequency::from_unit::<Hertz>(5.0e14_f64);

    let energy: Energy = green.photon_energy();
    let want = 6.626_070_15e-34 * 5.0e14;

    assert!((energy.in_unit::<Joule>() - want).abs() <= 1e-9 * want);
}

#[test]
fn radiation_pressure_matches_closed_form() {
    let flux = Intensity::from_unit::<WattPerSquareMeter>(1_361.0_f64);

    let pressure: Pressure = flux.radiation_pressure();
    let want = 1_361.0 / 299_792_458.0;

    assert!((pressure.in_unit::<Pascal>() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn molar_photon_flux_matches_closed_form() {
    let flux = Intensity::from_unit::<WattPerSquareMeter>(1_000.0_f64);
    let frequency = Frequency::from_unit::<Hertz>(5.0e14_f64);

    let molar: MolarFlux = flux.molar_photon_flux(frequency);
    let want = 1_000.0 / (6.626_070_15e-34 * 5.0e14) / 6.022_140_76e23;

    assert!((molar.in_unit::<MolePerSquareMeterSecond>() - want).abs() <= 1e-9 * want);
}
