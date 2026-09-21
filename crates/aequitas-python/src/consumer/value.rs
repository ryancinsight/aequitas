//! Reading a protocol-conforming object's magnitude and dimension tag.

use pyo3::Borrowed;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyString, PyStringMethods};

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
/// Release timings measured at the head that introduced this note, allocation
/// counts measured at the head that reduced them: the structural read cost two
/// Rust heap allocations and ~296 ns per call, and the Python-passed-quantity
/// path through [`crate::consumer::Dimensioned`] cost ~448 ns per argument
/// because a native quantity takes this structural read rather than a
/// downcast. It allocates nothing now. An allocation count does not depend on
/// the build profile; the times are magnitudes from one run on one machine,
/// not benchmarks, and they were not re-measured for this revision.
///
/// Nothing is allocated. The marker name is borrowed rather than copied, and
/// that is what fixes this crate's Python floor: `PyStringMethods::to_str` is
/// gated on `any(Py_3_10, not(Py_LIMITED_API))`, and `abi3-py310` is the
/// lowest stable-ABI level that satisfies the first disjunct, which is the one
/// that keeps a single wheel per platform. Every alternative that would have
/// held the floor at 3.8 copies: `to_cow` and `to_string_lossy` both fall back
/// to an owned `String` under the limited API, and `PartialEq<str>` routes
/// through `to_cow` below 3.13, so comparing eleven candidate names would
/// copy eleven times rather than once. Below 3.10 no borrowing read exists.
///
/// The second allocation was the exponent vector, and it is gone: the tag's
/// members are taken as objects and the exponents are read straight into the
/// fixed-width axis array, where unpacking the tag as `(Vec<i64>, String)`
/// built a seven-element vector per call only to copy it into that array and
/// drop it. Nothing about the admitted input changed -- a `Vec<i64>` accepts
/// any iterable of integers, and so does this loop, which still reports the
/// length it actually found, so every message this function raises is
/// unchanged and the cases that assert them still pass. A fixed-size array
/// *extraction* would have been the simpler edit and was rejected: it narrows
/// the protocol from any iterable to a sequence, silently, for input this
/// crate does not produce.
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

    // The two members come back as objects, so no intermediate vector is built
    // for exponents that are about to be copied into a fixed-width array.
    let (exponents, semantics): (Bound<'_, PyAny>, Bound<'_, PyAny>) =
        dimension.extract().map_err(|_| malformed_tag())?;

    let mut axes = [0_i8; AXES];
    let mut width = 0_usize;
    for exponent in exponents.try_iter().map_err(|_| malformed_tag())? {
        let exponent: i64 = exponent?.extract().map_err(|_| malformed_tag())?;
        // Past the seventh exponent the values are only counted, so an
        // over-long tag reports its true length rather than its first
        // unrepresentable entry -- which is what the vector form did.
        if width < AXES {
            axes[width] = i8::try_from(exponent).map_err(|_| {
                PyValueError::new_err(format!("dimension exponent {exponent} is out of range"))
            })?;
        }
        width += 1;
    }

    if width != AXES {
        return Err(PyValueError::new_err(format!(
            "`{DIMENSION_ATTR}` must carry exactly {AXES} exponents, got {width}"
        )));
    }

    // Borrowed, not extracted: a `String` here would be a heap copy of a
    // marker name that is immediately compared and dropped, and the floor
    // above exists so that this read can be the borrowed one.
    let semantics = semantics.cast::<PyString>().map_err(|_| malformed_tag())?;
    let semantics = semantics.to_str().map_err(|_| malformed_tag())?;

    let semantics = SemanticTag::from_name(semantics).ok_or_else(|| {
        PyValueError::new_err(format!("unknown dimension semantics '{semantics}'"))
    })?;

    Ok((base, DimensionTag::new(axes, semantics)))
}

/// The error a tag that is not `((int x AXES), str)` gets.
fn malformed_tag() -> PyErr {
    PyValueError::new_err(format!("`{DIMENSION_ATTR}` must be ((int x {AXES}), str)"))
}

/// Whether an object declares itself a quantity.
///
/// Presence of both attributes, not their contents: a malformed tag must
/// surface as the error [`read`] gives it rather than silently falling back to
/// a bare-number reading, which is the hole this check closes.
pub(crate) fn carries_protocol(object: Borrowed<'_, '_, PyAny>) -> bool {
    object.hasattr(BASE_ATTR).unwrap_or(false) && object.hasattr(DIMENSION_ATTR).unwrap_or(false)
}
