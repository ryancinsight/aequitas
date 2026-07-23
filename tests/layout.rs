//! Representation and zero-sized marker invariants.

use core::mem::{align_of, size_of};

use aequitas::systems::si::{
    dimensions,
    quantities::Length,
    units::{
        Gray, Joule, JoulePerCubicMeter, JoulePerMilliliter, JoulePerMole, JoulePerMoleKelvin,
        JoulePerSquareMeter, Meter, Millimeter, Pascal, PerCentimeter, PerMeter, PerSecond,
        SquareCentimeterPerGram, SquareMeterPerKilogram, SquareMeterPerSecond,
    },
};
use eunomia::{Bf16, F16, F32, F64};

fn assert_transparent<T>() {
    assert_eq!(size_of::<Length<T>>(), size_of::<T>());
    assert_eq!(align_of::<Length<T>>(), align_of::<T>());
}

#[test]
fn quantity_layout_matches_scalar_layout() {
    assert_transparent::<f32>();
    assert_transparent::<f64>();
    assert_transparent::<F16>();
    assert_transparent::<Bf16>();
    assert_transparent::<F32>();
    assert_transparent::<F64>();
}

#[test]
fn dimension_and_unit_markers_occupy_no_storage() {
    assert_eq!(size_of::<dimensions::Length>(), 0);
    assert_eq!(size_of::<dimensions::Pressure>(), 0);
    assert_eq!(size_of::<dimensions::ThermalDiffusivity>(), 0);
    assert_eq!(size_of::<dimensions::AbsorbedDose>(), 0);
    assert_eq!(size_of::<dimensions::MolarEnergy>(), 0);
    assert_eq!(size_of::<dimensions::MolarHeatCapacity>(), 0);
    assert_eq!(size_of::<dimensions::ReciprocalLength>(), 0);
    assert_eq!(size_of::<dimensions::AreaPerMass>(), 0);
    assert_eq!(size_of::<dimensions::EnergyPerArea>(), 0);
    assert_eq!(size_of::<dimensions::EnergyPerVolume>(), 0);
    assert_eq!(size_of::<dimensions::TemperatureDifference>(), 0);
    assert_eq!(size_of::<Meter>(), 0);
    assert_eq!(size_of::<Millimeter>(), 0);
    assert_eq!(size_of::<Pascal>(), 0);
    assert_eq!(size_of::<Joule>(), 0);
    assert_eq!(size_of::<SquareMeterPerSecond>(), 0);
    assert_eq!(size_of::<Gray>(), 0);
    assert_eq!(size_of::<JoulePerMole>(), 0);
    assert_eq!(size_of::<JoulePerMoleKelvin>(), 0);
    assert_eq!(size_of::<PerSecond>(), 0);
    assert_eq!(size_of::<PerMeter>(), 0);
    assert_eq!(size_of::<PerCentimeter>(), 0);
    assert_eq!(size_of::<SquareMeterPerKilogram>(), 0);
    assert_eq!(size_of::<SquareCentimeterPerGram>(), 0);
    assert_eq!(size_of::<JoulePerSquareMeter>(), 0);
    assert_eq!(size_of::<JoulePerCubicMeter>(), 0);
    assert_eq!(size_of::<JoulePerMilliliter>(), 0);
}
