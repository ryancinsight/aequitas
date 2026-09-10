//! Unit contracts.

mod affine;
mod linear;

pub use affine::AffineUnit;
pub use linear::LinearUnit;

pub(crate) mod private {
    pub trait Sealed {}
}
