//! Value-semantic tests for the Python-visible quantity.
//!
//! These run against a real interpreter so the `#[pymethods]` surface is
//! exercised as Python sees it, not as Rust would call it. The leaves
//! divide by the contract under test.

mod arithmetic;
mod comparison;
mod construction;
mod fixtures;
mod protocol;
