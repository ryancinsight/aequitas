//! The affine contract: an offset belongs to a quantity, not to a symbol.
//!
//! `degC` names the ice point for a temperature and nothing for a difference,
//! so both halves live in one leaf -- were one lookup to serve both, every
//! temperature difference would be wrong by 273.15 K, and the differential
//! oracle below is what makes that visible rather than plausible.

use aequitas::systems::si::quantities::ThermodynamicTemperature;
use aequitas::systems::si::units::{DegreeCelsius, DegreeFahrenheit};

use super::fixtures::{assert_exact, assert_exact_in_context, sweep};
use crate::units::by_name;

/// `degC` names an offset for a temperature and none for a difference.
///
/// The affine contract's reason to exist, crossing into the binding: the
/// symbol resolves per quantity, so a Celsius temperature carries the ice
/// point and a Celsius difference carries nothing. Were one lookup to serve
/// both, every temperature difference would be wrong by 273.15 K.
#[test]
fn celsius_means_an_offset_for_a_temperature_and_none_for_a_difference() {
    let absolute = by_name("thermodynamic_temperature").expect("registered");
    let difference = by_name("temperature_difference").expect("registered");

    let absolute_celsius = absolute.unit("degC").expect("affine degC is registered");
    let difference_celsius = difference.unit("degC").expect("linear degC is registered");

    assert_exact(
        absolute_celsius
            .offset
            .expect("degC is affine for a temperature"),
        273.15,
    );
    assert!(
        difference_celsius.offset.is_none(),
        "a Celsius difference must carry no offset"
    );
    assert_exact(absolute_celsius.to_base(0.0), 273.15);
    assert_exact(difference_celsius.to_base(10.0), 10.0);
}

/// Assert the registry's affine conversions equal Aequitas's, both directions.
macro_rules! assert_affine_conversion_matches {
    ($unit:ty, $symbol:literal) => {{
        let named = by_name("thermodynamic_temperature").expect("registered");
        let unit = named.unit($symbol).expect("affine unit is registered");

        for value in sweep() {
            let through_registry = unit.to_base(value);
            let through_aequitas =
                ThermodynamicTemperature::<f64>::from_affine_unit::<$unit>(value).into_base();
            assert_exact_in_context(
                through_registry,
                through_aequitas,
                &format!("{} into base diverges from Aequitas at {value:e}", $symbol),
            );

            let back_through_registry = unit.from_base(through_aequitas);
            let back_through_aequitas =
                ThermodynamicTemperature::<f64>::from_base(through_aequitas)
                    .in_affine_unit::<$unit>();
            assert_exact_in_context(
                back_through_registry,
                back_through_aequitas,
                &format!(
                    "{} out of base diverges from Aequitas at {value:e}",
                    $symbol
                ),
            );
        }
    }};
}

#[test]
fn affine_conversions_agree_with_the_law_crate() {
    assert_affine_conversion_matches!(DegreeCelsius, "degC");
    assert_affine_conversion_matches!(DegreeFahrenheit, "degF");
}
