//! Python bindings for the Aequitas physical-quantity surface.
//!
//! Aequitas encodes SI dimensions in types: `Quantity<T, D>` is
//! `#[repr(transparent)]` over `T` with `D` in `PhantomData`, so dimensional
//! correctness is discharged during monomorphization and erased before the
//! binary exists. There is nothing left at runtime for Python to hold.
//!
//! This crate therefore exports a runtime image rather than the types
//! themselves: a magnitude in canonical SI base units carried alongside the
//! seven SI exponents and a semantic discriminant. Both halves are derived
//! from the Rust type parameters (see [`tag::TaggedDimension`]) rather than
//! transcribed, and every unit symbol and scale factor is read from the
//! `LinearUnit` associated constants, so the law crate stays the single source
//! of truth and this surface cannot disagree with it about what a unit means.
//!
//! Dimensional violations that `rustc` rejects at compile time surface here as
//! Python exceptions. Static checking is recovered in the consumer's own type
//! checker through the generated stubs, not abandoned.
//!
//! # Layering
//!
//! `aequitas` itself never depends on `pyo3`; this crate is the only place the
//! two meet.

#![deny(missing_docs)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod consumer;
pub mod protocol;
pub mod quantity;
pub mod tag;
pub mod units;

pub use consumer::Dimensioned;
pub use quantity::PyQuantity;

/// Names of every quantity Aequitas defines.
#[pyfunction]
fn quantity_names() -> Vec<&'static str> {
    units::all().iter().map(|quantity| quantity.name).collect()
}

/// Unit symbols admitted by `quantity`, in registration order.
///
/// # Errors
///
/// Raises `KeyError` when no such quantity exists.
#[pyfunction]
fn unit_symbols(quantity: &str) -> PyResult<Vec<&'static str>> {
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
fn dimension_of<'py>(
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
fn base_value_of(value: &Bound<'_, PyAny>, expected: &str) -> PyResult<f64> {
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

/// The registered extension surface.
///
/// # Errors
///
/// Propagates registration failures from the interpreter.
#[pymodule]
fn _pyaequitas(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyQuantity>()?;
    module.add_function(wrap_pyfunction!(quantity_names, module)?)?;
    module.add_function(wrap_pyfunction!(unit_symbols, module)?)?;
    module.add_function(wrap_pyfunction!(dimension_of, module)?)?;
    module.add_function(wrap_pyfunction!(base_value_of, module)?)?;

    module.add("BASE_ATTR", protocol::BASE_ATTR)?;
    module.add("DIMENSION_ATTR", protocol::DIMENSION_ATTR)?;
    module.add("PROTOCOL_VERSION", protocol::PROTOCOL_VERSION)?;

    // The unit table is exported as data so the pure-Python layer can build
    // per-quantity constructors without a second inventory.
    let table = PyDict::new(module.py());
    for quantity in units::all() {
        let symbols: Vec<&str> = quantity.symbols().collect();
        table.set_item(quantity.name, symbols)?;
    }
    module.add("UNITS", table)?;

    Ok(())
}
