//! Constructing a quantity: from a named unit, or from base units.
//!
//! This is the unit-resolution seam, and it is scoped to a named quantity on
//! purpose: a symbol does not identify a unit on its own, so the name is what
//! says which `LinearUnit` impl to read. Both constructors return
//! [`Classed`], so a caller receives the class for the resulting dimension.

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

use super::classes::Classed;
use super::model::PyQuantity;
use crate::units;

#[pymethods]
impl PyQuantity {
    /// Construct a quantity of `quantity` from a value in `unit`.
    ///
    /// Unit resolution is scoped to the named quantity because a symbol does
    /// not identify a unit on its own: `Pa` is both a pressure and a stress,
    /// `K` both an absolute temperature and a temperature difference.
    #[staticmethod]
    #[pyo3(signature = (value, unit, *, quantity))]
    pub(crate) fn from_unit(value: f64, unit: &str, quantity: &str) -> PyResult<Classed> {
        let named = units::by_name(quantity)
            .ok_or_else(|| PyKeyError::new_err(format!("unknown quantity '{quantity}'")))?;
        let resolved = named.unit(unit).ok_or_else(|| {
            let known: Vec<&str> = named.symbols().collect();
            PyValueError::new_err(format!(
                "unknown unit '{unit}' for {quantity}; known units: {}",
                if known.is_empty() {
                    "none (construct from base units)".to_owned()
                } else {
                    known.join(", ")
                }
            ))
        })?;
        Ok(Self::from_base(resolved.to_base(value), named.tag).into())
    }

    /// Construct a quantity of `quantity` from a value already in base units.
    #[staticmethod]
    pub(crate) fn from_base_of(value: f64, quantity: &str) -> PyResult<Classed> {
        let named = units::by_name(quantity)
            .ok_or_else(|| PyKeyError::new_err(format!("unknown quantity '{quantity}'")))?;
        Ok(Self::from_base(value, named.tag).into())
    }
}
