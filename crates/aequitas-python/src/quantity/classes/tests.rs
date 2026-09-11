//! The class table against the registry it must partition.

use std::collections::HashSet;

use pyo3::prelude::*;

use aequitas::systems::si::dimensions;

use super::Classed;
use super::inventory::CLASSES;
use super::model::class_for;
use crate::quantity::PyQuantity;
use crate::tag::{DimensionTag, SemanticTag, TaggedDimension};
use crate::units;

#[test]
fn every_named_quantity_has_a_class_carrying_its_name() {
    for quantity in units::all() {
        let class =
            class_for(quantity.tag).unwrap_or_else(|| panic!("`{}` has no class", quantity.alias));
        assert!(
            class.name == quantity.alias || class.aliases.contains(&quantity.alias),
            "`{}` resolves to `{}`, which does not carry its name",
            quantity.alias,
            class.name,
        );
    }
}

#[test]
fn there_is_one_class_per_distinct_named_dimension() {
    let distinct: HashSet<DimensionTag> =
        units::all().iter().map(|quantity| quantity.tag).collect();
    assert_eq!(CLASSES.len(), distinct.len());
    let names: usize = CLASSES.iter().map(|class| 1 + class.aliases.len()).sum();
    assert_eq!(
        names,
        units::all().len(),
        "every alias is named exactly once"
    );
}

#[test]
fn aliases_of_one_rust_type_share_its_class() {
    let class = class_for(<dimensions::KinematicViscosity as TaggedDimension>::TAG)
        .expect("kinematic viscosity is named");
    assert_eq!(class.name, "AreaPerTime");
    assert_eq!(class.aliases, ["ThermalDiffusivity", "KinematicViscosity"]);
}

#[test]
fn a_semantic_marker_keeps_classes_apart() {
    let pressure = class_for(<dimensions::Pressure as TaggedDimension>::TAG).expect("named");
    let stress = class_for(<dimensions::Stress as TaggedDimension>::TAG).expect("named");
    assert_eq!((pressure.name, stress.name), ("Pressure", "Stress"));
}

#[test]
fn a_quantity_becomes_an_instance_of_its_class_in_python() {
    Python::attach(|py| {
        let length = PyQuantity::from_base(2.0, <dimensions::Length as TaggedDimension>::TAG);
        let object = Classed::from(length)
            .into_pyobject(py)
            .expect("instantiates");
        assert_eq!(
            object.get_type().name().expect("named").to_string(),
            "Length"
        );
        let back: PyQuantity = object.extract().expect("a Length is a Quantity");
        assert_eq!(back.base_value().to_bits(), 2.0_f64.to_bits());
        assert_eq!(back.tag(), length.tag());

        let unnamed = DimensionTag::new([5, 0, 0, 0, 0, 0, 0], SemanticTag::Base);
        let object = Classed::from(PyQuantity::from_base(1.0, unnamed))
            .into_pyobject(py)
            .expect("instantiates");
        assert!(
            object.get_type().is(py.get_type::<PyQuantity>()),
            "m^5 has no class and stays a plain Quantity"
        );
    });
}
