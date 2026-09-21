//! Reading a quantity from Python: magnitude, units, dimension, identity.
//!
//! Every getter here answers a question a Python caller asks about an
//! existing quantity, including the two that make it usable as a dict key and
//! inside an f-string. Nothing in this leaf constructs or converts.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use super::model::PyQuantity;
use super::wire::dimension_tuple;
use crate::units;

#[pymethods]
impl PyQuantity {
    /// Magnitude in canonical SI base units.
    #[getter]
    pub(crate) fn base(&self) -> f64 {
        self.base_value()
    }

    /// Magnitude expressed in `unit`.
    ///
    /// # Errors
    ///
    /// Raises `ValueError` when no named quantity of this dimension defines
    /// `unit`.
    pub(crate) fn in_unit(&self, unit: &str) -> PyResult<f64> {
        let tag = self.tag();
        let resolved = units::all()
            .iter()
            .filter(|quantity| quantity.tag == tag)
            .find_map(|quantity| quantity.unit(unit))
            .ok_or_else(|| {
                PyValueError::new_err(format!("unit '{unit}' does not belong to `{tag}`"))
            })?;
        Ok(resolved.from_base(self.base_value()))
    }

    /// The dimension tag, as `((exponents...), semantics)`.
    ///
    /// This is the wire form of the cross-extension protocol: a consumer
    /// compares the tuple rather than downcasting to this class, so two
    /// independently built extension modules interoperate without sharing a
    /// `pyclass` ABI.
    #[getter]
    pub(crate) fn dimension<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        dimension_tuple(py, self.tag())
    }

    /// A name Aequitas gives this dimension, when one exists.
    ///
    /// A label, not an identity: seven dimensions carry several aliases, so
    /// `m^2/s` reports `area_per_time` whether it arose as a thermal
    /// diffusivity or a kinematic viscosity. Use [`Self::quantities`] when the
    /// distinction matters.
    #[getter]
    pub(crate) fn quantity(&self) -> Option<&'static str> {
        units::by_tag(self.tag()).map(|quantity| quantity.name)
    }

    /// Every name Aequitas gives this dimension, in declaration order.
    ///
    /// Empty for a dimension no alias names, such as one arising mid-way
    /// through a longer derivation.
    #[getter]
    pub(crate) fn quantities(&self) -> Vec<&'static str> {
        units::names_for_tag(self.tag()).collect()
    }

    /// True when every SI exponent is zero.
    ///
    /// An angle answers `True` here while remaining distinct from a bare
    /// scalar, matching `AngleSemantics`.
    #[getter]
    pub(crate) fn is_dimensionless(&self) -> bool {
        self.tag().is_dimensionless()
    }

    pub(crate) fn __repr__(&self) -> String {
        let tag = self.tag();
        match units::by_tag(tag) {
            Some(named) => format!("Quantity({}, '{}')", self.base_value(), named.name),
            None => format!("Quantity({}, <{}>)", self.base_value(), tag),
        }
    }

    pub(crate) fn __str__(&self) -> String {
        format!("{} {}", self.base_value(), self.tag())
    }

    pub(crate) fn __hash__(&self) -> u64 {
        // `f64` is not `Hash`; its bit pattern is, and two quantities compare
        // equal only when both the magnitude and the tag match, so hashing the
        // bits keeps the `Eq`/`Hash` agreement std requires. `-0.0` and `0.0`
        // compare equal with different bits, so `-0.0` normalizes first.
        let base = self.base_value();
        let tag = self.tag();
        let bits = if base == 0.0 {
            0.0_f64.to_bits()
        } else {
            base.to_bits()
        };
        let mut hash = bits;
        for exponent in tag.exponents() {
            hash = hash.rotate_left(7) ^ u64::from(exponent.cast_unsigned());
        }
        hash.rotate_left(7) ^ u64::from(tag.semantics().discriminant())
    }
}
