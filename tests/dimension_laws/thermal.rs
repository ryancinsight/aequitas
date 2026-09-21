//! Thermal identities: heat capacity, diffusivity and the temperature-response
//! coefficients that must reduce to a dimensionless factor.

use aequitas::systems::si::{
    quantities::{
        Dimensionless, Energy, EnergyPerVolume, Mass, MassDensity, MassDensityPerTemperature,
        ReciprocalLength, ReciprocalLengthPerTemperature, ReciprocalTemperature,
        ReciprocalTemperatureSquared, SpecificEnergy, SpecificHeatCapacity, TemperatureDifference,
        ThermalConductivity, ThermalDiffusivity, ThermodynamicTemperature, Velocity,
        VelocityPerTemperature, Volume,
    },
    units::{
        JoulePerCubicMeter, JoulePerKilogram, JoulePerKilogramKelvin, JoulePerMilliliter, Kelvin,
        Kilogram, KilogramPerCubicMeter, KilogramPerCubicMeterKelvin, MeterPerSecond,
        MeterPerSecondKelvin, PerKelvin, PerMeter, PerMeterKelvin, PerSquareKelvin,
    },
};

#[test]
fn mass_times_specific_heat_and_temperature_is_energy() {
    let mass = Mass::from_unit::<Kilogram>(2.0_f64);
    let specific_heat = SpecificHeatCapacity::from_unit::<JoulePerKilogramKelvin>(4_200.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(3.0_f64);
    let energy: Energy = mass * specific_heat * temperature;

    assert_eq!(energy.into_base().to_bits(), 25_200.0_f64.to_bits());
}

#[test]
fn specific_energy_has_a_joule_per_kilogram_contract() {
    let energy = SpecificEnergy::from_unit::<JoulePerKilogram>(12.5_f64);

    assert_eq!(energy.into_base().to_bits(), 12.5_f64.to_bits());
}

#[test]
fn conductivity_over_density_and_specific_heat_is_thermal_diffusivity() {
    let conductivity = ThermalConductivity::from_base(0.6_f64);
    let density = MassDensity::from_base(1_000.0_f64);
    let specific_heat = SpecificHeatCapacity::from_base(4_000.0_f64);

    let diffusivity: ThermalDiffusivity = conductivity / (density * specific_heat);

    assert_eq!(diffusivity.into_base().to_bits(), 1.5e-7_f64.to_bits());
}

#[test]
fn temperature_response_coefficients_reduce_to_dimensionless_factors() {
    let delta = TemperatureDifference::from_unit::<Kelvin>(2.0_f64);
    let slope = ReciprocalTemperature::from_unit::<PerKelvin>(0.25_f64);
    let curvature = ReciprocalTemperatureSquared::from_unit::<PerSquareKelvin>(0.125_f64);

    let linear: Dimensionless = slope * delta;
    let quadratic: Dimensionless = curvature * delta * delta;

    assert_eq!(linear.into_base().to_bits(), 0.5_f64.to_bits());
    assert_eq!(quadratic.into_base().to_bits(), 0.5_f64.to_bits());
}

#[test]
fn thermal_coefficient_dimensions_close_over_temperature_difference() {
    let delta = TemperatureDifference::from_unit::<Kelvin>(2.0_f64);
    let velocity_slope = VelocityPerTemperature::from_unit::<MeterPerSecondKelvin>(3.0_f64);
    let density_slope =
        MassDensityPerTemperature::from_unit::<KilogramPerCubicMeterKelvin>(4.0_f64);
    let absorption_slope = ReciprocalLengthPerTemperature::from_unit::<PerMeterKelvin>(5.0_f64);

    let velocity: Velocity = velocity_slope * delta;
    let density: MassDensity = density_slope * delta;
    let absorption: ReciprocalLength = absorption_slope * delta;

    assert_eq!(
        velocity.in_unit::<MeterPerSecond>().to_bits(),
        6.0_f64.to_bits()
    );
    assert_eq!(
        density.in_unit::<KilogramPerCubicMeter>().to_bits(),
        8.0_f64.to_bits()
    );
    assert_eq!(
        absorption.in_unit::<PerMeter>().to_bits(),
        10.0_f64.to_bits()
    );
}

#[test]
fn energy_density_and_temperature_difference_dimensions_close() {
    let energy = Energy::from_unit::<aequitas::systems::si::units::Joule>(12.0_f64);
    let volume = Volume::from_unit::<aequitas::systems::si::units::CubicMeter>(3.0_f64);
    let energy_density: EnergyPerVolume = energy / volume;
    assert_eq!(
        energy_density.in_unit::<JoulePerCubicMeter>().to_bits(),
        4.0_f64.to_bits()
    );
    assert_eq!(
        EnergyPerVolume::from_unit::<JoulePerMilliliter>(2.0_f64)
            .in_unit::<JoulePerCubicMeter>()
            .to_bits(),
        2.0e6_f64.to_bits()
    );

    let delta = TemperatureDifference::from_unit::<Kelvin>(2.0_f64);
    let specific_heat = SpecificHeatCapacity::from_unit::<JoulePerKilogramKelvin>(4_000.0_f64);
    let mass = Mass::from_unit::<Kilogram>(3.0_f64);
    let energy_from_delta: Energy = mass * specific_heat * delta;
    assert_eq!(
        energy_from_delta.into_base().to_bits(),
        24_000.0_f64.to_bits()
    );

    let slope = ReciprocalTemperature::from_unit::<PerKelvin>(0.25_f64);
    let response: Dimensionless = slope * delta;
    assert_eq!(response.into_base().to_bits(), 0.5_f64.to_bits());

    let absolute = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);
    let absolute_energy: Energy = mass * specific_heat * absolute;
    assert_eq!(
        absolute_energy.into_base().to_bits(),
        3_600_000.0_f64.to_bits()
    );
}
