//! Transport and fluid identities: viscosity, flow, acoustic impedance and the
//! power-density gradients they feed.

use aequitas::systems::si::{
    quantities::{
        AcousticImpedance, Area, AreaPerTime, DynamicViscosity, Intensity, KinematicViscosity,
        Length, MassDensity, MassDensityRate, NumberDensity, ReciprocalLength,
        ReciprocalTimeSquared, Time, Velocity, VolumetricFlowRate, VolumetricPowerDensity,
        VolumetricPowerDensityGradient,
    },
    units::{
        CubicMeterPerSecond, KilogramPerCubicMeter, KilogramPerCubicMeterSecond, Meter,
        MeterPerSecond, PascalSecond, PerCubicMeter, PerMeter, Rayl, Second, SquareMeter,
        SquareMeterPerSecond, WattPerCubicMeter, WattPerMeterFourth, WattPerSquareMeter,
    },
};

#[test]
fn number_density_uses_the_entity_per_volume_contract() {
    let concentration = NumberDensity::from_unit::<PerCubicMeter>(1.0e12_f64);

    assert_eq!(concentration.into_base().to_bits(), 1.0e12_f64.to_bits());
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
