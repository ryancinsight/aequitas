//! Atlas physical-quantity and dimensional-law foundation.
//!
//! Aequitas stores quantities in canonical SI base units and expresses
//! dimensions through zero-sized type parameters. Arithmetic is monomorphized
//! over the underlying [`eunomia::FloatElement`] without dynamic dispatch,
//! allocation, or runtime dimension metadata.

#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod dimension;
pub mod quantity;
pub mod systems;
pub mod unit;

pub use quantity::Quantity;
