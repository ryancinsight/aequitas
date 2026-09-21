//! The Python-visible quantity and its arithmetic.
//!
//! The `#[pymethods]` surface is one class spread across leaves, each naming
//! a seam: `construct` builds one from units, `inspect` reads one back,
//! `arithmetic` operates on two, and `wire` renders and re-reads the
//! cross-extension form. `pyo3`'s `multiple-pymethods` feature is what makes
//! the division free of charge.

mod arithmetic;
pub(crate) mod classes;
mod construct;
mod inspect;
mod model;
mod wire;

pub use model::PyQuantity;
pub use wire::{dimension_tuple, extract_quantity};

#[cfg(test)]
mod tests;
