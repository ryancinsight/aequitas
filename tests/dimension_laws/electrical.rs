//! Electrical identities: the base-quantity composition of charge,
//! capacitance and conductance, and the dosimetric rate built from them.

use aequitas::systems::si::{
    quantities::{
        Capacitance, ElectricCharge, ElectricConductance, ElectricCurrent, ElectricPotential,
        ElectricalConductivity, Length, MassDensity, SpecificAbsorptionRate, Time,
    },
    units::{
        Ampere, Coulomb, Farad, KilogramPerCubicMeter, Meter, Second, Siemens, SiemensPerMeter,
        Volt, WattPerKilogram,
    },
};

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
