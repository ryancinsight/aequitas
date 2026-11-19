//! Python images of the two kinds of logarithmic level.

use aequitas::Quantity;
use aequitas::systems::si::{
    dimensions::Dimensionless,
    levels::{AmplitudeLevel, Bel, Decibel, LogarithmicUnit, Neper, PowerLevel},
};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::quantity::PyQuantity;

#[pyclass(
    name = "AmplitudeLevel",
    module = "pyaequitas._pyaequitas",
    frozen,
    from_py_object
)]
#[derive(Clone, Copy)]
pub(crate) struct PyAmplitudeLevel(AmplitudeLevel);

#[pymethods]
impl PyAmplitudeLevel {
    #[new]
    #[pyo3(signature = (value, unit = "Np"))]
    fn new(value: f64, unit: &str) -> PyResult<Self> {
        match unit {
            Neper::SYMBOL | "Neper" => Ok(Self(AmplitudeLevel::from_unit::<Neper>(value))),
            _ => Err(invalid_unit(unit, "amplitude")),
        }
    }

    #[staticmethod]
    fn from_ratio(value: PyQuantity, reference: PyQuantity) -> PyResult<Self> {
        let (value, reference) = ratio_operands(value, reference)?;
        AmplitudeLevel::from_ratio(value, reference)
            .map(Self)
            .ok_or_else(invalid_ratio)
    }

    fn in_unit(slf: &Bound<'_, Self>, unit: &str) -> PyResult<f64> {
        match unit {
            Neper::SYMBOL | "Neper" => Ok(slf.get().0.in_unit::<Neper>()),
            _ => Err(invalid_unit(unit, "amplitude")),
        }
    }

    fn ratio(slf: &Bound<'_, Self>) -> f64 {
        slf.get().0.ratio()
    }

    fn square_law_power(slf: &Bound<'_, Self>) -> PyPowerLevel {
        PyPowerLevel(slf.get().0.square_law_power())
    }
}

#[pyclass(
    name = "PowerLevel",
    module = "pyaequitas._pyaequitas",
    frozen,
    from_py_object
)]
#[derive(Clone, Copy)]
pub(crate) struct PyPowerLevel(PowerLevel);

#[pymethods]
impl PyPowerLevel {
    #[new]
    #[pyo3(signature = (value, unit = "dB"))]
    fn new(value: f64, unit: &str) -> PyResult<Self> {
        match unit {
            Bel::SYMBOL | "Bel" => Ok(Self(PowerLevel::from_unit::<Bel>(value))),
            Decibel::SYMBOL | "Decibel" => Ok(Self(PowerLevel::from_unit::<Decibel>(value))),
            _ => Err(invalid_unit(unit, "power")),
        }
    }

    #[staticmethod]
    fn from_ratio(value: PyQuantity, reference: PyQuantity) -> PyResult<Self> {
        let (value, reference) = ratio_operands(value, reference)?;
        PowerLevel::from_ratio(value, reference)
            .map(Self)
            .ok_or_else(invalid_ratio)
    }

    fn in_unit(slf: &Bound<'_, Self>, unit: &str) -> PyResult<f64> {
        match unit {
            Bel::SYMBOL | "Bel" => Ok(slf.get().0.in_unit::<Bel>()),
            Decibel::SYMBOL | "Decibel" => Ok(slf.get().0.in_unit::<Decibel>()),
            _ => Err(invalid_unit(unit, "power")),
        }
    }

    fn ratio(slf: &Bound<'_, Self>) -> f64 {
        slf.get().0.ratio()
    }

    fn square_law_amplitude(slf: &Bound<'_, Self>) -> PyAmplitudeLevel {
        PyAmplitudeLevel(slf.get().0.square_law_amplitude())
    }
}

fn ratio_operands(
    value: PyQuantity,
    reference: PyQuantity,
) -> PyResult<(Quantity<f64, Dimensionless>, Quantity<f64, Dimensionless>)> {
    if value.tag() != reference.tag() {
        return Err(PyValueError::new_err(
            "a level requires a reference of the same quantity kind",
        ));
    }
    // The runtime tags have established equality of physical kind and both
    // inputs use canonical units, so the shared dimension cancels in the ratio.
    Ok((
        Quantity::from_base(value.base_value()),
        Quantity::from_base(reference.base_value()),
    ))
}

fn invalid_unit(unit: &str, kind: &str) -> PyErr {
    PyValueError::new_err(format!(
        "unit '{unit}' is invalid for a logarithmic {kind} level"
    ))
}

fn invalid_ratio() -> PyErr {
    PyValueError::new_err("a level requires positive, finite measurements")
}
