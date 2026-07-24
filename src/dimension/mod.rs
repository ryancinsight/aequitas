//! Type-level physical dimensions and their algebra.

mod algebra;
mod model;

pub use algebra::{DivideDimension, MultiplyDimension};
pub use model::{
    AbsoluteTemperatureSemantics, BaseSemantics, Dimension, SurfaceTensionSemantics,
    TemperatureDifferenceSemantics,
};
