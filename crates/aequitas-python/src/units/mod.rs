//! The generated quantity and unit inventory.

pub mod inventory;
pub mod registry;

use inventory::Inventory;
use registry::{Quantities, Quantity};

use crate::tag::DimensionTag;

/// Every quantity Aequitas names, in declaration order.
#[must_use]
pub fn all() -> &'static [Quantity] {
    Inventory::ALL
}

/// Find a quantity by its Python attribute name.
#[must_use]
pub fn by_name(name: &str) -> Option<&'static Quantity> {
    all().iter().find(|quantity| quantity.name == name)
}

/// Find the first named quantity carrying `tag`.
///
/// A dimension does not identify a quantity. Seven dimensions in the SI
/// inventory carry more than one alias -- `m^2/s` is `AreaPerTime`,
/// `ThermalDiffusivity` and `KinematicViscosity`; `J/kg` is both
/// `AbsorbedDose` and `SpecificEnergy` -- so this answers "what may this be
/// called", never "what is this". Use [`names_for_tag`] where the full set
/// matters, and never treat this result as identity.
#[must_use]
pub fn by_tag(tag: DimensionTag) -> Option<&'static Quantity> {
    all().iter().find(|quantity| quantity.tag == tag)
}

/// Every name Aequitas gives `tag`, in declaration order.
pub fn names_for_tag(tag: DimensionTag) -> impl Iterator<Item = &'static str> {
    all()
        .iter()
        .filter(move |quantity| quantity.tag == tag)
        .map(|quantity| quantity.name)
}

#[cfg(test)]
mod tests;
