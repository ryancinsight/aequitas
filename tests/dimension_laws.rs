//! Compile-time dimensional identities with exact binary values.

use aequitas::systems::si::{
    quantities::{
        AbsorbedDose, AbsorbedDoseRate, Acceleration, AcousticImpedance, Angle, Area, AreaPerMass,
        AreaPerTime, Capacitance, Compliance, Dimensionless, DynamicViscosity, ElectricCharge,
        ElectricConductance, ElectricCurrent, ElectricPotential, ElectricalConductivity,
        ElectricalImpedance, Energy, EnergyPerArea, EnergyPerVolume, Force, HydraulicInertance,
        HydraulicResistance, Intensity, KinematicViscosity, Length, Mass, MassDensity,
        MassDensityPerTemperature, MassDensityRate, MolarEnergy, MolarHeatCapacity, NumberDensity,
        Polarizability, Power, Pressure, PressureGradient, PressurePerElectricCurrent,
        PressureRate, QuadraticHydraulicResistance, ReciprocalLength,
        ReciprocalLengthPerTemperature, ReciprocalTemperature, ReciprocalTemperatureSquared,
        ReciprocalTime, ReciprocalTimeSquared, SpecificAbsorptionRate, SpecificEnergy,
        SpecificHeatCapacity, SurfaceTension, TemperatureDifference, ThermalConductivity,
        ThermalDiffusivity, ThermodynamicTemperature, Time, Velocity, VelocityPerTemperature,
        Volume, VolumetricFlowRate, VolumetricPowerDensity, VolumetricPowerDensityGradient,
    },
    units::{
        Ampere, Coulomb, CubicMeterPerSecond, Farad, Gray, GrayPerSecond, JoulePerCubicMeter,
        JoulePerKilogram, JoulePerKilogramKelvin, JoulePerMilliliter, JoulePerMole,
        JoulePerMoleKelvin, Kelvin, Kilogram, KilogramPerCubicMeter, KilogramPerCubicMeterKelvin,
        KilogramPerCubicMeterSecond, Meter, MeterPerSecond, MeterPerSecondKelvin,
        MeterPerSecondSquared, Newton, NewtonPerMeter, Pascal, PascalPerSecond, PascalSecond,
        PerCubicMeter, PerKelvin, PerMeter, PerMeterKelvin, PerSecond, PerSquareKelvin, Radian,
        Rayl, Second, Siemens, SiemensPerMeter, SquareMeter, SquareMeterPerSecond, Volt, Watt,
        WattPerCubicMeter, WattPerKilogram, WattPerMeterFourth, WattPerSquareMeter,
    },
};

#[test]
fn angle_has_a_distinct_radian_semantic_contract() {
    let angle = Angle::from_unit::<Radian>(core::f64::consts::FRAC_PI_2);

    assert_eq!(
        angle.in_unit::<Radian>().to_bits(),
        core::f64::consts::FRAC_PI_2.to_bits()
    );
}

#[test]
fn number_density_uses_the_entity_per_volume_contract() {
    let concentration = NumberDensity::from_unit::<PerCubicMeter>(1.0e12_f64);

    assert_eq!(concentration.into_base().to_bits(), 1.0e12_f64.to_bits());
}

