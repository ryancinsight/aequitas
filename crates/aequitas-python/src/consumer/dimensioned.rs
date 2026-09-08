//! The dimension-checked extractor a consumer binds its arguments to.

use core::marker::PhantomData;

use aequitas::Quantity;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{Borrowed, FromPyObject};

use super::value::{carries_protocol, read};
use crate::tag::TaggedDimension;

/// A parameter accepting a base-unit float or a quantity of dimension `D`.
///
/// The float arm preserves an already-published contract: a consumer whose
/// signature was `f64` in canonical SI base units keeps accepting exactly what
/// it accepted before, unchecked, because there is nothing in a bare number to
/// check. The quantity arm is the addition, and it is checked -- passing a
/// time where a length belongs raises rather than silently scaling.
///
/// `D` is a type-level Aequitas dimension, so the expected tag is a constant
/// and the check is a comparison of two 8-byte values.
///
/// ```ignore
/// #[pyfunction]
/// fn focus(radius: Dimensioned<dimensions::Length>) -> f64 {
///     let radius: Length<f64> = radius.quantity();
///     // ...
/// }
/// ```
#[derive(Debug)]
pub struct Dimensioned<D> {
    base: f64,
    dimension: PhantomData<D>,
}

impl<D> Dimensioned<D> {
    /// Construct from a magnitude already in canonical SI base units.
    #[must_use]
    pub const fn from_base(base: f64) -> Self {
        Self {
            base,
            dimension: PhantomData,
        }
    }

    /// The magnitude in canonical SI base units.
    #[must_use]
    pub const fn base(&self) -> f64 {
        self.base
    }

    /// The typed Aequitas quantity.
    ///
    /// The point of the extractor: past this call the value is dimensioned in
    /// the Rust type system again, and the binding layer is the only place a
    /// bare number existed.
    #[must_use]
    pub const fn quantity(&self) -> Quantity<f64, D> {
        Quantity::from_base(self.base)
    }
}

impl<D> Clone for Dimensioned<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> Copy for Dimensioned<D> {}

impl<'py, D> FromPyObject<'_, 'py> for Dimensioned<D>
where
    D: TaggedDimension,
{
    type Error = PyErr;

    fn extract(object: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        // The protocol is checked first, and the order is load-bearing. `f64`
        // extraction goes through `PyFloat_AsDouble`, which honours
        // `__float__` -- and a quantity type may well define one; pint's does.
        // Trying the number first therefore let any such object through as a
        // bare magnitude with its dimension never examined, so a time was
        // accepted where a length belonged. An object that declares itself a
        // quantity is treated as one.
        if !carries_protocol(object)
            && let Ok(base) = object.extract::<f64>()
        {
            return Ok(Self::from_base(base));
        }

        let (base, tag) = read(&object)?;
        let expected = <D as TaggedDimension>::TAG;
        if tag != expected {
            return Err(PyValueError::new_err(format!(
                "expected a quantity of `{expected}`, got `{tag}`"
            )));
        }
        Ok(Self::from_base(base))
    }
}
