//! Free functions the extension exposes for inspecting the unit registry.

use pyo3::prelude::*;

use crate::{quantity, units};

/// Names of every quantity Aequitas defines.
#[pyfunction]
pub(crate) fn quantity_names() -> Vec<&'static str> {
    units::all().iter().map(|quantity| quantity.name).collect()
}

/// Unit symbols admitted by `quantity`, in registration order.
///
/// # Errors
///
/// Raises `KeyError` when no such quantity exists.
#[pyfunction]
pub(crate) fn unit_symbols(quantity: &str) -> PyResult<Vec<&'static str>> {
    units::by_name(quantity)
        .map(|named| named.symbols().collect())
        .ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!("unknown quantity '{quantity}'"))
        })
}

/// The dimension tag of `quantity`, in protocol form.
///
/// # Errors
///
/// Raises `KeyError` when no such quantity exists.
#[pyfunction]
pub(crate) fn dimension_of<'py>(
    py: Python<'py>,
    quantity: &str,
) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
    let named = units::by_name(quantity).ok_or_else(|| {
        pyo3::exceptions::PyKeyError::new_err(format!("unknown quantity '{quantity}'"))
    })?;
    quantity::dimension_tuple(py, named.tag)
}

/// Read a magnitude in base units out of any protocol-conforming object.
///
/// The entry point a consumer uses to accept quantities without depending on
/// this crate's `pyclass` ABI: it validates the dimension tag against the
/// named `expected` quantity and returns the base-unit magnitude.
///
/// # Errors
///
/// Raises `KeyError` for an unknown quantity name, `TypeError` for a
/// non-conforming object, and `ValueError` when the dimension does not match.
#[pyfunction]
pub(crate) fn base_value_of(value: &Bound<'_, PyAny>, expected: &str) -> PyResult<f64> {
    let named = units::by_name(expected).ok_or_else(|| {
        pyo3::exceptions::PyKeyError::new_err(format!("unknown quantity '{expected}'"))
    })?;
    let quantity = quantity::extract_quantity(value)?;
    if quantity.tag() != named.tag {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "expected {expected} (`{}`), got `{}`",
            named.tag,
            quantity.tag()
        )));
    }
    Ok(quantity.base_value())
}
