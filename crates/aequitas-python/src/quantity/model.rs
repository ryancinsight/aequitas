//! The Python-visible physical quantity: its value and its accessors.
//!
//! The `#[pymethods]` surface is divided by seam across sibling leaves —
//! [`super::construct`], [`super::inspect`] and [`super::wire`] — so no one
//! file carries every contract. All of them reach the magnitude and the tag
//! through the accessors below rather than the fields, which keeps the two
//! fields private to this leaf.

use pyo3::prelude::*;

use crate::tag::DimensionTag;

/// A physical quantity: a magnitude in canonical SI base units and the
/// dimension it belongs to.
///
/// Immutable by construction. Aequitas' Rust quantities are
/// `#[repr(transparent)]` values whose dimension is a type parameter; the
/// Python image keeps the magnitude and carries the dimension as data, since
/// there is no compile step here to discharge it.
// `from_py_object` is explicit: `extract_quantity` downcasts through it as
// the fast path before falling back to the duck-typed protocol.
#[pyclass(
    name = "Quantity",
    module = "pyaequitas._pyaequitas",
    frozen,
    subclass,
    from_py_object
)]
#[derive(Clone, Copy, Debug)]
pub struct PyQuantity {
    base: f64,
    tag: DimensionTag,
}

impl PyQuantity {
    /// Construct from a magnitude already in canonical SI base units.
    #[must_use]
    pub const fn from_base(base: f64, tag: DimensionTag) -> Self {
        Self { base, tag }
    }

    /// Magnitude in canonical SI base units.
    #[must_use]
    pub const fn base_value(&self) -> f64 {
        self.base
    }

    /// Runtime dimension.
    #[must_use]
    pub const fn tag(&self) -> DimensionTag {
        self.tag
    }
}
