//! The extension module's registration surface.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::quantity::PyQuantity;
use crate::queries::{base_value_of, dimension_of, quantity_names, unit_symbols};
use crate::{protocol, units};

/// The registered extension surface.
///
/// # Errors
///
/// Propagates registration failures from the interpreter.
///
/// # Free-threaded interpreters
///
/// `gil_used = false` declares the module safe without the GIL; without it a
/// free-threaded interpreter turns the GIL back on for the whole process when
/// it imports this module. The declaration rests on an audit: every pyclass is
/// `frozen` over `Copy` data, so no method mutates shared state, and the
/// module holds no mutable state of its own -- the unit and class tables are
/// `const`.
#[pymodule(gil_used = false)]
fn _pyaequitas(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyQuantity>()?;
    crate::quantity::classes::register(module)?;
    module.add_function(wrap_pyfunction!(quantity_names, module)?)?;
    module.add_function(wrap_pyfunction!(unit_symbols, module)?)?;
    module.add_function(wrap_pyfunction!(dimension_of, module)?)?;
    module.add_function(wrap_pyfunction!(base_value_of, module)?)?;

    module.add("BASE_ATTR", protocol::BASE_ATTR)?;
    module.add("DIMENSION_ATTR", protocol::DIMENSION_ATTR)?;
    module.add("PROTOCOL_VERSION", protocol::PROTOCOL_VERSION)?;

    // The unit table is exported as data so the pure-Python layer can build
    // per-quantity constructors without a second inventory.
    let table = PyDict::new(module.py());
    for quantity in units::all() {
        let symbols: Vec<&str> = quantity.symbols().collect();
        table.set_item(quantity.name, symbols)?;
    }
    module.add("UNITS", table)?;

    Ok(())
}
