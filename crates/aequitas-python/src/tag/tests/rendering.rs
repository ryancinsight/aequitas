//! The wire form: semantic names, and the unit-algebra rendering.
//!
//! Both are read by a Python consumer rather than by these tests, so the
//! contract is that a name or a rendering is stable and unambiguous -- two
//! markers may not share a wire name, and a rendering may not hide the marker
//! that separates two dimensionally identical tags.

use aequitas::systems::si::dimensions;

use crate::tag::{SemanticTag, TaggedDimension};

#[test]
fn semantic_names_are_unique_and_round_trip() {
    let mut names: Vec<&str> = SemanticTag::ALL.iter().map(|tag| tag.name()).collect();
    let count = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), count, "two markers share a wire name");

    for tag in SemanticTag::ALL {
        let found = SemanticTag::ALL
            .into_iter()
            .find(|candidate| candidate.name() == tag.name());
        assert_eq!(found, Some(tag));
    }
}

#[test]
fn display_renders_unit_algebra_and_marks_semantics() {
    let pressure = <dimensions::Pressure as TaggedDimension>::TAG;
    assert_eq!(pressure.to_string(), "m^-1 kg s^-2");

    let stress = <dimensions::Stress as TaggedDimension>::TAG;
    assert_eq!(stress.to_string(), "m^-1 kg s^-2 [stress]");

    let scalar = <dimensions::Dimensionless as TaggedDimension>::TAG;
    assert_eq!(scalar.to_string(), "dimensionless");

    let length = <dimensions::Length as TaggedDimension>::TAG;
    assert_eq!(length.to_string(), "m");
}
