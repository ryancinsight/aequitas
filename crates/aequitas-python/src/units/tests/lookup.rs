//! Unit lookup, and the aliasing it has to survive.
//!
//! A symbol and a marker name are two spellings of one unit, and one
//! dimension may carry several quantity names. The contract is that a lookup
//! resolves what is registered -- under every spelling, without merging two
//! quantities that share a unit -- and resolves nothing else.

use aequitas::systems::si::{dimensions, units as si_units};
use aequitas::unit::LinearUnit;

use super::fixtures::assert_exact;
use crate::units::{by_name, by_tag, names_for_tag};

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
    let names: Vec<&str> = names_for_tag(diffusivity.tag).collect();

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
    assert_eq!(names_for_tag(exotic).count(), 0);
    assert!(by_tag(exotic).is_none());
}

#[test]
fn tag_lookup_finds_a_name_for_a_registered_dimension() {
    let length = by_name("length").expect("length is registered");
    assert_eq!(by_tag(length.tag).map(|found| found.name), Some("length"));
}
