//! Complex-valued quantity and unit-conversion identities.

use aequitas::systems::si::{
    quantities::{ElectricCurrent, ElectricPotential, ElectricalImpedance, Length, Polarizability},
    units::{FaradSquareMeter, Kilometer, Meter},
};
use eunomia::{Complex64, ComplexField};

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
