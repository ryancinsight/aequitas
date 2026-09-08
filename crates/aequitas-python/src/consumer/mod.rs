//! The entry point another extension module uses to accept quantities.
//!
//! A consumer wants one thing: a parameter that today is a bare `f64` in
//! canonical SI base units should keep accepting that float and additionally
//! accept a quantity, with the dimension checked. [`Dimensioned`] is that
//! parameter type, so a call site changes from `f64` to
//! `Dimensioned<dimensions::Length>` and nothing else.
//!
//! Nothing here touches [`crate::PyQuantity`]. A consumer that never registers
//! that class must not be made to instantiate its type object, and a
//! structural read is what keeps the two wheels' versions independent
//! (see [`crate::protocol`]).

mod dimensioned;
mod value;

#[cfg(test)]
mod tests;

pub use dimensioned::Dimensioned;
pub use value::read;