#[test]
fn length_divided_by_time_is_velocity() {
    let length = Length::from_unit::<Meter>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let velocity: Velocity = length / time;

    assert_eq!(velocity.into_base().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn velocity_divided_by_time_is_acceleration() {
    let velocity = Velocity::from_unit::<MeterPerSecond>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let acceleration: Acceleration = velocity / time;

    assert_eq!(
        acceleration.in_unit::<MeterPerSecondSquared>().to_bits(),
        4.0_f64.to_bits()
    );
}

#[test]
fn pressure_divided_by_time_is_pressure_rate() {
    let pressure = Pressure::from_unit::<Pascal>(12.0_f64);
    let time = Time::from_unit::<Second>(3.0_f64);
    let rate: PressureRate = pressure / time;

    assert_eq!(
        rate.in_unit::<PascalPerSecond>().to_bits(),
        4.0_f64.to_bits()
    );
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
fn pressure_times_area_is_force() {
    let pressure = Pressure::from_unit::<Pascal>(6.0_f64);
    let area = Area::from_unit::<SquareMeter>(2.0_f64);
    let force: Force = pressure * area;

    assert_eq!(force.in_unit::<Newton>().to_bits(), 12.0_f64.to_bits());
}

#[test]
fn surface_tension_is_a_distinct_force_per_length_quantity() {
    let tension = SurfaceTension::from_unit::<NewtonPerMeter>(0.072_f64);
    let radius = Length::from_unit::<Meter>(2.0e-3_f64);
    let pressure: Pressure = tension / radius;

    assert_eq!(
        tension.in_unit::<NewtonPerMeter>().to_bits(),
        0.072_f64.to_bits()
    );
    assert_eq!(pressure.in_unit::<Pascal>().to_bits(), 36.0_f64.to_bits());
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
fn planar_flow_rate_per_width_uses_area_per_time() {
    let flow_per_width = AreaPerTime::from_base(0.25_f64);

    assert_eq!(flow_per_width.into_base().to_bits(), 0.25_f64.to_bits());
}

#[test]
fn reciprocal_time_squared_carries_vorticity_squared_dimension() {
    let enstrophy = ReciprocalTimeSquared::from_base(4.0_f64);

    assert_eq!(enstrophy.into_base().to_bits(), 4.0_f64.to_bits());
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
fn thermal_coefficient_units_preserve_eunomia_complex_values() {
    use eunomia::Complex64;

    let coefficient: VelocityPerTemperature<Complex64> =
        VelocityPerTemperature::from_unit::<MeterPerSecondKelvin>(Complex64::new(1.25, -0.5));

    assert_eq!(
        coefficient.in_unit::<MeterPerSecondKelvin>(),
        Complex64::new(1.25, -0.5)
    );
}

#[test]
fn volumetric_power_density_gradient_has_a_watt_per_meter_fourth_contract() {
    let density = VolumetricPowerDensity::from_unit::<WattPerCubicMeter>(12.0_f64);
    let length = Length::from_unit::<Meter>(3.0_f64);
    let gradient: VolumetricPowerDensityGradient = density / length;

    assert_eq!(
        gradient.in_unit::<WattPerMeterFourth>().to_bits(),
        4.0_f64.to_bits()
    );
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
fn vascular_result_dimensions_are_named() {
    let pressure = Pressure::from_base(1.0_f64);
    let length = Length::from_base(2.0_f64);
    let flow = VolumetricFlowRate::from_base(3.0_f64);
    let time = Time::from_base(4.0_f64);

    let gradient: PressureGradient = pressure / length;
    let resistance: HydraulicResistance = pressure / flow;
    let inertance: HydraulicInertance = resistance * time;
    let compliance: Compliance = Volume::from_base(5.0_f64) / pressure;

    assert_eq!(gradient.into_base().to_bits(), 0.5_f64.to_bits());
    assert_eq!(resistance.into_base().to_bits(), (1.0_f64 / 3.0).to_bits());
    assert_eq!(inertance.into_base().to_bits(), (4.0_f64 / 3.0).to_bits());
    assert_eq!(compliance.into_base().to_bits(), 5.0_f64.to_bits());
}

#[test]
fn transducer_and_quadratic_flow_dimensions_are_named() {
    let pressure = Pressure::from_base(8.0_f64);
    let current = ElectricCurrent::from_base(2.0_f64);
    let gain: PressurePerElectricCurrent = pressure / current;
    let flow = VolumetricFlowRate::from_base(3.0_f64);
    let quadratic_resistance: QuadraticHydraulicResistance = pressure / (flow * flow);

    assert_eq!(gain.into_base().to_bits(), 4.0_f64.to_bits());
    assert_eq!(
        quadratic_resistance.into_base().to_bits(),
        (8.0_f64 / 9.0).to_bits()
    );
}

#[test]
fn electrical_dimensions_compose_from_base_quantities() {
    let current = ElectricCurrent::from_unit::<Ampere>(2.0_f64);
    let duration = Time::from_unit::<Second>(3.0_f64);
    let charge: ElectricCharge = current * duration;
    let potential = ElectricPotential::from_unit::<Volt>(5.0_f64);
    let capacitance: Capacitance = charge / potential;
    let conductance: ElectricConductance = current / potential;

    assert_eq!(charge.in_unit::<Coulomb>().to_bits(), 6.0_f64.to_bits());
    assert_eq!(capacitance.in_unit::<Farad>().to_bits(), 1.2_f64.to_bits());
    assert_eq!(
        conductance.in_unit::<Siemens>().to_bits(),
        0.4_f64.to_bits()
    );
}

#[test]
fn sar_uses_electrical_conductivity_and_field_magnitude() {
    let conductivity = ElectricalConductivity::from_unit::<SiemensPerMeter>(0.5_f64);
    let potential = ElectricPotential::from_unit::<Volt>(2.0_f64);
    let length = Length::from_unit::<Meter>(1.0_f64);
    let electric_field = potential / length;
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);

    let sar: SpecificAbsorptionRate = conductivity * electric_field * electric_field / density;

    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        0.002_f64.to_bits()
    );
}

#[test]
fn complex_phasors_preserve_units_and_dimension() {
    use aequitas::systems::si::{quantities::Length, units::Kilometer};
    use eunomia::Complex64;

    let phasor = Complex64::new(1.25, -2.5);
    let length: Length<Complex64> = Length::from_unit::<Kilometer>(phasor);
    assert_eq!(length.in_unit::<Kilometer>(), phasor);

    let voltage = ElectricPotential::from_base(Complex64::new(3.0, 4.0));
    let current = ElectricCurrent::from_base(Complex64::new(1.0, 0.0));
    let impedance: ElectricalImpedance<Complex64> = voltage / current;
    assert_eq!(impedance.into_base(), Complex64::new(3.0, 4.0));
}

#[test]
fn complex_unit_conversion_scales_real_and_quadrature_components() {
    use aequitas::systems::si::{
        quantities::Length,
        units::{Kilometer, Meter},
    };
    use eunomia::{Complex64, ComplexField};

    let phasor = Complex64::new(1.25, -2.5);
    let length: Length<Complex64> = Length::from_unit::<Kilometer>(phasor);
    let meters = length.in_unit::<Meter>();

    assert_eq!(meters, Complex64::new(1_250.0, -2_500.0));
    assert_eq!(ComplexField::real(meters).to_bits(), 1_250.0_f64.to_bits());
    assert_eq!(
        ComplexField::imaginary(meters).to_bits(),
        (-2_500.0_f64).to_bits()
    );
}

#[test]
fn complex_polarizability_preserves_units_and_dimension() {
    use aequitas::systems::si::units::FaradSquareMeter;
    use eunomia::Complex64;

    let alpha: Polarizability<Complex64> =
        Polarizability::from_unit::<FaradSquareMeter>(Complex64::new(2.0, -0.5));

    assert_eq!(
        alpha.in_unit::<FaradSquareMeter>(),
        Complex64::new(2.0, -0.5)
    );
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

#[test]
fn fluid_and_acoustic_dimensions_close() {
    let dynamic = DynamicViscosity::from_unit::<PascalSecond>(0.004_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let kinematic: KinematicViscosity = dynamic / density;
    let flow = VolumetricFlowRate::from_unit::<CubicMeterPerSecond>(0.002_f64);
    let area = Area::from_unit::<SquareMeter>(0.01_f64);
    let velocity: Velocity = flow / area;
    let impedance: AcousticImpedance = density * velocity;
    let intensity = Intensity::from_unit::<WattPerSquareMeter>(12.0_f64);
    let absorption = ReciprocalLength::from_unit::<PerMeter>(2.0_f64);
    let power_density: VolumetricPowerDensity = absorption * intensity;

    assert_eq!(
        kinematic.in_unit::<SquareMeterPerSecond>().to_bits(),
        4.0e-6_f64.to_bits()
    );
    assert_eq!(
        velocity.in_unit::<MeterPerSecond>().to_bits(),
        0.2_f64.to_bits()
    );
    assert_eq!(impedance.in_unit::<Rayl>().to_bits(), 200.0_f64.to_bits());
    assert_eq!(
        power_density.in_unit::<WattPerCubicMeter>().to_bits(),
        24.0_f64.to_bits()
    );
}

#[test]
fn mass_density_rate_has_pennes_dimensions() {
    let blood_perfusion = MassDensityRate::from_unit::<KilogramPerCubicMeterSecond>(0.5_f64);
    let duration = Time::from_unit::<Second>(2.0_f64);
    let exchanged_density: MassDensity = blood_perfusion * duration;

    assert_eq!(
        blood_perfusion
            .in_unit::<KilogramPerCubicMeterSecond>()
            .to_bits(),
        0.5_f64.to_bits()
    );
    assert_eq!(exchanged_density.into_base().to_bits(), 1.0_f64.to_bits());
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

#[test]
fn absolute_temperature_arithmetic_preserves_affine_semantics() {
    let lower = ThermodynamicTemperature::from_unit::<Kelvin>(290.0_f64);
    let upper = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);
    let delta: TemperatureDifference = upper - lower;

    assert_eq!(delta.in_unit::<Kelvin>().to_bits(), 10.0_f64.to_bits());
    assert_eq!(
        (lower + delta).in_unit::<Kelvin>().to_bits(),
        300.0_f64.to_bits()
    );
    assert_eq!(
        (delta + lower).in_unit::<Kelvin>().to_bits(),
        300.0_f64.to_bits()
    );
    assert_eq!(
        (upper - delta).in_unit::<Kelvin>().to_bits(),
        290.0_f64.to_bits()
    );
}

#[test]
fn power_divided_by_mass_is_specific_absorption_rate() {
    let power = Power::from_unit::<Watt>(12.0_f64);
    let mass = Mass::from_unit::<Kilogram>(3.0_f64);
    let sar: SpecificAbsorptionRate = power / mass;

    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        4.0_f64.to_bits()
    );
}

#[test]
fn absorbed_dose_divided_by_time_is_absorbed_dose_rate() {
    let dose = AbsorbedDose::from_unit::<Gray>(8.0_f64);
    let time = Time::from_unit::<Second>(2.0_f64);
    let rate: AbsorbedDoseRate = dose / time;

    assert_eq!(rate.in_unit::<GrayPerSecond>().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn absorbed_dose_rate_times_time_is_absorbed_dose() {
    let rate = AbsorbedDoseRate::from_unit::<GrayPerSecond>(2.5_f64);
    let time = Time::from_unit::<Second>(4.0_f64);
    let dose: AbsorbedDose = rate * time;

    assert_eq!(dose.in_unit::<Gray>().to_bits(), 10.0_f64.to_bits());
}

#[test]
fn watt_per_kilogram_and_gray_per_second_are_one_coherent_unit() {
    let sar = SpecificAbsorptionRate::from_unit::<WattPerKilogram>(7.5_f64);

    assert_eq!(sar.in_unit::<GrayPerSecond>().to_bits(), 7.5_f64.to_bits());
    assert_eq!(sar.into_base().to_bits(), 7.5_f64.to_bits());
}

#[test]
fn volumetric_power_density_divided_by_mass_density_is_specific_absorption_rate() {
    let deposition = VolumetricPowerDensity::from_unit::<WattPerCubicMeter>(2_000.0_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let sar: SpecificAbsorptionRate = deposition / density;

    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        2.0_f64.to_bits()
    );
}
