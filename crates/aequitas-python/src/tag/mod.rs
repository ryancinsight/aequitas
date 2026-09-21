//! Runtime image of Aequitas' type-level dimensions.
//!
//! Aequitas discharges dimensional correctness during monomorphization and
//! keeps no runtime metadata. Python has no compile step, so the boundary
//! carries a value that reproduces the same algebra: the seven SI base
//! exponents and the semantic discriminant, derived from the type parameters
//! rather than transcribed beside them.

mod algebra;
mod derive;
mod error;
mod model;
mod semantics;

pub use derive::TaggedDimension;
pub use error::{TagNotDivisible, TagOverflow};
pub use model::{AXES, AXIS_SYMBOLS, DimensionTag};
pub use semantics::{SemanticTag, TaggedSemantics};

#[cfg(test)]
mod tests;
