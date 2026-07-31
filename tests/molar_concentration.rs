//! Provider conversion and type-identity regressions.

use aequitas::systems::si::quantities::{Length, MolarConcentration, NumberDensity};
use aequitas::systems::si::units::{MicromolePerLiter, MolePerCubicMeter, Nanometer};

fn assert_conversion(actual: f64, expected: f64) {
    // One unit-scale multiplication contributes at most a small multiple of
    // machine epsilon at the tested magnitude.
    let tolerance = 2.0 * f64::EPSILON * expected.abs().max(1.0);
    assert!((actual - expected).abs() <= tolerance);
}

#[test]
fn molar_concentration_units_preserve_si_base_values() {
    let threshold = MolarConcentration::from_unit::<MicromolePerLiter>(10.0);
    assert_conversion(threshold.into_base(), 0.01);

    let coherent = MolarConcentration::from_unit::<MolePerCubicMeter>(0.01);
    assert_conversion(coherent.into_base(), 0.01);
}

#[test]
fn nanometer_converts_optical_wavelength_to_metres() {
    let wavelength = Length::from_unit::<Nanometer>(532.0);
    assert_conversion(wavelength.into_base(), 532.0e-9);
}

#[test]
fn molar_concentration_has_distinct_type_identity() {
    fn accepts_molar(_: MolarConcentration) {}
    fn accepts_number_density(_: NumberDensity) {}

    accepts_molar(MolarConcentration::from_base(1.0));
    accepts_number_density(NumberDensity::from_base(1.0));
    assert_eq!(
        core::mem::size_of::<MolarConcentration>(),
        core::mem::size_of::<f64>()
    );
}
