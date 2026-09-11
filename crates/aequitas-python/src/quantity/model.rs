//! The Python-visible physical quantity.

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

use crate::tag::DimensionTag;
use crate::units;

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

#[pymethods]
impl PyQuantity {
    /// Construct a quantity of `quantity` from a value in `unit`.
    ///
    /// Unit resolution is scoped to the named quantity because a symbol does
    /// not identify a unit on its own: `Pa` is both a pressure and a stress,
    /// `K` both an absolute temperature and a temperature difference.
    #[staticmethod]
    #[pyo3(signature = (value, unit, *, quantity))]
    pub(crate) fn from_unit(value: f64, unit: &str, quantity: &str) -> PyResult<Self> {
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
        Ok(Self::from_base(resolved.to_base(value), named.tag))
    }

    /// Construct a quantity of `quantity` from a value already in base units.
    #[staticmethod]
    pub(crate) fn from_base_of(value: f64, quantity: &str) -> PyResult<Self> {
        let named = units::by_name(quantity)
            .ok_or_else(|| PyKeyError::new_err(format!("unknown quantity '{quantity}'")))?;
        Ok(Self::from_base(value, named.tag))
    }

    /// Magnitude in canonical SI base units.
    #[getter]
    pub(crate) fn base(&self) -> f64 {
        self.base
    }

    /// Magnitude expressed in `unit`.
    ///
    /// # Errors
    ///
    /// Raises `ValueError` when no named quantity of this dimension defines
    /// `unit`.
    pub(crate) fn in_unit(&self, unit: &str) -> PyResult<f64> {
        let resolved = units::all()
            .iter()
            .filter(|quantity| quantity.tag == self.tag)
            .find_map(|quantity| quantity.unit(unit))
            .ok_or_else(|| {
                PyValueError::new_err(format!("unit '{unit}' does not belong to `{}`", self.tag))
            })?;
        Ok(resolved.from_base(self.base))
    }

    /// The dimension tag, as `((exponents...), semantics)`.
    ///
    /// This is the wire form of the cross-extension protocol: a consumer
    /// compares the tuple rather than downcasting to this class, so two
    /// independently built extension modules interoperate without sharing a
    /// `pyclass` ABI.
    #[getter]
    pub(crate) fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        dimension_tuple(py, self.tag)
    }

    /// Protocol alias of [`Self::base`].
    #[getter(__aequitas_base__)]
    pub(crate) fn protocol_base(&self) -> f64 {
        self.base
    }

    /// Protocol alias of [`Self::dimension`].
    #[getter(__aequitas_dimension__)]
    pub(crate) fn protocol_dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        dimension_tuple(py, self.tag)
    }

    /// A name Aequitas gives this dimension, when one exists.
    ///
    /// A label, not an identity: seven dimensions carry several aliases, so
    /// `m^2/s` reports `area_per_time` whether it arose as a thermal
    /// diffusivity or a kinematic viscosity. Use [`Self::quantities`] when the
    /// distinction matters.
    #[getter]
    pub(crate) fn quantity(&self) -> Option<&'static str> {
        units::by_tag(self.tag).map(|quantity| quantity.name)
    }

    /// Every name Aequitas gives this dimension, in declaration order.
    ///
    /// Empty for a dimension no alias names, such as one arising mid-way
    /// through a longer derivation.
    #[getter]
    pub(crate) fn quantities(&self) -> Vec<&'static str> {
        units::names_for_tag(self.tag).collect()
    }

    /// True when every SI exponent is zero.
    ///
    /// An angle answers `True` here while remaining distinct from a bare
    /// scalar, matching `AngleSemantics`.
    #[getter]
    pub(crate) fn is_dimensionless(&self) -> bool {
        self.tag.is_dimensionless()
    }

    pub(crate) fn __repr__(&self) -> String {
        match units::by_tag(self.tag) {
            Some(named) => format!("Quantity({}, '{}')", self.base, named.name),
            None => format!("Quantity({}, <{}>)", self.base, self.tag),
        }
    }

    pub(crate) fn __str__(&self) -> String {
        format!("{} {}", self.base, self.tag)
    }

    pub(crate) fn __hash__(&self) -> u64 {
        // `f64` is not `Hash`; its bit pattern is, and two quantities compare
        // equal only when both the magnitude and the tag match, so hashing the
        // bits keeps the `Eq`/`Hash` agreement std requires. `-0.0` and `0.0`
        // compare equal with different bits, so `-0.0` normalizes first.
        let bits = if self.base == 0.0 {
            0.0_f64.to_bits()
        } else {
            self.base.to_bits()
        };
        let mut hash = bits;
        for exponent in self.tag.exponents() {
            hash = hash.rotate_left(7) ^ u64::from(exponent.cast_unsigned());
        }
        hash.rotate_left(7) ^ u64::from(self.tag.semantics().discriminant())
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
