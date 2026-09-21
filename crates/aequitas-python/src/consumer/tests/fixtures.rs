//! Shared fixtures: the object builder and the foreign quantities.
//!
//! `build` runs a snippet in a fresh namespace, so each case states its object
//! as Python source -- which is what the extractor actually meets -- rather
//! than as a Rust-side stand-in that could not fail the same way.

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Bind a Python object built by `source`, whose last statement assigns
/// `result`.
pub(super) fn build<'py>(py: Python<'py>, source: &str) -> Bound<'py, PyAny> {
    let globals = PyDict::new(py);
    py.run(
        &std::ffi::CString::new(source).expect("no interior nul"),
        Some(&globals),
        None,
    )
    .expect("the fixture defines");
    globals
        .get_item("result")
        .expect("lookup")
        .expect("`result` is assigned")
}

/// A foreign object carrying the length dimension.
pub(super) const A_LENGTH: &str = r"
class Foreign:
    __aequitas_base__ = 0.0025
    __aequitas_dimension__ = ((1, 0, 0, 0, 0, 0, 0), 'base')

result = Foreign()
";

/// A foreign object carrying the time dimension.
pub(super) const A_TIME: &str = r"
class Foreign:
    __aequitas_base__ = 2.0
    __aequitas_dimension__ = ((0, 0, 1, 0, 0, 0, 0), 'base')

result = Foreign()
";

/// A foreign object carrying the stress marker.
pub(super) const A_STRESS: &str = r"
class Foreign:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((-1, 1, -2, 0, 0, 0, 0), 'stress')

result = Foreign()
";

/// A foreign object carrying the pressure dimension.
pub(super) const A_PRESSURE: &str = r"
class Foreign:
    __aequitas_base__ = 1.0
    __aequitas_dimension__ = ((-1, 1, -2, 0, 0, 0, 0), 'base')

result = Foreign()
";
