//! Unit contracts.

mod linear;

pub use linear::LinearUnit;

pub(crate) mod private {
    pub trait Sealed {}
}
