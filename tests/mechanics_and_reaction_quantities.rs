//! Type-identity and dimensional-algebra regressions for the mechanics and
//! reaction quantities that Ares and Prometheus consume.
//!
//! Charters: atlas ADR 0057 (Ares) and ADR 0058 (Prometheus). Both type every
//! physical value on a public boundary, so these quantities land before either
//! package exists — retrofitting a semantics marker after a public API ships
//! is a breaking change.

use core::any::TypeId;

use aequitas::systems::si::dimensions;
use aequitas::systems::si::quantities::{
    MolarConcentration, MolarFlux, Pressure, ReactionRate, Stress, Time,
};
use aequitas::systems::si::units::{
    Megapascal, MolePerCubicMeterSecond, MolePerSquareMeterSecond, Pascal,
};

// ---------------------------------------------------------------------------
// Stress is dimensionally pressure and semantically distinct
// ---------------------------------------------------------------------------

#[test]
fn stress_and_pressure_are_distinct_types() {
    // The whole point of the marker: a modulus or a hydrostatic pressure must
    // not be assignable where a stress is meant, even though the SI exponents
    // agree.
    assert_ne!(
        TypeId::of::<dimensions::Stress>(),
        TypeId::of::<dimensions::Pressure>()
    );
    assert_ne!(TypeId::of::<Stress<f64>>(), TypeId::of::<Pressure<f64>>());
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "values round-trip through from_base/into_base, and the divided cases (2/4, 6/3) are exactly representable in binary, so exact equality is the correct assertion"
)]
fn stress_converts_through_pressure_units() {
    // Distinct identity must not mean an unusable quantity. Stress carries its
    // own LinearUnit impls on the pascal family, exactly as Kelvin carries them
    // for both ThermodynamicTemperature and TemperatureDifference. Testing via
    // from_base would bypass the unit contract entirely.
    let yield_strength = Stress::<f64>::from_unit::<Megapascal>(250.0);
    assert_eq!(yield_strength.into_base(), 250.0e6);

    // Round trip through a different scale on the same family.
    assert_eq!(yield_strength.in_unit::<Pascal>(), 250.0e6);
    assert_eq!(yield_strength.in_unit::<Megapascal>(), 250.0);

    // Same numeric magnitude as the pressure reading it is not interchangeable
    // with: the separation is in the type, never in the value.
    let pressure = Pressure::<f64>::from_unit::<Megapascal>(250.0);
    assert_eq!(yield_strength.into_base(), pressure.into_base());
}

#[expect(
    clippy::float_cmp,
    reason = "coherent-unit scales are exactly 1.0, so the conversion is exact"
)]
#[test]
fn reaction_quantities_convert_through_their_coherent_units() {
    // A dimension with no unit impl is reachable only through from_base, which
    // bypasses the unit contract. Both new dimensions carry their coherent SI
    // unit so consumers state values in units rather than in base scalars.
    let rate = ReactionRate::<f64>::from_unit::<MolePerCubicMeterSecond>(2.5);
    assert_eq!(rate.into_base(), 2.5);
    assert_eq!(rate.in_unit::<MolePerCubicMeterSecond>(), 2.5);

    let flux = MolarFlux::<f64>::from_unit::<MolePerSquareMeterSecond>(0.75);
    assert_eq!(flux.into_base(), 0.75);
    assert_eq!(flux.in_unit::<MolePerSquareMeterSecond>(), 0.75);
}

#[test]
fn stress_markers_are_zero_sized() {
    assert_eq!(size_of::<dimensions::Stress>(), 0);
    assert_eq!(size_of::<Stress<f64>>(), size_of::<f64>());
}

// ---------------------------------------------------------------------------
// Reaction quantities close the species-balance algebra
// ---------------------------------------------------------------------------

#[test]
#[expect(
    clippy::float_cmp,
    reason = "values round-trip through from_base/into_base, and the divided cases (2/4, 6/3) are exactly representable in binary, so exact equality is the correct assertion"
)]
fn concentration_over_time_is_a_reaction_rate() {
    // The identity a species balance relies on: d[c]/dt has reaction-rate
    // dimension. Aequitas proves it at the type level; this asserts the
    // exponents agree so the proof is about the right thing.
    let concentration = MolarConcentration::<f64>::from_base(2.0);
    let time = Time::<f64>::from_base(4.0);
    let quotient = concentration / time;

    let expected = ReactionRate::<f64>::from_base(0.5);
    assert_eq!(quotient.into_base(), expected.into_base());
    assert_eq!(
        TypeId::of::<dimensions::ReactionRate>(),
        TypeId::of::<
            <dimensions::MolarConcentration as aequitas::dimension::DivideDimension<
                dimensions::Time,
            >>::Output,
        >()
    );
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "values round-trip through from_base/into_base, and the divided cases (2/4, 6/3) are exactly representable in binary, so exact equality is the correct assertion"
)]
fn molar_flux_over_length_is_a_reaction_rate() {
    // The divergence relation: a flux differentiated across a length is a
    // volumetric source, which is what the transport consumer assembles.
    let flux = MolarFlux::<f64>::from_base(6.0);
    let length = aequitas::systems::si::quantities::Length::<f64>::from_base(3.0);
    let quotient = flux / length;

    assert_eq!(quotient.into_base(), 2.0);
    assert_eq!(
        TypeId::of::<dimensions::ReactionRate>(),
        TypeId::of::<<dimensions::MolarFlux as aequitas::dimension::DivideDimension<
            dimensions::Length,
        >>::Output>()
    );
}

#[test]
fn reaction_quantities_are_mutually_distinct() {
    assert_ne!(
        TypeId::of::<dimensions::ReactionRate>(),
        TypeId::of::<dimensions::MolarFlux>()
    );
    assert_ne!(
        TypeId::of::<dimensions::ReactionRate>(),
        TypeId::of::<dimensions::MolarConcentration>()
    );
}

#[test]
fn reaction_quantities_are_transparent_over_their_scalar() {
    assert_eq!(size_of::<ReactionRate<f64>>(), size_of::<f64>());
    assert_eq!(size_of::<MolarFlux<f32>>(), size_of::<f32>());
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "values round-trip through from_base/into_base, and the divided cases (2/4, 6/3) are exactly representable in binary, so exact equality is the correct assertion"
)]
fn reaction_quantities_are_generic_over_the_supported_scalars() {
    // Fake-generic guard: the same construction must work at every shipped
    // scalar, not only the f64 default.
    let wide = ReactionRate::<f64>::from_base(1.5);
    let narrow = ReactionRate::<f32>::from_base(1.5);
    assert_eq!(wide.into_base(), 1.5_f64);
    assert_eq!(narrow.into_base(), 1.5_f32);
}
