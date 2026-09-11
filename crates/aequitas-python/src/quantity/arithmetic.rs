//! Dimensional arithmetic on the Python side.
//!
//! Every operation here has a type-level counterpart in Aequitas. Addition
//! requires identical dimensions, which `Add` expresses by taking `Self`.
//! Multiplying or dividing by a quantity combines exponents and normalizes the
//! semantic marker, matching `MultiplyDimension` and `DivideDimension`, whose
//! outputs are always `Dimension<..., BaseSemantics>`. Scaling by a bare number
//! keeps the dimension, marker included, because `Mul<T>` and `Div<T>` for
//! `Quantity<T, D>` return `Self`. What `rustc` rejects at compile time this
//! module raises at call time.

use pyo3::exceptions::{PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyFloat, PyInt};

use super::classes::Classed;
use super::model::{PyQuantity, extract_quantity};
use crate::consumer::carries_protocol;
use crate::tag::DimensionTag;

/// A binary operation was given operands of incompatible dimension.
fn mismatch(op: &str, left: DimensionTag, right: DimensionTag) -> PyErr {
    PyValueError::new_err(format!(
        "cannot {op} `{left}` and `{right}`: dimensions differ"
    ))
}

/// Either a quantity or a bare number, as accepted by scalar-tolerant operators.
enum Operand {
    Quantity(PyQuantity),
    Scalar(f64),
}

impl Operand {
    /// A bare `float`/`int` is a dimensionless scalar; an object carrying
    /// the quantity protocol is a quantity; only an object without the
    /// protocol is read through `__float__`.
    ///
    /// The protocol must be consulted before `__float__`: a quantity type
    /// may define both (pint's defines `__float__`), and reading one as a
    /// scalar keeps its magnitude and drops its dimension, so `length * time`
    /// came back a length without an error. `Dimensioned<D>` had the same
    /// order and was corrected in #58.
    ///
    /// The two exact-type arms are the fast paths, not a semantic choice: a
    /// native quantity needs no attribute lookup, and an exact `float` or
    /// `int` cannot carry the protocol attributes.
    fn parse(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(native) = value.cast::<PyQuantity>() {
            return Ok(Self::Quantity(*native.get()));
        }
        if value.is_exact_instance_of::<PyFloat>() || value.is_exact_instance_of::<PyInt>() {
            return value.extract::<f64>().map(Self::Scalar);
        }
        if !carries_protocol(value.as_borrowed())
            && let Ok(scalar) = value.extract::<f64>()
        {
            return Ok(Self::Scalar(scalar));
        }
        // Conforming, or neither a quantity nor a number: the protocol read
        // yields the quantity or the diagnostic naming what was expected.
        extract_quantity(value).map(Self::Quantity)
    }

    /// View as a quantity, treating a bare scalar as dimensionless.
    fn as_quantity(&self) -> PyQuantity {
        match *self {
            Self::Quantity(quantity) => quantity,
            Self::Scalar(scalar) => PyQuantity::from_base(scalar, DimensionTag::DIMENSIONLESS),
        }
    }
}

