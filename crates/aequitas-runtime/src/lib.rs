//! Exact runtime SI dimensions for Aequitas quantities and homogeneous buffers.
#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

mod dimension;
mod quantity;

pub use dimension::{Exponent, RuntimeDimension, StaticDimension};
pub use quantity::RuntimeQuantity;
