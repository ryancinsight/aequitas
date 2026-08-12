//! Value-semantic checks for [`UnitDisplay`] formatting.

use aequitas::quantity::UnitDisplay;
use aequitas::systems::si::{
    quantities::{Energy, Length, Velocity},
    units::{Joule, Kilometer, MeterPerSecond},
};

#[test]
fn velocity_displays_value_with_unit_symbol() {
    let v = Velocity::from_base(2.5); // 2.5 m/s in canonical SI base
    let rendered = format!("{}", UnitDisplay::new(&v, MeterPerSecond));
    assert_eq!(rendered, "2.5 m/s");
}

#[test]
fn length_displays_scaled_unit() {
    let l = Length::from_base(1500.0); // 1500 m
    let rendered = format!("{}", UnitDisplay::new(&l, Kilometer));
    assert_eq!(rendered, "1.5 km");
}

#[test]
fn energy_displays_derived_unit() {
    let e = Energy::from_base(3.0); // 3 J
    let rendered = format!("{}", UnitDisplay::new(&e, Joule));
    assert_eq!(rendered, "3 J");
}

#[test]
fn unit_display_debug_matches_display() {
    let v = Velocity::from_base(1.0);
    let wrapper = UnitDisplay::new(&v, MeterPerSecond);
    assert_eq!(format!("{wrapper}"), format!("{wrapper:?}"));
}

#[test]
fn unit_display_does_not_mutate_the_quantity() {
    let v = Velocity::from_base(2.5);
    let _ = format!("{}", UnitDisplay::new(&v, MeterPerSecond));
    // The quantity still holds its canonical base value.
    assert!((*v.as_base() - 2.5_f64).abs() < f64::EPSILON);
}
