use crate::{
    systems::si::dimensions,
    unit::{
        LinearUnit, UnitDimension,
        private::{Named, Sealed},
    },
};

/// J·s, the coherent SI unit for [`Action`](dimensions::Action).
#[derive(Clone, Copy, Debug, Default)]
pub struct JouleSecond;
impl Sealed for JouleSecond {}
impl Named for JouleSecond {}
impl UnitDimension for JouleSecond {
    type Dimension = dimensions::Action;
}
impl LinearUnit<dimensions::Action> for JouleSecond {
    const SYMBOL: &'static str = "J·s";
    const SCALE: f64 = 1.0;
}

/// mol⁻¹, the coherent SI unit for [`ReciprocalAmountOfSubstance`](dimensions::ReciprocalAmountOfSubstance).
#[derive(Clone, Copy, Debug, Default)]
pub struct PerMole;
impl Sealed for PerMole {}
impl Named for PerMole {}
impl UnitDimension for PerMole {
    type Dimension = dimensions::ReciprocalAmountOfSubstance;
}
impl LinearUnit<dimensions::ReciprocalAmountOfSubstance> for PerMole {
    const SYMBOL: &'static str = "mol⁻¹";
    const SCALE: f64 = 1.0;
}

/// F/m, the coherent SI unit for [`Permittivity`](dimensions::Permittivity).
#[derive(Clone, Copy, Debug, Default)]
pub struct FaradPerMeter;
impl Sealed for FaradPerMeter {}
impl Named for FaradPerMeter {}
impl UnitDimension for FaradPerMeter {
    type Dimension = dimensions::Permittivity;
}
impl LinearUnit<dimensions::Permittivity> for FaradPerMeter {
    const SYMBOL: &'static str = "F/m";
    const SCALE: f64 = 1.0;
}

/// H/m, the coherent SI unit for [`MagneticPermeability`](dimensions::MagneticPermeability).
#[derive(Clone, Copy, Debug, Default)]
pub struct HenryPerMeter;
impl Sealed for HenryPerMeter {}
impl Named for HenryPerMeter {}
impl UnitDimension for HenryPerMeter {
    type Dimension = dimensions::MagneticPermeability;
}
impl LinearUnit<dimensions::MagneticPermeability> for HenryPerMeter {
    const SYMBOL: &'static str = "H/m";
    const SCALE: f64 = 1.0;
}

/// W/(m²·K⁴), the coherent SI unit for [`StefanBoltzmannCoefficient`](dimensions::StefanBoltzmannCoefficient).
#[derive(Clone, Copy, Debug, Default)]
pub struct WattPerSquareMeterKelvinToFourth;
impl Sealed for WattPerSquareMeterKelvinToFourth {}
impl Named for WattPerSquareMeterKelvinToFourth {}
impl UnitDimension for WattPerSquareMeterKelvinToFourth {
    type Dimension = dimensions::StefanBoltzmannCoefficient;
}
impl LinearUnit<dimensions::StefanBoltzmannCoefficient> for WattPerSquareMeterKelvinToFourth {
    const SYMBOL: &'static str = "W/(m²·K⁴)";
    const SCALE: f64 = 1.0;
}
