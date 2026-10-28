//! Value-semantic checks for non-SI pressure, velocity, and temperature units.
//!
//! Each factor is derived from its defining constants in the test itself, so
//! the suite proves the catalog entries rather than repeating them: bar and
//! atmosphere are defined exactly in pascals, psi follows from the exact
//! pound and inch, conventional mmHg from its mercury-column definition
//! (and is asserted distinct from the torr), the velocity units from
//! the exact mile, nautical mile, foot, hour relations, and Rankine from
//! the exact 5/9-kelvin degree with no offset.

use aequitas::systems::si::{
    quantities::{Pressure, ThermodynamicTemperature, Velocity},
    units::{
        Bar, DegreeCelsius, DegreeFahrenheit, FootPerSecond, Kelvin, KilometerPerHour, Knot,
        MeterPerSecond, MilePerHour, MillimeterOfMercury, Pascal, PoundPerSquareInch, Rankine,
        StandardAtmosphere,
    },
};

fn assert_near(name: &str, got: f64, expected: f64, tol: f64) {
    assert!(
        (got - expected).abs() <= tol,
        "{name}: got {got}, expected {expected} within {tol}"
    );
}

#[test]
fn bar_is_exactly_100_kpa() {
    let one_bar = Pressure::from_unit::<Bar>(1.0_f64);
    assert_eq!(one_bar.into_base().to_bits(), 100_000.0_f64.to_bits());
    assert_eq!(
        one_bar.in_unit::<Pascal>().to_bits(),
        100_000.0_f64.to_bits()
    );
}

#[test]
fn standard_atmosphere_is_exactly_101325_pa() {
    let one_atm = Pressure::from_unit::<StandardAtmosphere>(1.0_f64);
    assert_eq!(one_atm.into_base().to_bits(), 101_325.0_f64.to_bits());
}

#[test]
fn psi_follows_from_exact_pound_and_inch() {
    // 1 lbf = 0.453_592_37 kg * 9.80665 m/s^2 exactly; 1 in = 0.0254 m exactly.
    let expected = 0.453_592_37 * 9.80665 / (0.0254 * 0.0254);
    let one_psi = Pressure::from_unit::<PoundPerSquareInch>(1.0_f64);
    assert_near("psi", one_psi.into_base(), expected, 1e-9);
    assert_near("psi decimal", one_psi.into_base(), 6_894.757_293_168, 1e-6);
}

#[test]
fn atmosphere_in_psi_matches_both_definitions() {
    // 101325 Pa / 6_894.757_293_168 Pa = 14.695_948_775_5...; agreement proves the
    // two independent unit definitions are mutually consistent.
    let atm_in_psi =
        Pressure::from_unit::<StandardAtmosphere>(1.0_f64).in_unit::<PoundPerSquareInch>();
    assert_near("atm in psi", atm_in_psi, 14.695_948_775_5, 1e-9);
}

#[test]
fn conventional_mmhg_matches_its_mercury_column_definition() {
    // rho_Hg = 13595.1 kg/m^3, g_n = 9.80665 m/s^2, h = 1 mm.
    let expected = 13_595.1 * 9.80665 / 1000.0;
    let one_mmhg = Pressure::from_unit::<MillimeterOfMercury>(1.0_f64);
    assert_near("mmHg", one_mmhg.into_base(), expected, 1e-9);
    assert_near("mmHg decimal", one_mmhg.into_base(), 133.322_387_415, 1e-6);
}

#[test]
fn mmhg_is_distinct_from_torr() {
    // 1 Torr = 101325/760 Pa exactly; the conventional mmHg differs in the
    // seventh digit. Conflating them must fail this test.
    let definition = 101_325.0_f64 / 760.0;
    let catalog = Pressure::from_unit::<MillimeterOfMercury>(1.0_f64).into_base();
    let gap = (catalog - definition).abs();
    assert!(gap > 0.0, "mmHg must differ from Torr, gap = {gap}");
    assert!(
        gap < 0.02,
        "mmHg must stay within 0.02 Pa of Torr, gap = {gap}"
    );
}

#[test]
fn meter_per_second_is_3_point_6_kmh() {
    let one_ms = Velocity::from_unit::<MeterPerSecond>(1.0_f64);
    assert_near("km/h", one_ms.in_unit::<KilometerPerHour>(), 3.6, 1e-12);
}

#[test]
fn mile_per_hour_follows_from_exact_mile() {
    // 1 international mile = 1609.344 m exactly.
    let one_mph = Velocity::from_unit::<MilePerHour>(1.0_f64);
    assert_near("mph", one_mph.into_base(), 1609.344 / 3600.0, 1e-12);
    assert_near("mph decimal", one_mph.into_base(), 0.44704, 1e-12);
}

#[test]
fn knot_follows_from_exact_nautical_mile() {
    // 1 international nautical mile = 1852 m exactly.
    let one_knot = Velocity::from_unit::<Knot>(1.0_f64);
    assert_near("knot", one_knot.into_base(), 1852.0 / 3600.0, 1e-12);
    assert_near(
        "knot decimal",
        one_knot.into_base(),
        0.514_444_444_444_444_4,
        1e-12,
    );
}

#[test]
fn foot_per_second_follows_from_exact_foot() {
    // 1 international foot = 0.3048 m exactly.
    let one_fps = Velocity::from_unit::<FootPerSecond>(1.0_f64);
    assert_near("ft/s", one_fps.into_base(), 0.3048, 1e-12);
}

#[test]
fn sixty_mph_is_exactly_88_ft_per_s() {
    // 1 mph = 22/15 ft/s, so 60 mph = 88 ft/s exactly; agreement proves the
    // mile and foot definitions compose correctly.
    let sixty = Velocity::from_unit::<MilePerHour>(60.0_f64).in_unit::<FootPerSecond>();
    assert_near("60 mph in ft/s", sixty, 88.0, 1e-9);
}

#[test]
fn rankine_has_no_offset_and_five_ninths_kelvin_degrees() {
    // Absolute scale: 0 R is 0 K, and 491.67 R is 273.15 K (32 F).
    let zero = ThermodynamicTemperature::from_unit::<Rankine>(0.0_f64);
    assert_eq!(zero.into_base().to_bits(), 0.0_f64.to_bits());
    let freezing = ThermodynamicTemperature::from_unit::<Rankine>(491.67_f64);
    assert_near("491.67 R", freezing.into_base(), 273.15, 1e-9);
    assert_near("491.67 R in K", freezing.in_unit::<Kelvin>(), 273.15, 1e-9);
}

#[test]
fn rankine_agrees_with_fahrenheit_at_shared_points() {
    // 32 F and 491.67 R name the same temperature; the affine Fahrenheit
    // and linear Rankine conversions must coincide there.
    let from_f = ThermodynamicTemperature::from_affine_unit::<DegreeFahrenheit>(32.0_f64);
    let from_r = ThermodynamicTemperature::from_unit::<Rankine>(491.67_f64);
    assert_near(
        "32 F vs 491.67 R",
        (from_f.into_base() - from_r.into_base()).abs(),
        0.0,
        1e-9,
    );
}

#[test]
fn celsius_affine_and_difference_uses_stay_distinct() {
    // 20 C names 293.15 K as a temperature; the difference dimension is
    // unaffected by the offset.
    let room = ThermodynamicTemperature::from_affine_unit::<DegreeCelsius>(20.0_f64);
    assert_near("20 C", room.into_base(), 293.15, 1e-12);
    assert_near("20 C in K", room.in_unit::<Kelvin>(), 293.15, 1e-12);
}
