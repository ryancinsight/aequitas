//! Complex-valued quantity and unit-conversion identities.
//!
//! A complex value is an axis of its own rather than a domain: the unit legs of
//! these identities are electrical, mechanical and thermal, but what each
//! asserts is that a marker's scale and the dimension algebra act on the real
//! and quadrature components alike. Keeping them together is what makes that
//! one claim readable in one place.

use aequitas::systems::si::{
    quantities::{
        ElectricCurrent, ElectricPotential, ElectricalImpedance, FlexuralRigidity, Length,
        MechanicalImpedance, Polarizability, VelocityPerTemperature, VolumeChargeDensity,
    },
    units::{
        CoulombPerCubicMeter, FaradSquareMeter, Joule, KilogramPerSecond, Kilometer, Meter,
        MeterPerSecondKelvin,
    },
};
use eunomia::{Complex64, ComplexField};

#[test]
fn mechanical_impedance_keeps_its_force_per_velocity_unit_for_complex_values() {
    use eunomia::Complex64;

    let impedance = MechanicalImpedance::from_unit::<KilogramPerSecond>(Complex64::new(2.0, -3.0));

    assert_eq!(
        impedance.in_unit::<KilogramPerSecond>(),
        Complex64::new(2.0, -3.0)
    );
}

#[test]
fn mems_quantities_preserve_complex_unit_components() {
    use eunomia::Complex64;

    let charge_density =
        VolumeChargeDensity::from_unit::<CoulombPerCubicMeter>(Complex64::new(1.5, -0.25));
    let rigidity = FlexuralRigidity::from_unit::<Joule>(Complex64::new(2.0, -0.5));

    assert_eq!(
        charge_density.in_unit::<CoulombPerCubicMeter>(),
        Complex64::new(1.5, -0.25)
    );
    assert_eq!(rigidity.in_unit::<Joule>(), Complex64::new(2.0, -0.5));
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
fn complex_phasors_preserve_units_and_dimension() {
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
    let alpha: Polarizability<Complex64> =
        Polarizability::from_unit::<FaradSquareMeter>(Complex64::new(2.0, -0.5));

    assert_eq!(
        alpha.in_unit::<FaradSquareMeter>(),
        Complex64::new(2.0, -0.5)
    );
}
