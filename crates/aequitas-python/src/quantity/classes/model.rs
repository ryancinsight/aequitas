//! The class table and the conversion that consults it.

use core::ops::Deref;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyModule};

use super::inventory::CLASSES;
use crate::quantity::PyQuantity;
use crate::tag::DimensionTag;

/// Build an instance of one generated class around a quantity.
pub(crate) type Instantiate = for<'py> fn(Python<'py>, PyQuantity) -> PyResult<Bound<'py, PyAny>>;

/// Add one generated class to the extension module.
pub(crate) type Register = fn(&Bound<'_, PyModule>) -> PyResult<()>;

/// One Python class: the dimension it stands for and every name it has.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Class {
    /// Python class name: the first Aequitas alias of the dimension, which is
    /// also the name `Quantity.quantity` reports for it.
    pub(crate) name: &'static str,
    /// Runtime image of the dimension.
    pub(crate) tag: DimensionTag,
    /// The dimension's other Aequitas aliases, bound as further names.
    pub(crate) aliases: &'static [&'static str],
    /// Instantiate the class around a quantity of this dimension.
    pub(crate) instantiate: Instantiate,
    /// Add the class to the extension module.
    pub(crate) register: Register,
}

/// The class for `tag`, when a named dimension has one.
#[must_use]
pub(crate) fn class_for(tag: DimensionTag) -> Option<&'static Class> {
    CLASSES.iter().find(|class| class.tag == tag)
}

/// True when no two classes share a dimension.
///
/// Evaluated in a `const` assertion over the generated table, so a grouping
/// the generator got wrong fails the build instead of producing two classes
/// arithmetic cannot choose between.
#[must_use]
pub(crate) const fn tags_distinct(classes: &[Class]) -> bool {
    let mut first = 0;
    while first < classes.len() {
        let mut second = first + 1;
        while second < classes.len() {
            if classes[first].tag.same_as(classes[second].tag) {
                return false;
            }
            second += 1;
        }
        first += 1;
    }
    true
}

/// Add every class to `module`, bind each alias as a further name of its
/// class, and export the full name-to-class table as `CLASSES`.
///
/// # Errors
///
/// Propagates registration failures from the interpreter.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let table = PyDict::new(module.py());
    for class in CLASSES {
        (class.register)(module)?;
        let object = module.getattr(class.name)?;
        table.set_item(class.name, &object)?;
        for &alias in class.aliases {
            module.add(alias, &object)?;
            table.set_item(alias, &object)?;
        }
    }
    module.add("CLASSES", table)
}

/// A quantity on its way to Python, where it becomes an instance of its
/// dimension's class.
///
/// Every operation that yields a quantity returns this rather than the base
/// class, so two lengths multiply to an `Area` a type checker can name. A
/// dimension no class names -- `m^5`, arising mid-derivation -- stays a plain
/// `Quantity`, which is what the stubs promise for it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Classed(PyQuantity);

impl From<PyQuantity> for Classed {
    fn from(quantity: PyQuantity) -> Self {
        Self(quantity)
    }
}

impl Deref for Classed {
    type Target = PyQuantity;

    fn deref(&self) -> &PyQuantity {
        &self.0
    }
}

impl<'py> IntoPyObject<'py> for Classed {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        match class_for(self.0.tag()) {
            Some(class) => (class.instantiate)(py, self.0),
            None => Bound::new(py, self.0).map(Bound::into_any),
        }
    }
}
