//! Chemistry-law values: ideal gas, Fick flux, Arrhenius and Q10.

use aequitas::systems::si::{
    quantities::{
        AmountOfSubstance, Dimensionless, Length, MassDiffusivity, MolarConcentration, MolarEnergy,
        Pressure, ReciprocalTime, ThermodynamicTemperature, Volume,
    },
    units::{CubicMeter, JoulePerMole, Kelvin, Meter, Mole, Pascal, PerSecond},
};

#[test]
fn ideal_gas_pressure_matches_closed_form() {
    let amount = AmountOfSubstance::from_unit::<Mole>(1.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(273.15_f64);
    let volume = Volume::from_unit::<CubicMeter>(0.022_414_f64);

    let pressure: Pressure = amount.ideal_gas_pressure(temperature, volume);
    let want = 8.314_462_618_153_24 * 273.15 / 0.022_414;

    assert!((pressure.into_base() - want).abs() <= 1e-6 * want.abs());
    assert!((pressure.in_unit::<Pascal>() - want).abs() <= 1e-6 * want.abs());
}

#[test]
fn fick_flux_matches_closed_form() {
    use aequitas::systems::si::{
        quantities::MolarFlux as Flux,
        units::{MolePerCubicMeter, MolePerSquareMeterSecond},
    };
    let upstream = MolarConcentration::from_unit::<MolePerCubicMeter>(10.0_f64);
    let downstream = MolarConcentration::from_unit::<MolePerCubicMeter>(4.0_f64);
    let diffusivity = MassDiffusivity::from_base(1.0e-9_f64);

    let flux: Flux = MolarConcentration::fick_flux(
        upstream,
        downstream,
        diffusivity,
        Length::from_unit::<Meter>(0.01_f64),
    );
    let want: f64 = 1.0e-9 * (10.0 - 4.0) / 0.01;

    assert!((flux.in_unit::<MolePerSquareMeterSecond>() - want).abs() <= 1e-15);
}

#[test]
fn arrhenius_rate_matches_closed_form() {
    let activation = MolarEnergy::from_unit::<JoulePerMole>(50_000.0_f64);
    let pre = ReciprocalTime::from_unit::<PerSecond>(1.0e13_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);

    let rate: ReciprocalTime = activation.arrhenius_rate(pre, temperature);
    let want: f64 = 1.0e13_f64 * (-50_000.0_f64 / (8.314_462_618_153_24_f64 * 300.0_f64)).exp();

    assert!((rate.into_base() - want).abs() <= 1e-6 * want.abs());
}

#[test]
fn q10_of_typical_activation_exceeds_one() {
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);
    let activation = MolarEnergy::from_unit::<JoulePerMole>(50_000.0_f64);

    let q10: Dimensionless = temperature.q10_factor(activation);
    let want: f64 = (50_000.0_f64 / 8.314_462_618_153_24_f64
        * (1.0_f64 / 300.0_f64 - 1.0_f64 / 310.0_f64))
        .exp();

    assert!(q10.into_base() > 1.0);
    assert!((q10.into_base() - want).abs() <= 1e-9 * want.abs());
}
