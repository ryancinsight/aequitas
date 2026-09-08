//! Reading a protocol-conforming object's magnitude and dimension tag.

use pyo3::Borrowed;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::protocol::{BASE_ATTR, DIMENSION_ATTR};
use crate::tag::{AXES, DimensionTag, SemanticTag};

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

/// Whether an object declares itself a quantity.
///
/// Presence of both attributes, not their contents: a malformed tag must
/// surface as the error [`read`] gives it rather than silently falling back to
/// a bare-number reading, which is the hole this check closes.
pub(super) fn carries_protocol(object: Borrowed<'_, '_, PyAny>) -> bool {
    object.hasattr(BASE_ATTR).unwrap_or(false) && object.hasattr(DIMENSION_ATTR).unwrap_or(false)
}
