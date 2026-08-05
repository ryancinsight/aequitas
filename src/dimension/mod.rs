//! Type-level physical dimensions and their algebra.

mod algebra;
mod model;

pub use algebra::{DivideDimension, MultiplyDimension};
pub use model::{
    AbsoluteTemperatureSemantics, AngleSemantics, BaseSemantics, Dimension,
    MechanicalImpedanceSemantics, MolarConcentrationSemantics, ReciprocalVolumeSemantics,
    SpringStiffnessSemantics, SurfaceTensionSemantics, TemperatureDifferenceSemantics,
};
