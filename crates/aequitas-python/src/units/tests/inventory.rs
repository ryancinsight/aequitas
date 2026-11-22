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
fn every_symbol_resolves_once_per_named_quantity() {
    for quantity in all() {
        let mut symbols = std::collections::HashSet::new();
        for symbol in quantity.symbols() {
            assert!(
                symbols.insert(symbol),
                "{} has duplicate {symbol}",
                quantity.name
            );
            assert!(quantity.unit(symbol).is_some());
        }
    }
}

#[test]
fn a_token_has_one_conversion_per_full_dimension_tag() {
    let mut seen = std::collections::HashMap::new();
    for quantity in all() {
        for unit in quantity.units {
            let signature = (unit.scale.to_bits(), unit.offset.map(f64::to_bits));
            for token in [unit.symbol, unit.name]
                .into_iter()
                .chain(unit.alternate_symbols().iter().copied())
            {
                if let Some((previous, quantity_name, marker)) =
                    seen.insert((quantity.tag, token), (signature, quantity.name, unit.name))
                {
                    assert_eq!(
                        previous, signature,
                        "conflicting conversions for {token:?} in {}: {quantity_name}::{marker} and {}::{}",
                        quantity.tag, quantity.name, unit.name
                    );
                }
            }
        }
    }
}
