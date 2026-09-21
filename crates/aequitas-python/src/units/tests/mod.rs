//! Conformance of the generated inventory against the law crate.
//!
//! These assert the property the whole binding rests on: a conversion made
//! through the registry equals the one Aequitas makes through `in_unit`. The
//! registry reads `LinearUnit`'s associated constants, so a divergence here
//! means the *inventory* is wrong -- a quantity mapped to the wrong dimension
//! -- not that a scale was mistyped, which the design makes unrepresentable.
//!
//! The leaves divide by the contract under test: the shape of the generated
//! inventory, lookup and aliasing, the differential conversion oracle, and the
//! affine offset contract.

mod affine;
mod conversion;
mod fixtures;
mod inventory;
mod lookup;
