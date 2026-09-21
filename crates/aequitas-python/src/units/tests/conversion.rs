//! The differential oracle: registry conversions against the law crate's.
//!
//! Calls `Unit::to_base` and `Unit::from_base` -- the code `model.rs` runs --
//! rather than re-deriving the arithmetic, which would test nothing about the
//! binding. The oracle is bitwise equality over a multi-decade sweep, because
//! a handful of round numbers is exactly the input set that hid an ulp-level
//! disagreement the seven-point version of this test missed.

use aequitas::systems::si::quantities::{Area, Energy, Length, Mass, Pressure, Time, Volume};
use aequitas::systems::si::units::{
    Centimeter, CubicMillimeter, ElectronVolt, Gram, Kilopascal, MegaElectronVolt, Megapascal,
    Microsecond, Millimeter, Millisecond, Nanometer, SquareCentimeter,
};

use super::fixtures::{assert_exact_in_context, sweep};
use crate::units::by_name;

/// Assert that the registry's conversions equal Aequitas's, both directions.
///
/// Calls `Unit::to_base` and `Unit::from_base` -- the code `model.rs` runs --
/// rather than re-deriving the arithmetic, which would test nothing about the
/// binding.
macro_rules! assert_conversion_matches {
    ($quantity:literal, $alias:ident, $unit:ty, $symbol:literal) => {{
        let named = by_name($quantity).expect("quantity is registered");
        let unit = named.unit($symbol).expect("unit is registered");

        for value in sweep() {
            let through_registry = unit.to_base(value);
            let through_aequitas = $alias::<f64>::from_unit::<$unit>(value).into_base();
            assert_exact_in_context(
                through_registry,
                through_aequitas,
                &format!(
                    "{} {} into base diverges from Aequitas at {value:e}",
                    $quantity, $symbol
                ),
            );

            let back_through_registry = unit.from_base(through_aequitas);
            let back_through_aequitas =
                $alias::<f64>::from_base(through_aequitas).in_unit::<$unit>();
            assert_exact_in_context(
                back_through_registry,
                back_through_aequitas,
                &format!(
                    "{} {} out of base diverges from Aequitas at {value:e}",
                    $quantity, $symbol
                ),
            );
        }
    }};
}

#[test]
fn conversions_agree_with_the_law_crate() {
    assert_conversion_matches!("length", Length, Millimeter, "mm");
    assert_conversion_matches!("length", Length, Centimeter, "cm");
    assert_conversion_matches!("length", Length, Nanometer, "nm");
    assert_conversion_matches!("mass", Mass, Gram, "g");
    assert_conversion_matches!("time", Time, Millisecond, "ms");
    assert_conversion_matches!("time", Time, Microsecond, "\u{b5}s");
    assert_conversion_matches!("pressure", Pressure, Kilopascal, "kPa");
    assert_conversion_matches!("pressure", Pressure, Megapascal, "MPa");
    assert_conversion_matches!("energy", Energy, ElectronVolt, "eV");
    assert_conversion_matches!("energy", Energy, MegaElectronVolt, "MeV");
    assert_conversion_matches!("area", Area, SquareCentimeter, "cm\u{b2}");
    assert_conversion_matches!("volume", Volume, CubicMillimeter, "mm\u{b3}");
}
