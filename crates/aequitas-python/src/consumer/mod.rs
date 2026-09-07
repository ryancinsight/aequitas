//! The entry point another extension module uses to accept quantities.
//!
//! A consumer wants one thing: a parameter that today is a bare `f64` in
//! canonical SI base units should keep accepting that float and additionally
//! accept a quantity, with the dimension checked. [`Dimensioned`] is that
//! parameter type, so a call site changes from `f64` to
//! `Dimensioned<dimensions::Length>` and nothing else.
//!
//! Nothing here touches [`crate::PyQuantity`]. A consumer that never registers
//! that class must not be made to instantiate its type object, and a
//! structural read is what keeps the two wheels' versions independent
//! (see [`crate::protocol`]).

use core::marker::PhantomData;

use aequitas::Quantity;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::{Borrowed, FromPyObject};

use crate::protocol::{BASE_ATTR, DIMENSION_ATTR};
use crate::tag::{AXES, DimensionTag, SemanticTag, TaggedDimension};

/// Read a magnitude and dimension out of any protocol-conforming object.
///
/// Structural, not nominal: the object need only expose [`BASE_ATTR`] and
/// [`DIMENSION_ATTR`], so an object built by a different version of this wheel
/// -- or by hand in Python -- is accepted on equal terms.
///
/// # Errors
///
/// Raises `TypeError` when the object exposes neither attribute, and
/// `ValueError` when the tag is malformed or names an unknown marker.
pub fn read(value: &Bound<'_, PyAny>) -> PyResult<(f64, DimensionTag)> {
    let (Ok(base), Ok(dimension)) = (value.getattr(BASE_ATTR), value.getattr(DIMENSION_ATTR))
    else {
        return Err(PyTypeError::new_err(format!(
            "expected a quantity exposing `{BASE_ATTR}` and `{DIMENSION_ATTR}`, got `{}`",
            value.get_type().name()?,
        )));
    };

    let base: f64 = base.extract()?;
    let (exponents, semantics): (Vec<i64>, String) = dimension.extract().map_err(|_| {
        PyValueError::new_err(format!("`{DIMENSION_ATTR}` must be ((int x {AXES}), str)"))
    })?;

    if exponents.len() != AXES {
        return Err(PyValueError::new_err(format!(
            "`{DIMENSION_ATTR}` must carry exactly {AXES} exponents, got {}",
            exponents.len()
        )));
    }

    let mut axes = [0_i8; AXES];
    for (slot, exponent) in axes.iter_mut().zip(exponents) {
        *slot = i8::try_from(exponent).map_err(|_| {
            PyValueError::new_err(format!("dimension exponent {exponent} is out of range"))
        })?;
    }

    let semantics = SemanticTag::ALL
        .into_iter()
        .find(|candidate| candidate.name() == semantics)
        .ok_or_else(|| {
            PyValueError::new_err(format!("unknown dimension semantics '{semantics}'"))
        })?;

    Ok((base, DimensionTag::new(axes, semantics)))
}

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
        // A bare number keeps its historical meaning: base SI units. Checked
        // first because it is both the common case and the cheap one.
        if let Ok(base) = object.extract::<f64>() {
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

#[cfg(test)]
mod tests;