#[pymethods]
impl PyQuantity {
    pub(crate) fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        let rhs = extract_quantity(other)?;
        if self.tag() != rhs.tag() {
            return Err(mismatch("add", self.tag(), rhs.tag()));
        }
        Ok(Self::from_base(self.base_value() + rhs.base_value(), self.tag()).into())
    }

    pub(crate) fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        let rhs = extract_quantity(other)?;
        if self.tag() != rhs.tag() {
            return Err(mismatch("subtract", self.tag(), rhs.tag()));
        }
        Ok(Self::from_base(self.base_value() - rhs.base_value(), self.tag()).into())
    }

    /// Scaling by a bare number keeps the dimension, marker included:
    /// `Quantity<T, D> * T` is `Quantity<T, D>` in Aequitas, so a stress times
    /// two is still a stress. Only a quantity operand combines dimensions.
    pub(crate) fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        let (factor, tag) = match Operand::parse(other)? {
            Operand::Scalar(scalar) => (scalar, self.tag()),
            Operand::Quantity(rhs) => (
                rhs.base_value(),
                self.tag()
                    .multiply(rhs.tag())
                    .map_err(|error| PyValueError::new_err(error.to_string()))?,
            ),
        };
        Ok(Self::from_base(self.base_value() * factor, tag).into())
    }

    /// Multiplication is commutative, so a scalar on the left routes here.
    pub(crate) fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        self.__mul__(other)
    }

    /// Division by a bare number keeps the dimension, as `Div<T>` does in
    /// Aequitas; division by a quantity combines dimensions.
    pub(crate) fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        let (divisor, tag) = match Operand::parse(other)? {
            Operand::Scalar(scalar) => (scalar, self.tag()),
            Operand::Quantity(rhs) => (
                rhs.base_value(),
                self.tag()
                    .divide(rhs.tag())
                    .map_err(|error| PyValueError::new_err(error.to_string()))?,
            ),
        };
        if divisor == 0.0 {
            return Err(PyZeroDivisionError::new_err("division by a zero quantity"));
        }
        Ok(Self::from_base(self.base_value() / divisor, tag).into())
    }

    /// Division is not commutative: a scalar on the left divides by this
    /// quantity, so the dimension inverts.
    pub(crate) fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Classed> {
        let lhs = Operand::parse(other)?.as_quantity();
        if self.base_value() == 0.0 {
            return Err(PyZeroDivisionError::new_err("division by a zero quantity"));
        }
        let tag = lhs
            .tag()
            .divide(self.tag())
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self::from_base(lhs.base_value() / self.base_value(), tag).into())
    }

    pub(crate) fn __neg__(&self) -> Classed {
        Self::from_base(-self.base_value(), self.tag()).into()
    }

    pub(crate) fn __pos__(&self) -> Classed {
        (*self).into()
    }

    pub(crate) fn __abs__(&self) -> Classed {
        Self::from_base(self.base_value().abs(), self.tag()).into()
    }

    /// Integer powers only.
    ///
    /// A fractional power would produce a dimension the exponent vector cannot
    /// hold, which is the same reason Aequitas offers `sqrt`/`cbrt` over exact
    /// divisions rather than a general `powf`.
    pub(crate) fn __pow__(
        &self,
        exponent: &Bound<'_, PyAny>,
        modulo: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Classed> {
        if modulo.is_some() {
            return Err(PyValueError::new_err(
                "modular exponentiation is not defined for a physical quantity",
            ));
        }
        let Ok(power) = exponent.extract::<i32>() else {
            return Err(PyValueError::new_err(
                "a quantity may be raised only to an integer power; use `sqrt` \
                 or `cbrt` for exact roots",
            ));
        };
        let narrowed = i8::try_from(power)
            .map_err(|_| PyValueError::new_err(format!("exponent {power} is out of range")))?;
        let tag = self
            .tag()
            .powi(narrowed)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self::from_base(self.base_value().powi(power), tag).into())
    }

    /// Square root, defined when every exponent is even.
    ///
    /// Runtime image of `SqrtDimension`, which has no impl for dimensions
    /// whose exponents do not halve exactly.
    pub(crate) fn sqrt(&self) -> PyResult<Classed> {
        let tag = self
            .tag()
            .root(2)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self::from_base(self.base_value().sqrt(), tag).into())
    }

    /// Cube root, defined when every exponent is a multiple of three.
    ///
    /// Sign-preserving, matching Aequitas' `cbrt`: the cube root of a negative
    /// volume is a negative length.
    pub(crate) fn cbrt(&self) -> PyResult<Classed> {
        let tag = self
            .tag()
            .root(3)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self::from_base(self.base_value().cbrt(), tag).into())
    }

    /// Multiplicative inverse.
    pub(crate) fn reciprocal(&self) -> PyResult<Classed> {
        if self.base_value() == 0.0 {
            return Err(PyZeroDivisionError::new_err(
                "reciprocal of a zero quantity",
            ));
        }
        let tag = self
            .tag()
            .reciprocal()
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self::from_base(self.base_value().recip(), tag).into())
    }

    pub(crate) fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<Py<PyAny>> {
        let py = other.py();
        let Ok(rhs) = extract_quantity(other) else {
            // Equality against a foreign object is `False`, not an error, so a
            // quantity behaves in a heterogeneous container. An ordering
            // comparison against one stays an error.
            return match op {
                pyo3::basic::CompareOp::Eq => Ok(false.into_pyobject(py)?.to_owned().into()),
                pyo3::basic::CompareOp::Ne => Ok(true.into_pyobject(py)?.to_owned().into()),
                _ => Ok(py.NotImplemented()),
            };
        };

        if self.tag() != rhs.tag() {
            return match op {
                pyo3::basic::CompareOp::Eq => Ok(false.into_pyobject(py)?.to_owned().into()),
                pyo3::basic::CompareOp::Ne => Ok(true.into_pyobject(py)?.to_owned().into()),
                _ => Err(mismatch("order", self.tag(), rhs.tag())),
            };
        }

        let result = op.matches(
            self.base_value()
                .partial_cmp(&rhs.base_value())
                .ok_or_else(|| PyValueError::new_err("comparison against NaN is undefined"))?,
        );
        Ok(result.into_pyobject(py)?.to_owned().into())
    }
}
