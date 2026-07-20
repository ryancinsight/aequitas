//! Representation and zero-sized marker invariants.

use core::mem::{align_of, size_of};

use aequitas::systems::si::{
    dimensions,
    quantities::Length,
    units::{Joule, Meter, Millimeter, Pascal, SquareMeterPerSecond},
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
    assert_eq!(size_of::<Meter>(), 0);
    assert_eq!(size_of::<Millimeter>(), 0);
    assert_eq!(size_of::<Pascal>(), 0);
    assert_eq!(size_of::<Joule>(), 0);
    assert_eq!(size_of::<SquareMeterPerSecond>(), 0);
}
