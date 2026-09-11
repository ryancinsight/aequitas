//! Conformance of the generated inventory against the law crate.
//!
//! These assert the property the whole binding rests on: a conversion made
//! through the registry equals the one Aequitas makes through `in_unit`. The
//! registry reads `LinearUnit`'s associated constants, so a divergence here
//! means the *inventory* is wrong -- a quantity mapped to the wrong dimension
//! -- not that a scale was mistyped, which the design makes unrepresentable.

use aequitas::systems::si::quantities::{Area, Energy, Length, Mass, Pressure, Time, Volume};
use aequitas::systems::si::units::{
    Centimeter, CubicMillimeter, ElectronVolt, Gram, Kilopascal, MegaElectronVolt, Megapascal,
    Microsecond, Millimeter, Millisecond, Nanometer, SquareCentimeter,
};
use aequitas::systems::si::{dimensions, units as si_units};
use aequitas::unit::LinearUnit;

use super::{all, by_name, by_tag};
use crate::tag::TaggedDimension;

/// Assert two `f64` values are bitwise identical.
///
/// The oracle here is exact equality, not a tolerance: the binding performs
/// the same `f64` operation on the same operands that Aequitas performs, so
/// evaluation order provably matches and any difference is a defect rather
/// than rounding: no reordering, no widening, no separate accumulation.
#[track_caller]
fn assert_exact(actual: f64, expected: f64) {
    assert_exact_in_context(actual, expected, "values diverge");
}

/// Assert bitwise identity, naming which case diverged.
///
/// Same oracle as [`assert_exact`]; the context distinguishes one expansion of
/// the conversion macro from the next.
#[expect(
    clippy::float_cmp,
    reason = "bitwise equality is the correct oracle for an identical operation"
)]
#[track_caller]
fn assert_exact_in_context(actual: f64, expected: f64, context: &str) {
    assert_eq!(actual, expected, "{context}");
}

#[test]
fn the_inventory_covers_every_named_quantity() {
    // 81 aliases in `systems::si::quantities`. A new alias that never reached
    // the generator fails here as well as in the generator's `check` mode.
    assert_eq!(all().len(), 81);
}

#[test]
fn quantity_names_are_unique() {
    let mut names: Vec<&str> = all().iter().map(|quantity| quantity.name).collect();
    let count = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), count, "two quantities share a Python name");
}

#[test]
fn every_scale_is_finite_and_positive() {
    for quantity in all() {
        for unit in quantity.units {
            assert!(
                unit.scale.is_finite() && unit.scale > 0.0,
                "{}::{} has scale {}",
                quantity.name,
                unit.name,
                unit.scale
            );
        }
    }
}

#[test]
fn registry_tags_match_the_type_level_dimensions() {
    assert_eq!(
        by_name("length").expect("length is registered").tag,
        <dimensions::Length as TaggedDimension>::TAG
    );
    assert_eq!(
        by_name("stress").expect("stress is registered").tag,
        <dimensions::Stress as TaggedDimension>::TAG
    );
    assert_eq!(
        by_name("thermal_diffusivity")
            .expect("thermal diffusivity is registered")
            .tag,
        <dimensions::ThermalDiffusivity as TaggedDimension>::TAG
    );
}

/// Values a conversion is checked at: a seeded sweep across many decades,
/// plus the witnesses a probe found where dividing by the scale and
/// multiplying by its reciprocal disagree.
///
/// A handful of round numbers is exactly the input set most likely to hide an
/// ulp-level difference, which is how the earlier seven-point version passed
/// while a quarter of the units disagreed with the law crate.
fn sweep() -> Vec<f64> {
    let mut values = vec![
        12.5,
        3.0,
        532.0,
        250.0,
        40.0,
        101.325,
        2.5,
        6.318_609_123_747_463e-11,
        -9.823_474_589_799_991e-2,
        1.097_804_469_310_134_3,
        3.037_926_988_917_369_6e-4,
    ];
    // xorshift64: deterministic, dependency-free, and spread over 24 decades.
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..4096 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let mantissa =
            f64::from(u32::try_from(state >> 40).expect("24 bits fit")) / f64::from(1_u32 << 24);
        let exponent = i32::try_from(state % 25).expect("under 25") - 12;
        let sign = if state & 1 == 0 { 1.0 } else { -1.0 };
        values.push(sign * (1.0 + mantissa) * 10_f64.powi(exponent));
    }
    values
}

/// Assert that the registry's conversions equal Aequitas's, both directions.
///
/// Calls `Unit::to_base` and `Unit::from_base` -- the code `model.rs` runs --
/// rather than re-deriving the arithmetic, which would test nothing about the
/// binding.
macro_rules! assert_conversion_matches {
    ($quantity:literal, $alias:ident, $unit:ty, $symbol:literal) => {{
        let named = by_name($quantity).expect("quantity is registered");
        let unit = named.unit($symbol).expect("unit is registered");

        for value in sweep() {
            let through_registry = unit.to_base(value);
            let through_aequitas = $alias::<f64>::from_unit::<$unit>(value).into_base();
            assert_exact_in_context(
                through_registry,
                through_aequitas,
                &format!(
                    "{} {} into base diverges from Aequitas at {value:e}",
                    $quantity, $symbol
                ),
            );

            let back_through_registry = unit.from_base(through_aequitas);
            let back_through_aequitas =
                $alias::<f64>::from_base(through_aequitas).in_unit::<$unit>();
            assert_exact_in_context(
                back_through_registry,
                back_through_aequitas,
                &format!(
                    "{} {} out of base diverges from Aequitas at {value:e}",
                    $quantity, $symbol
                ),
            );
        }
    }};
}

