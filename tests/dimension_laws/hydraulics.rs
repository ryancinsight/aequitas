//! Hydraulic identities: the named result dimensions of pressure, flow and
//! applied current, all built from base values.

use aequitas::systems::si::quantities::{
    Compliance, ElectricCurrent, HydraulicInertance, HydraulicResistance, Length, Pressure,
    PressureGradient, PressurePerElectricCurrent, QuadraticHydraulicResistance, Time, Volume,
    VolumetricFlowRate,
};

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
