//! Property checks for bounded linear-unit round trips.

use aequitas::systems::si::{
    quantities::{Energy, Length, MassDensity},
    units::{GramPerCubicCentimeter, Kilometer, MegaElectronVolt},
};
use proptest::prelude::*;

fn round_trip_bound(value: f64) -> f64 {
    // One multiplication and one division contribute at most two elementary
    // rounding errors. The factor of four covers their composition and the
    // conversion coefficient's representation error.
    4.0 * f64::EPSILON * value.abs().max(1.0)
}

proptest! {
    #[test]
    fn kilometer_round_trip_respects_floating_point_bound(
        value in -1.0e12_f64..1.0e12_f64
    ) {
        let quantity = Length::from_unit::<Kilometer>(value);
        let recovered = quantity.in_unit::<Kilometer>();

        prop_assert!((recovered - value).abs() <= round_trip_bound(value));
    }

    #[test]
    fn density_round_trip_respects_floating_point_bound(
        value in -1.0e9_f64..1.0e9_f64
    ) {
        let quantity = MassDensity::from_unit::<GramPerCubicCentimeter>(value);
        let recovered = quantity.in_unit::<GramPerCubicCentimeter>();

        prop_assert!((recovered - value).abs() <= round_trip_bound(value));
    }

    #[test]
    fn energy_round_trip_respects_floating_point_bound(
        value in -1.0e9_f64..1.0e9_f64
    ) {
        let quantity = Energy::from_unit::<MegaElectronVolt>(value);
        let recovered = quantity.in_unit::<MegaElectronVolt>();

        prop_assert!((recovered - value).abs() <= round_trip_bound(value));
    }
}
