//! Affine temperature scales, against published fixed points.
//!
//! The reference values are definitional rather than measured: the Celsius
//! scale is defined against the kelvin by `t/°C = T/K − 273.15` (SI Brochure,
//! 9th edition, §2.3.1), and the Fahrenheit scale by `t/°F = T/K × 9/5 −
//! 459.67`. Every expectation below follows from those two identities, so a
//! failure is a defect in the conversion rather than a disagreement about
//! measurement.

use aequitas::systems::si::{
    quantities::{TemperatureDifference, ThermodynamicTemperature},
    units::{DegreeCelsius, DegreeFahrenheit, Kelvin},
};

/// Absolute tolerance for a single conversion, at a base-unit magnitude.
///
/// One multiplication and one addition are two elementary roundings, and the
/// offset's own representation contributes a third; the factor of eight covers
/// their composition.
fn bound(kelvin: f64) -> f64 {
    8.0 * f64::EPSILON * kelvin.abs().max(1.0)
}

/// Absolute tolerance for a round trip, in the unit's own degrees.
///
/// The result's magnitude is the wrong scale to measure this against. A round
/// trip passes through `value × SCALE + OFFSET`, and near the bottom of a scale
/// whose zero is far from the base unit's, those two terms very nearly cancel:
/// -459.67 degF is absolute zero, so the base value is about 1e-14 while the
/// terms that produced it are about 255 K each. The rounding error is set by
/// the terms, not by what survives them.
///
/// So the error admitted at the base is `eps * max(|value * SCALE|, |OFFSET|)`,
/// and converting back divides by `SCALE`, giving
/// `eps * max(|value|, |OFFSET| / SCALE)` in the unit's degrees. `zero_point`
/// is that `|OFFSET| / SCALE`: how far the unit's zero sits below the base
/// unit's, measured in the unit's own degrees — 273.15 for Celsius and 459.67
/// for Fahrenheit.
fn round_trip_bound(reading: f64, zero_point: f64) -> f64 {
    8.0 * f64::EPSILON * reading.abs().max(zero_point).max(1.0)
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "exact: SCALE is 1.0, so 0 degC yields the OFFSET constant itself               with no arithmetic to round. A tolerance here would accept a               perturbed ice point."
)]
fn celsius_zero_is_the_ice_point_in_kelvin() {
    let freezing = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeCelsius>(0.0);
    assert_eq!(
        freezing.in_unit::<Kelvin>(),
        273.15,
        "0 degC is 273.15 K by the definition of the Celsius scale"
    );
}

#[test]
fn celsius_hundred_is_the_steam_point_in_kelvin() {
    let boiling = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeCelsius>(100.0);
    assert!(
        (boiling.in_unit::<Kelvin>() - 373.15).abs() <= bound(373.15),
        "100 degC is 373.15 K, got {}",
        boiling.in_unit::<Kelvin>()
    );
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "exact: (0 - OFFSET) * 1.0 is the negated constant, representable"
)]
fn absolute_zero_reads_minus_273_15_celsius() {
    let absolute_zero = ThermodynamicTemperature::<f64>::from_unit::<Kelvin>(0.0);
    let celsius = absolute_zero.in_affine_unit::<DegreeCelsius>();
    assert_eq!(celsius, -273.15, "0 K is -273.15 degC");
}

#[test]
fn fahrenheit_fixed_points_match_the_celsius_ones() {
    let freezing = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeFahrenheit>(32.0);
    let boiling = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeFahrenheit>(212.0);

    assert!(
        (freezing.in_unit::<Kelvin>() - 273.15).abs() <= bound(273.15),
        "32 degF is the ice point, got {}",
        freezing.in_unit::<Kelvin>()
    );
    assert!(
        (boiling.in_unit::<Kelvin>() - 373.15).abs() <= bound(373.15),
        "212 degF is the steam point, got {}",
        boiling.in_unit::<Kelvin>()
    );
}