#[test]
fn conversions_agree_with_the_law_crate() {
    assert_conversion_matches!("length", Length, Millimeter, "mm");
    assert_conversion_matches!("length", Length, Centimeter, "cm");
    assert_conversion_matches!("length", Length, Nanometer, "nm");
    assert_conversion_matches!("mass", Mass, Gram, "g");
    assert_conversion_matches!("time", Time, Millisecond, "ms");
    assert_conversion_matches!("time", Time, Microsecond, "\u{b5}s");
    assert_conversion_matches!("pressure", Pressure, Kilopascal, "kPa");
    assert_conversion_matches!("pressure", Pressure, Megapascal, "MPa");
    assert_conversion_matches!("energy", Energy, ElectronVolt, "eV");
    assert_conversion_matches!("energy", Energy, MegaElectronVolt, "MeV");
    assert_conversion_matches!("area", Area, SquareCentimeter, "cm\u{b2}");
    assert_conversion_matches!("volume", Volume, CubicMillimeter, "mm\u{b3}");
}

#[test]
fn a_shared_unit_marker_resolves_per_dimension() {
    // `Pascal` implements `LinearUnit` for both `Pressure` and `Stress`; the
    // registry must offer it under both without merging the two quantities.
    let pressure = by_name("pressure").expect("pressure is registered");
    let stress = by_name("stress").expect("stress is registered");

    assert!(pressure.unit("Pa").is_some());
    assert!(stress.unit("Pa").is_some());
    assert_ne!(pressure.tag, stress.tag);
    assert_exact(
        <si_units::Pascal as LinearUnit<dimensions::Pressure>>::SCALE,
        pressure.unit("Pa").expect("Pa is registered").scale,
    );
    assert_exact(
        <si_units::Pascal as LinearUnit<dimensions::Stress>>::SCALE,
        stress.unit("Pa").expect("Pa is registered").scale,
    );
}

#[test]
fn kelvin_serves_both_temperature_quantities_without_merging_them() {
    let absolute = by_name("thermodynamic_temperature").expect("registered");
    let difference = by_name("temperature_difference").expect("registered");

    assert!(absolute.unit("K").is_some());
    assert!(difference.unit("K").is_some());
    assert_ne!(absolute.tag, difference.tag);
}

#[test]
fn a_unit_lookup_accepts_symbol_or_marker_name() {
    let length = by_name("length").expect("length is registered");
    let by_symbol = length.unit("mm").expect("mm resolves");
    let by_marker = length.unit("Millimeter").expect("Millimeter resolves");

    assert_exact(by_symbol.scale, by_marker.scale);
    assert_eq!(by_symbol.name, "Millimeter");
}

#[test]
fn an_unknown_unit_does_not_resolve() {
    let length = by_name("length").expect("length is registered");
    assert!(length.unit("furlong").is_none());
    // A unit of another dimension is equally unresolvable here.
    assert!(length.unit("Pa").is_none());
}

#[test]
fn a_dimension_may_carry_several_names() {
    // Seven dimensions in the inventory are named more than once. The registry
    // must report all of them rather than presenting the first as identity --
    // `m^2/s` is equally a thermal diffusivity and a kinematic viscosity.
    let diffusivity = by_name("thermal_diffusivity").expect("registered");
    let names: Vec<&str> = super::names_for_tag(diffusivity.tag).collect();

    assert_eq!(
        names,
        [
            "area_per_time",
            "thermal_diffusivity",
            "kinematic_viscosity"
        ]
    );
    assert_eq!(
        by_tag(diffusivity.tag).map(|found| found.name),
        Some("area_per_time")
    );
}

#[test]
fn an_unnamed_dimension_carries_no_names() {
    let exotic =
        crate::tag::DimensionTag::new([4, 0, 0, 0, 0, 0, 0], crate::tag::SemanticTag::Base);
    assert_eq!(super::names_for_tag(exotic).count(), 0);
    assert!(by_tag(exotic).is_none());
}

#[test]
fn tag_lookup_finds_a_name_for_a_registered_dimension() {
    let length = by_name("length").expect("length is registered");
    assert_eq!(by_tag(length.tag).map(|found| found.name), Some("length"));
}

#[test]
fn dimensionless_carries_no_linear_units() {
    // No `LinearUnit` impl targets `Dimensionless`: a bare ratio has no unit
    // to convert through, and construction goes through base values.
    let scalar = by_name("dimensionless").expect("dimensionless is registered");
    assert!(scalar.units.is_empty());
}
