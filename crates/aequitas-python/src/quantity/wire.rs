//! The cross-extension wire form: render a quantity out, read one back.
//!
//! Two wheels built independently must interoperate without sharing a
//! `pyclass` ABI, so a quantity leaves as a plain `((exponents...), semantics)`
//! tuple plus a magnitude, and comes back through the structural read in
//! [`crate::consumer`]. The downcast in [`extract_quantity`] is only a fast
//! path over that; the tuple comparison is what makes the versions
//! independent.

use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use super::model::PyQuantity;
use crate::tag::DimensionTag;

#[pymethods]
impl PyQuantity {
    /// Protocol alias of [`Self::base`].
    #[getter(__aequitas_base__)]
    pub(crate) fn protocol_base(&self) -> f64 {
        self.base_value()
    }

    /// Protocol alias of [`Self::dimension`].
    #[getter(__aequitas_dimension__)]
    pub(crate) fn protocol_dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        dimension_tuple(py, self.tag())
    }
}

/// Render a tag as the protocol tuple `((i64 x 7), str)`.
///
/// # Errors
///
/// Propagates interpreter failures from tuple construction.
pub fn dimension_tuple(py: Python<'_>, tag: DimensionTag) -> PyResult<Bound<'_, PyTuple>> {
    let exponents = PyTuple::new(py, tag.exponents().iter().map(|&e| i64::from(e)))?;
    let semantics = PyString::new(py, tag.semantics().name());
    PyTuple::new(py, [exponents.into_any(), semantics.into_any()])
}

/// Read a quantity out of any object satisfying the protocol.
///
/// Tries this class first, then the structural read in [`crate::consumer`].
/// The downcast is only a fast path: the duck-typed arm is what lets a
/// consumer built against a different `aequitas-python` version interoperate,
/// because the tag tuple compares structurally where a downcast would not.
///
/// # Errors
///
/// Raises `TypeError` when the object is neither a quantity nor protocol-
/// conforming, and `ValueError` when the tag is malformed.
pub fn extract_quantity(value: &Bound<'_, PyAny>) -> PyResult<PyQuantity> {
    if let Ok(native) = value.extract::<PyQuantity>() {
        return Ok(native);
    }
    let (base, tag) = crate::consumer::read(value)?;
    Ok(PyQuantity::from_base(base, tag))
}
