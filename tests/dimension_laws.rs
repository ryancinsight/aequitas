//! Compile-time dimensional identities with exact binary values.

use aequitas::systems::si::{
    quantities::{
        Area, Dimensionless, Energy, Length, Mass, MassDensity, Power, Pressure,
        ReciprocalTemperature, ReciprocalTemperatureSquared, SpecificHeatCapacity,
        ThermalConductivity, ThermalDiffusivity, ThermodynamicTemperature, Time, Velocity, Volume,
    },
    units::{
        JoulePerKilogramKelvin, Kelvin, Kilogram, Meter, Pascal, PerKelvin, PerSquareKelvin,
        Second, SquareMeter, Watt,
    },
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

#[test]
fn mass_times_specific_heat_and_temperature_is_energy() {
    let mass = Mass::from_unit::<Kilogram>(2.0_f64);
    let specific_heat = SpecificHeatCapacity::from_unit::<JoulePerKilogramKelvin>(4_200.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(3.0_f64);
    let energy: Energy = mass * specific_heat * temperature;

    assert_eq!(energy.into_base().to_bits(), 25_200.0_f64.to_bits());
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
    let delta = ThermodynamicTemperature::from_unit::<Kelvin>(2.0_f64);
    let slope = ReciprocalTemperature::from_unit::<PerKelvin>(0.25_f64);
    let curvature = ReciprocalTemperatureSquared::from_unit::<PerSquareKelvin>(0.125_f64);

    let linear: Dimensionless = slope * delta;
    let quadratic: Dimensionless = curvature * delta * delta;

    assert_eq!(linear.into_base().to_bits(), 0.5_f64.to_bits());
    assert_eq!(quadratic.into_base().to_bits(), 0.5_f64.to_bits());
}
