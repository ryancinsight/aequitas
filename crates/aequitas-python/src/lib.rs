//! Python bindings for the Aequitas physical-quantity surface.
//!
//! Aequitas encodes SI dimensions in types: `Quantity<T, D>` is
//! `#[repr(transparent)]` over `T` with `D` in `PhantomData`, so dimensional
//! correctness is discharged during monomorphization and erased before the
//! binary exists. There is nothing left at runtime for Python to hold.
//!
//! This crate therefore exports a runtime image rather than the types
//! themselves: a magnitude in canonical SI base units carried alongside the
//! seven SI exponents and a semantic discriminant. Both halves are derived
//! from the Rust type parameters (see [`tag::TaggedDimension`]) rather than
//! transcribed, and every unit symbol and scale factor is read from the
//! `LinearUnit` associated constants, so the law crate stays the single source
//! of truth and this surface cannot disagree with it about what a unit means.
//!
//! Dimensional violations that `rustc` rejects at compile time surface here as
//! Python exceptions. Static checking is recovered in the consumer's own type
//! checker through the generated stubs, not abandoned.
//!
//! # Layering
//!
//! `aequitas` itself never depends on `pyo3`; this crate is the only place the
//! two meet.

#![deny(missing_docs)]

pub mod consumer;
mod module;
pub mod protocol;
pub mod quantity;
mod queries;
pub mod tag;
pub mod units;

pub use consumer::Dimensioned;
pub use quantity::PyQuantity;
