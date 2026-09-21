//! The shape of the generated inventory.
//!
//! These are claims about the table itself rather than about any one
//! conversion: it is complete, its names are unique, its scales are
//! physically meaningful, and every tag it carries is the law crate's own.
//! A generator that dropped an alias, or paired a quantity with the wrong
//! dimension, fails here first.

use aequitas::systems::si::dimensions;

use crate::tag::TaggedDimension;
use crate::units::{all, by_name};

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
            if let Some(offset) = unit.offset {
                assert!(
                    offset.is_finite(),
                    "{}::{} has offset {offset}",
                    quantity.name,
                    unit.name
                );
            }
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

#[test]
fn dimensionless_carries_no_linear_units() {
    // No `LinearUnit` impl targets `Dimensionless`: a bare ratio has no unit
    // to convert through, and construction goes through base values.
    let scalar = by_name("dimensionless").expect("dimensionless is registered");
    assert!(scalar.units.is_empty());
}
