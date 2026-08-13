//! Type-level physical dimensions and their algebra.

mod algebra;
mod model;
mod root;

pub use algebra::{DivideDimension, MultiplyDimension};
pub use model::{
    AbsoluteTemperatureSemantics, AngleSemantics, BaseSemantics, Dimension,
    FlexuralRigiditySemantics, MechanicalImpedanceSemantics, MolarConcentrationSemantics,
    ReciprocalVolumeSemantics, SpringStiffnessSemantics, SurfaceTensionSemantics,
    TemperatureDifferenceSemantics,
};
pub use root::{CbrtDimension, SqrtDimension};
