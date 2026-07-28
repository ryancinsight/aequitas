//! Atlas physical-quantity and dimensional-law foundation.
//!
//! Aequitas stores quantities in canonical SI base units and expresses
//! dimensions through zero-sized type parameters. Unit conversion is
//! monomorphized over Eunomia's [`eunomia::UnitScalar`] seam; real arithmetic
//! retains [`eunomia::FloatElement`] semantics and complex phasors use
//! Eunomia's native `Complex` representation. Neither path uses dynamic
//! dispatch, allocation, or runtime dimension metadata.

#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod dimension;
pub mod quantity;
pub mod systems;
pub mod unit;

pub use quantity::Quantity;
