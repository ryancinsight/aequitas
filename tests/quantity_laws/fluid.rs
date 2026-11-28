//! Fluid-law values: Reynolds, Hagen–Poiseuille, Stokes, Darcy–Weisbach,
//! Young–Laplace, Womersley and the volumetric-flow identity.

use aequitas::systems::si::{
    quantities::{
        AngularVelocity, Area, Dimensionless, DynamicViscosity, Force, Length, MassDensity,
        Pressure, Velocity, VolumetricFlowRate,
    },
    units::{Hertz, KilogramPerCubicMeter, Meter, MeterPerSecond, Pascal, PascalSecond},
};

#[test]
fn reynolds_number_of_water_in_a_pipe() {
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(1.0_f64);
    let length = Length::from_unit::<Meter>(0.1_f64);
    let viscosity = DynamicViscosity::from_unit::<PascalSecond>(1.0e-3_f64);

    let re: Dimensionless = density.reynolds_number(velocity, length, viscosity);

    assert_eq!(re.into_base().to_bits(), 100_000.0_f64.to_bits());
}

#[test]
fn hagen_poiseuille_drop_matches_closed_form() {
    let viscosity = DynamicViscosity::from_unit::<PascalSecond>(1.0e-3_f64);
    let length = Length::from_unit::<Meter>(1.0_f64);
    let flow = VolumetricFlowRate::from_base(1.0e-6_f64);
    let diameter = Length::from_unit::<Meter>(0.01_f64);

    let drop = viscosity.hagen_poiseuille_drop(length, flow, diameter);
    let want = 128.0 * 1.0e-3 * 1.0 * 1.0e-6 / (core::f64::consts::PI * 0.01_f64.powi(4));

    assert!((drop.into_base() - want).abs() <= 1e-12 * want.abs());
}

#[test]
fn stokes_drag_matches_closed_form() {
    let viscosity = DynamicViscosity::from_unit::<PascalSecond>(1.0e-3_f64);
    let radius = Length::from_unit::<Meter>(1.0e-6_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(1.0e-3_f64);

    let drag: Force = viscosity.stokes_drag(radius, velocity);
    let want = 6.0 * core::f64::consts::PI * 1.0e-3 * 1.0e-6 * 1.0e-3;

    assert_eq!(drag.into_base().to_bits(), want.to_bits());
}

#[test]
fn darcy_weisbach_drop_matches_closed_form() {
    let friction = Dimensionless::from_base(0.02_f64);
    let length = Length::from_unit::<Meter>(10.0_f64);
    let diameter = Length::from_unit::<Meter>(0.1_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(2.0_f64);

    let drop = Pressure::darcy_weisbach_drop(friction, length, diameter, density, velocity);
    let want: f64 = 0.02 * (10.0 / 0.1) * 1_000.0 * 4.0 / 2.0;

    assert_eq!(drop.into_base().to_bits(), want.to_bits());
    assert_eq!(drop.in_unit::<Pascal>().to_bits(), want.to_bits());
}

#[test]
fn young_laplace_pressure_matches_closed_form() {
    use aequitas::systems::si::{quantities::SurfaceTension, units::NewtonPerMeter};
    let tension = SurfaceTension::from_unit::<NewtonPerMeter>(0.072_f64);
    let radius = Length::from_unit::<Meter>(2.0e-3_f64);

    let pressure: Pressure = tension.young_laplace_pressure(radius);

    assert_eq!(pressure.in_unit::<Pascal>().to_bits(), 72.0_f64.to_bits());
}

#[test]
fn womersley_number_matches_closed_form() {
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_060.0_f64);
    let radius = Length::from_unit::<Meter>(0.01_f64);
    let omega =
        AngularVelocity::from_frequency(aequitas::systems::si::quantities::Frequency::from_unit::<
            Hertz,
        >(1.0_f64));
    let viscosity = DynamicViscosity::from_unit::<PascalSecond>(4.0e-3_f64);

    let wo: Dimensionless = density.womersley_number(radius, omega, viscosity);
    let want = 0.01 * (2.0 * core::f64::consts::PI * 1_060.0 / 4.0e-3).sqrt();

    assert!((wo.into_base() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn volumetric_flow_is_velocity_times_area() {
    use aequitas::systems::si::units::{CubicMeterPerSecond, SquareMeter};
    let area = Area::from_unit::<SquareMeter>(0.01_f64);
    let velocity = Velocity::from_unit::<MeterPerSecond>(0.2_f64);

    let flow: VolumetricFlowRate = area.volumetric_flow(velocity);

    assert_eq!(
        flow.in_unit::<CubicMeterPerSecond>().to_bits(),
        0.002_f64.to_bits()
    );
}
