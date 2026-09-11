//! One Python class per named dimension, over the one runtime quantity.
//!
//! Aequitas names 81 quantities over 73 distinct dimensions. Each distinct
//! dimension becomes a subclass of `Quantity` named after its first alias --
//! `Length`, `Area`, `AreaPerTime` -- and every operation that yields a
//! quantity instantiates the class for the result's dimension, so a type
//! checker reading the stubs sees the same classes the interpreter creates.
//!
//! Aliases that name one Rust dimension type (`ThermalDiffusivity` and
//! `KinematicViscosity` are both `AreaPerTime`) are one class, bound under
//! each name, because a result carries a dimension and no name: a class per
//! alias would leave arithmetic unable to say which to return.

mod declare;
mod inventory;
mod model;

pub(crate) use model::{Class, Classed, register, tags_distinct};

#[cfg(test)]
mod tests;
