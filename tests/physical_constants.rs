//! Dimensional and value identities for the shared physical constants.

mod support;

use aequitas::systems::si::{
    constants::{
        AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT, ELECTRON_MASS, ELEMENTARY_CHARGE,
        MOLAR_GAS_CONSTANT, PLANCK_CONSTANT, REDUCED_PLANCK_CONSTANT, SPEED_OF_LIGHT,
        STANDARD_ATMOSPHERIC_PRESSURE, STANDARD_GRAVITY, STEFAN_BOLTZMANN_CONSTANT,
        VACUUM_IMPEDANCE, VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY,
    },
    quantities::{
        Acceleration, ElectricCharge, ElectricalImpedance, Energy, Frequency, Intensity,
        MolarHeatCapacity, ThermodynamicTemperature, Velocity,
    },
    units::{
        Coulomb, Hertz, Joule, Kelvin, MeterPerSecondSquared, StandardAtmosphere,
        WattPerSquareMeter,
    },
};
use typenum::P4;

#[test]
fn defining_constants_keep_their_dimensions_in_formulas() {
    let _subscriber = support::trace_phases();
    let frequency = tracing::info_span!("arrange").in_scope(|| Frequency::from_unit::<Hertz>(2.0));
    let (photon_energy, molar): (Energy, MolarHeatCapacity) =
        tracing::info_span!("act").in_scope(|| {
            (
                PLANCK_CONSTANT * frequency,
                AVOGADRO_CONSTANT * BOLTZMANN_CONSTANT,
            )
        });
    let _assert = tracing::info_span!("assert").entered();
    assert_eq!(
        photon_energy.in_unit::<Joule>().to_bits(),
        1.325_214_03e-33_f64.to_bits()
    );

    let charge: ElectricCharge = ELEMENTARY_CHARGE;
    assert_eq!(
        charge.in_unit::<Coulomb>().to_bits(),
        1.602_176_634e-19_f64.to_bits()
    );

    assert!(
        (*REDUCED_PLANCK_CONSTANT.as_base() * core::f64::consts::TAU - PLANCK_CONSTANT.into_base())
            .abs()
            <= f64::EPSILON * PLANCK_CONSTANT.into_base()
    );
    // Both decimal constants are rounded once to binary64 and multiplied once.
    let error = (molar.into_base() - 8.314_462_618_153_24).abs();
    assert!(error <= 3.0 * f64::EPSILON * molar.into_base());
    assert!(
        (MOLAR_GAS_CONSTANT.into_base() - molar.into_base()).abs()
            <= 3.0 * f64::EPSILON * molar.into_base()
    );
}

#[test]
fn reference_values_convert_and_radiant_flux_type_checks() {
    let _subscriber = support::trace_phases();
    let temperature = tracing::info_span!("arrange")
        .in_scope(|| ThermodynamicTemperature::from_unit::<Kelvin>(300.0));
    let flux: Intensity = tracing::info_span!("act")
        .in_scope(|| STEFAN_BOLTZMANN_CONSTANT * temperature.powi::<P4>());
    let _assert = tracing::info_span!("assert").entered();
    let gravity: Acceleration = STANDARD_GRAVITY;
    assert_eq!(
        gravity.in_unit::<MeterPerSecondSquared>().to_bits(),
        9.806_65_f64.to_bits()
    );
    assert_eq!(
        STANDARD_ATMOSPHERIC_PRESSURE
            .in_unit::<StandardAtmosphere>()
            .to_bits(),
        1.0_f64.to_bits()
    );

    assert!(
        (flux.in_unit::<WattPerSquareMeter>() - 459.300_327_953_939).abs()
            <= 4.0 * f64::EPSILON * 459.300_327_953_939
    );
}

#[test]
fn vacuum_constants_recover_electromagnetic_wave_speed() {
    let _subscriber = support::trace_phases();
    let (mu, epsilon, light_speed) = tracing::info_span!("arrange")
        .in_scope(|| (VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY, SPEED_OF_LIGHT));
    let (speed, impedance): (Velocity, ElectricalImpedance) = tracing::info_span!("act")
        .in_scope(|| ((mu * epsilon).sqrt().reciprocal(), mu * light_speed));
    let _assert = tracing::info_span!("assert").entered();
    // The source rounds each vacuum coefficient to 11 significant digits.
    // Propagate both half-last-digit bounds through (mu * epsilon)^(-1/2).
    let bound = SPEED_OF_LIGHT.into_base()
        * (0.5e-17 / VACUUM_PERMEABILITY.into_base()
            + 0.5e-22 / VACUUM_PERMITTIVITY.into_base()
            + 8.0 * f64::EPSILON);
    assert!((speed.into_base() - light_speed.into_base()).abs() <= bound);

    // Half a decimal ulp at each published coefficient, plus roundoff.
    let impedance_bound =
        0.5e-9 + 0.5e-17 * light_speed.into_base() + 2.0 * f64::EPSILON * impedance.into_base();
    assert!((impedance.into_base() - VACUUM_IMPEDANCE.into_base()).abs() <= impedance_bound);
    assert_eq!(
        ELECTRON_MASS.into_base().to_bits(),
        9.109_383_713_9e-31_f64.to_bits()
    );
}

#[test]
fn materializing_a_constant_keeps_its_dimension() {
    use aequitas::systems::si::quantities::Pressure;
    use aequitas::systems::si::units::Pascal;

    let _subscriber = support::trace_phases();
    let source = tracing::info_span!("arrange").in_scope(|| STANDARD_ATMOSPHERIC_PRESSURE);
    let pressure: Pressure<f32> = tracing::info_span!("act").in_scope(|| source.cast());
    let _assert = tracing::info_span!("assert").entered();
    assert_eq!(
        pressure.in_unit::<Pascal>().to_bits(),
        101_325.0_f32.to_bits()
    );
}
