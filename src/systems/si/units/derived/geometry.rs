use crate::{
    systems::si::dimensions,
    unit::{LinearUnit, private::Sealed},
};

/// Square metre, the coherent SI unit of area.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeter;
impl Sealed for SquareMeter {}
impl crate::unit::private::Named for SquareMeter {}
impl crate::unit::UnitDimension for SquareMeter {
    type Dimension = dimensions::Area;
}

impl LinearUnit<dimensions::Area> for SquareMeter {
    const SYMBOL: &'static str = "m²";
    const SCALE: f64 = 1.0;
}

/// Per metre, the coherent SI unit of reciprocal length.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerMeter;
impl Sealed for PerMeter {}
impl crate::unit::private::Named for PerMeter {}
impl crate::unit::UnitDimension for PerMeter {
    type Dimension = dimensions::ReciprocalLength;
}

impl LinearUnit<dimensions::ReciprocalLength> for PerMeter {
    const SYMBOL: &'static str = "m⁻¹";
    const SCALE: f64 = 1.0;
}

/// Square metre per kilogram, the coherent SI unit of area per mass.
#[derive(Clone, Copy, Debug, Default)]
pub struct SquareMeterPerKilogram;
impl Sealed for SquareMeterPerKilogram {}
impl crate::unit::private::Named for SquareMeterPerKilogram {}
impl crate::unit::UnitDimension for SquareMeterPerKilogram {
    type Dimension = dimensions::AreaPerMass;
}

impl LinearUnit<dimensions::AreaPerMass> for SquareMeterPerKilogram {
    const SYMBOL: &'static str = "m²/kg";
    const SCALE: f64 = 1.0;
}

/// Cubic metre, the coherent SI unit of volume.
#[derive(Clone, Copy, Debug, Default)]
pub struct CubicMeter;
impl Sealed for CubicMeter {}
impl crate::unit::private::Named for CubicMeter {}
impl crate::unit::UnitDimension for CubicMeter {
    type Dimension = dimensions::Volume;
}

impl LinearUnit<dimensions::Volume> for CubicMeter {
    const SYMBOL: &'static str = "m³";
    const SCALE: f64 = 1.0;
}

/// Per cubic metre, the coherent SI unit of number density.
#[derive(Clone, Copy, Debug, Default)]
pub struct PerCubicMeter;
impl Sealed for PerCubicMeter {}
impl crate::unit::private::Named for PerCubicMeter {}
impl crate::unit::UnitDimension for PerCubicMeter {
    type Dimension = dimensions::NumberDensity;
}

impl LinearUnit<dimensions::NumberDensity> for PerCubicMeter {
    const SYMBOL: &'static str = "m⁻³";
    const SCALE: f64 = 1.0;
}
