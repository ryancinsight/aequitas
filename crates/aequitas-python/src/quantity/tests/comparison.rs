//! Ordering, equality, hashing, and rendering.
//!
//! Equality is total across dimensions and foreign objects -- it answers
//! `False` rather than raising -- while ordering refuses across them.

use super::fixtures::{length, time};
use crate::quantity::PyQuantity;
use crate::tag::{DimensionTag, SemanticTag};
use pyo3::prelude::*;

#[test]
fn comparison_orders_within_a_dimension_and_refuses_across_one() {
    Python::attach(|py| {
        let shorter = length(1.0);
        let longer = length(2.0).into_pyobject(py).expect("bind");
        let ordered = shorter
            .__richcmp__(&longer, pyo3::basic::CompareOp::Lt)
            .expect("lengths order");
        assert!(ordered.extract::<bool>(py).expect("bool"));

        let second = time(2.0).into_pyobject(py).expect("bind");
        assert!(
            shorter
                .__richcmp__(&second, pyo3::basic::CompareOp::Lt)
                .is_err(),
            "a length does not order against a time"
        );
    });
}

#[test]
fn equality_across_dimensions_is_false_not_an_error() {
    Python::attach(|py| {
        let metre = length(1.0);
        let second = time(1.0).into_pyobject(py).expect("bind");
        let equal = metre
            .__richcmp__(&second, pyo3::basic::CompareOp::Eq)
            .expect("equality is total");
        assert!(!equal.extract::<bool>(py).expect("bool"));
    });
}

#[test]
fn equality_against_a_foreign_object_is_false() {
    Python::attach(|py| {
        let metre = length(1.0);
        let text = "not a quantity".into_pyobject(py).expect("bind");
        let equal = metre
            .__richcmp__(&text, pyo3::basic::CompareOp::Eq)
            .expect("equality is total");
        assert!(!equal.extract::<bool>(py).expect("bool"));
    });
}

#[test]
fn equal_quantities_hash_equally() {
    let one = length(1.5);
    let other = length(1.5);
    assert_eq!(one.__hash__(), other.__hash__());

    // Signed zero compares equal, so it must hash equally too.
    assert_eq!(length(0.0).__hash__(), length(-0.0).__hash__());

    // A different dimension with the same magnitude should not collide.
    assert_ne!(length(1.5).__hash__(), time(1.5).__hash__());
}

#[test]
fn repr_names_the_quantity_and_str_names_the_dimension() {
    let metre = length(2.0);
    assert_eq!(metre.__repr__(), "Quantity(2, 'length')");
    assert_eq!(metre.__str__(), "2 m");
}

#[test]
fn an_unnamed_derived_dimension_still_renders() {
    // L^5 has no alias; the repr falls back to the unit algebra rather than
    // failing or claiming a name it does not have.
    let odd = PyQuantity::from_base(
        1.0,
        DimensionTag::new([5, 0, 0, 0, 0, 0, 0], SemanticTag::Base),
    );
    assert_eq!(odd.quantity(), None);
    assert!(odd.__repr__().contains("m^5"), "{}", odd.__repr__());
}
