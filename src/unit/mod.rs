//! Unit contracts.

mod affine;
mod expression;
mod linear;
mod power;
mod product;
mod quotient;

pub use affine::AffineUnit;
pub use expression::{Unit, UnitDimension};
pub use linear::LinearUnit;
pub use power::Power;
pub use product::Product;
pub use quotient::Quotient;

pub(crate) mod private {
    pub trait Sealed {}
    pub trait Named: Sealed + super::UnitDimension {}
}
