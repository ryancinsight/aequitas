//! Conformance of the generated inventory against the law crate.
//!
//! These assert the property the whole binding rests on: a conversion made
//! through the registry equals the one Aequitas makes through `in_unit`. The
//! registry reads `LinearUnit`'s associated constants, so a divergence here
//! means the *inventory* is wrong -- a quantity mapped to the wrong dimension
//! -- not that a scale was mistyped, which the design makes unrepresentable.

use aequitas::systems::si::quantities::{Length, Mass, Pressure, Time};
use aequitas::systems::si::units::{
    Centimeter, Gram, Kilopascal, Megapascal, Millimeter, Millisecond, Nanometer,
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

/// Assert that a registry conversion equals the Aequitas conversion.
macro_rules! assert_conversion_matches {
    ($quantity:literal, $alias:ident, $unit:ty, $symbol:literal, $value:expr) => {{
        let named = by_name($quantity).expect("quantity is registered");
        let unit = named.unit($symbol).expect("unit is registered");

        let through_registry = $value * unit.scale;
        let through_aequitas = $alias::<f64>::from_unit::<$unit>($value).into_base();
        assert_exact_in_context(
            through_registry,
            through_aequitas,
            &format!("{} in {} diverges from Aequitas", $quantity, $symbol),
        );

        let back_through_registry = through_registry / unit.scale;
        let back_through_aequitas = $alias::<f64>::from_base(through_aequitas).in_unit::<$unit>();
        assert_exact(back_through_registry, back_through_aequitas);
    }};
}

#[test]
fn conversions_agree_with_the_law_crate() {
    assert_conversion_matches!("length", Length, Millimeter, "mm", 12.5);
    assert_conversion_matches!("length", Length, Centimeter, "cm", 3.0);
    assert_conversion_matches!("length", Length, Nanometer, "nm", 532.0);
    assert_conversion_matches!("mass", Mass, Gram, "g", 250.0);
    assert_conversion_matches!("time", Time, Millisecond, "ms", 40.0);
    assert_conversion_matches!("pressure", Pressure, Kilopascal, "kPa", 101.325);
    assert_conversion_matches!("pressure", Pressure, Megapascal, "MPa", 2.5);
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