#[test]
fn the_scales_cross_at_minus_forty() {
    // An oracle independent of either offset: the Celsius and Fahrenheit
    // readings coincide at exactly one temperature, and it is -40.
    let from_celsius = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeCelsius>(-40.0);
    let from_fahrenheit =
        ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeFahrenheit>(-40.0);

    let difference = (from_celsius.in_unit::<Kelvin>() - from_fahrenheit.in_unit::<Kelvin>()).abs();
    assert!(
        difference <= bound(233.15),
        "-40 degC and -40 degF are the same temperature; they differed by {difference}"
    );
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "exact by construction, and the point of the test: a Celsius               difference multiplies by 1.0 and adds nothing, so any tolerance               would be room for an offset to hide in"
)]
fn a_celsius_difference_carries_no_offset() {
    // The property the whole design exists for. Ten degrees Celsius warmer is
    // ten kelvin warmer. Were the offset applied here, this would read
    // 283.15 K and every temperature difference in the system would be wrong
    // by the ice point.
    let warmer = TemperatureDifference::<f64>::from_unit::<DegreeCelsius>(10.0);
    assert_eq!(
        warmer.in_unit::<Kelvin>(),
        10.0,
        "a Celsius difference is a kelvin difference"
    );
}

#[test]
fn a_fahrenheit_difference_carries_the_degree_size_and_no_offset() {
    let warmer = TemperatureDifference::<f64>::from_unit::<DegreeFahrenheit>(9.0);
    let expected = 5.0;
    assert!(
        (warmer.in_unit::<Kelvin>() - expected).abs() <= bound(expected),
        "nine Fahrenheit degrees are five kelvin, got {}",
        warmer.in_unit::<Kelvin>()
    );
}

#[test]
fn a_difference_and_a_temperature_disagree_by_exactly_the_offset() {
    // Stated as a relationship rather than two separate numbers, so the test
    // fails if either conversion drifts toward the other.
    let as_temperature = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeCelsius>(25.0)
        .in_unit::<Kelvin>();
    let as_difference =
        TemperatureDifference::<f64>::from_unit::<DegreeCelsius>(25.0).in_unit::<Kelvin>();

    assert!(
        ((as_temperature - as_difference) - 273.15).abs() <= bound(273.15),
        "the two readings of 25 degC must differ by the ice point exactly: \
         temperature={as_temperature}, difference={as_difference}"
    );
}

#[test]
fn celsius_round_trips_through_kelvin() {
    for reading in [-273.15_f64, -40.0, 0.0, 21.5, 100.0, 5000.0] {
        let recovered = ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeCelsius>(reading)
            .in_affine_unit::<DegreeCelsius>();
        assert!(
            (recovered - reading).abs() <= round_trip_bound(reading, 273.15),
            "round trip lost {reading}: recovered {recovered}"
        );
    }
}

#[test]
fn fahrenheit_round_trips_through_kelvin() {
    for reading in [-459.67_f64, -40.0, 32.0, 98.6, 212.0, 9000.0] {
        let recovered =
            ThermodynamicTemperature::<f64>::from_affine_unit::<DegreeFahrenheit>(reading)
                .in_affine_unit::<DegreeFahrenheit>();
        assert!(
            (recovered - reading).abs() <= round_trip_bound(reading, 459.67),
            "round trip lost {reading}: recovered {recovered}"
        );
    }
}

#[test]
fn f32_conversions_stay_in_f32() {
    // The conversion must not widen: the offset is materialised through
    // `T::ONE`, so it arrives in the scalar's own precision.
    let warm = aequitas::Quantity::<f32, aequitas::systems::si::dimensions::ThermodynamicTemperature>
        ::from_affine_unit::<DegreeCelsius>(25.0_f32);
    let kelvin: f32 = warm.in_unit::<Kelvin>();
    assert!(
        (kelvin - 298.15_f32).abs() <= 8.0 * f32::EPSILON * 298.15,
        "25 degC is 298.15 K in f32, got {kelvin}"
    );
}
