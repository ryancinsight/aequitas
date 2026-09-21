//! Standard arithmetic for quantities.
//!
//! `additive` is the dimension-generic kernel and `affine` the temperature
//! pairs that relate two dimensions instead of preserving one;
//! `multiplicative` composes the dimension algebra, while `scalar` and `unary`
//! cover the elementwise operations.

mod additive;
mod affine;
mod multiplicative;
mod scalar;
mod unary;
