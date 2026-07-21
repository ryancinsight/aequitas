//! Compile-time dimensional identities with exact binary values.

use aequitas::systems::si::{
    quantities::{
        AbsorbedDose, Area, AreaPerMass, Dimensionless, Energy, EnergyPerArea, Length, Mass,
        MassDensity, MolarEnergy, MolarHeatCapacity, Power, Pressure, ReciprocalLength,
        ReciprocalTemperature, ReciprocalTemperatureSquared, ReciprocalTime, SpecificHeatCapacity,
        ThermalConductivity, ThermalDiffusivity, ThermodynamicTemperature, Time, Velocity, Volume,
    },
    units::{
        Gray, JoulePerKilogramKelvin, JoulePerMole, JoulePerMoleKelvin, Kelvin, Kilogram, Meter,
        Pascal, PerKelvin, PerSecond, PerSquareKelvin, Second, SquareMeter, Watt,
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

#[test]
fn biological_response_dimensions_close() {
    let energy = Energy::from_base(12.0_f64);
    let mass = Mass::from_unit::<Kilogram>(3.0_f64);
    let dose: AbsorbedDose = energy / mass;
    assert_eq!(dose.in_unit::<Gray>().to_bits(), 4.0_f64.to_bits());

    let activation = MolarEnergy::from_unit::<JoulePerMole>(8_314.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(1_000.0_f64);
    let gas_constant: MolarHeatCapacity = activation / temperature;
    assert_eq!(
        gas_constant.in_unit::<JoulePerMoleKelvin>().to_bits(),
        8.314_f64.to_bits()
    );

    let rate = ReciprocalTime::from_unit::<PerSecond>(2.0_f64);
    let duration = Time::from_unit::<Second>(0.5_f64);
    let events: Dimensionless = rate * duration;
    assert_eq!(events.into_base().to_bits(), 1.0_f64.to_bits());
}

#[test]
fn photon_interaction_dimensions_close() {
    let area = Area::from_unit::<SquareMeter>(2.0_f64);
    let mass = Mass::from_unit::<Kilogram>(4.0_f64);
    let specific_area: AreaPerMass = area / mass;
    let density = MassDensity::from_base(6.0_f64);
    let attenuation: ReciprocalLength = specific_area * density;
    let path = Length::from_unit::<Meter>(3.0_f64);
    let optical_depth: Dimensionless = attenuation * path;
    let energy = Energy::from_base(8.0_f64);
    let exposure: EnergyPerArea = energy / area;

    assert_eq!(specific_area.into_base().to_bits(), 0.5_f64.to_bits());
    assert_eq!(attenuation.into_base().to_bits(), 3.0_f64.to_bits());
    assert_eq!(optical_depth.into_base().to_bits(), 9.0_f64.to_bits());
    assert_eq!(exposure.into_base().to_bits(), 4.0_f64.to_bits());
}
