//! Exact runtime SI dimensions for Aequitas quantities and homogeneous buffers.
#![doc = include_str!("../README.md")]
#![no_std]

mod dimension;
mod quantity;

pub use dimension::{Exponent, RuntimeDimension, StaticDimension};
pub use quantity::RuntimeQuantity;
