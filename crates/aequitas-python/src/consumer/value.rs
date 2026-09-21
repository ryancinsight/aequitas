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
/// # Cost
///
/// Measured at the head that introduced this note: two Rust heap allocations
/// and ~296 ns per call, and the Python-passed-quantity path through
/// [`crate::consumer::Dimensioned`] is ~448 ns per argument because a native
/// quantity takes this structural read rather than a downcast.
///
/// Both allocations were examined and neither can be removed without a
/// decision this crate does not own:
///
/// - The marker name cannot be borrowed. `PyStringMethods::to_str` is gated on
///   `any(Py_3_10, not(Py_LIMITED_API))`, and this crate builds `abi3-py38`, so
///   the borrowed form does not exist here; `to_cow` and `to_string_lossy` both
///   fall back to an owned copy under the limited API, which copies *twice*.
///   Borrowing it means dropping the abi3 floor -- a distribution decision.
/// - The exponent vector cannot be a stack array without narrowing the wire
///   form: `Vec<i64>` accepts any Python sequence of integers, where a
///   fixed-size array accepts only its own shape. That is a change to what the
///   protocol admits, and the Python-side suite that would justify it needs a
///   built wheel that this checkout does not have.
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
pub(crate) fn carries_protocol(object: Borrowed<'_, '_, PyAny>) -> bool {
    object.hasattr(BASE_ATTR).unwrap_or(false) && object.hasattr(DIMENSION_ATTR).unwrap_or(false)
}
