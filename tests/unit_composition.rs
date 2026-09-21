//! Value-semantic checks for generic linear-unit composition.

use core::fmt::Debug;

use aequitas::{
    Quantity,
    dimension::Dimension,
    quantity::UnitDisplay,
    systems::si::{
        quantities::{Area, Dimensionless, Intensity},
        units::{
            Centimeter, DegreeCelsius, Kelvin, Meter, Nanometer, Second, SquareCentimeter, Watt,
            WattPerSquareMeter,
        },
    },
    unit::{Power as UnitPower, Product, Quotient},
};
use eunomia::{Complex32, Complex64, UnitScalar};
use typenum::{N1, N2147483648, P1, P2, P20, P35, P2147483647, Z0};

fn assert_composed_intensity_matches_named<T>(value: T)
where
    T: UnitScalar + Copy + Debug + PartialEq,
{
    type Composed = Quotient<Watt, Product<Meter, Meter>>;

    let named = Intensity::<T>::from_unit::<WattPerSquareMeter>(value);
    let composed = Intensity::<T>::from_unit::<Composed>(value);

    assert_eq!(composed, named);
    assert_eq!(composed.in_unit::<WattPerSquareMeter>(), value);
}

#[test]
fn composed_intensity_matches_named_unit_for_supported_real_scalars() {
    assert_composed_intensity_matches_named(12.0_f32);
    assert_composed_intensity_matches_named(12.0_f64);
}

fn assert_centimeter_area_scale<T>(one: T, square_centimeters_per_square_meter: T)
where
    T: UnitScalar + Copy + Debug + PartialEq,
{
    type ProductArea = Product<Centimeter, Centimeter>;
    type PoweredArea = UnitPower<Centimeter, P2>;

    let named = Area::<T>::from_unit::<SquareCentimeter>(one);
    let product = Area::<T>::from_unit::<ProductArea>(one);
    let powered = Area::<T>::from_unit::<PoweredArea>(one);

    assert_eq!(product, named);
    assert_eq!(powered, named);
    assert_eq!(
        Area::<T>::from_unit::<ProductArea>(square_centimeters_per_square_meter),
        Area::<T>::from_base(one)
    );
}

#[test]
fn composed_centimeter_area_uses_the_product_scale() {
    assert_centimeter_area_scale(1.0_f32, 10_000.0_f32);
    assert_centimeter_area_scale(1.0_f64, 10_000.0_f64);
}

fn assert_scaled_denominator<T>(value: T, base_value: T)
where
    T: UnitScalar + Copy + Debug + PartialEq,
{
    type WattsPerSquareCentimeter = Quotient<Watt, UnitPower<Centimeter, P2>>;

    let composed = Intensity::<T>::from_unit::<WattsPerSquareCentimeter>(value);
    let coherent = Intensity::<T>::from_unit::<WattPerSquareMeter>(base_value);

    assert_eq!(composed, coherent);
}

#[test]
fn scaled_denominator_converts_watts_per_square_centimeter() {
    assert_scaled_denominator(12.0_f32, 120_000.0_f32);
    assert_scaled_denominator(12.0_f64, 120_000.0_f64);
}

#[test]
fn nested_quotients_and_inverse_powers_normalize_to_the_same_dimension() {
    type Grouped = Quotient<Watt, Product<Meter, Meter>>;
    type Nested = Quotient<Quotient<Watt, Meter>, Meter>;
    type InversePowers = Product<Watt, Product<UnitPower<Meter, N1>, UnitPower<Meter, N1>>>;

    let grouped = Intensity::from_unit::<Grouped>(7.5_f64);
    let nested = Intensity::from_unit::<Nested>(7.5_f64);
    let inverse_powers = Intensity::from_unit::<InversePowers>(7.5_f64);

    assert_eq!(nested, grouped);
    assert_eq!(inverse_powers, grouped);
}

#[test]
fn composed_unit_constructs_a_quantity_without_a_catalog_entry() {
    type SquareMeterSecond = Product<UnitPower<Meter, P2>, Second>;

    let quantity = Quantity::from_unit::<SquareMeterSecond>(3.25_f64);

    assert_eq!(quantity.into_base().to_bits(), 3.25_f64.to_bits());
}

#[test]
fn inferred_composed_dimension_matches_its_normalized_form() {
    type AreaTime = Dimension<P2, Z0, P1, Z0, Z0, Z0, Z0>;
    type SquareMeterSecond = Product<UnitPower<Meter, P2>, Second>;

    let inferred = Quantity::from_unit::<SquareMeterSecond>(3.25_f64);
    let normalized = Quantity::<f64, AreaTime>::from_base(3.25_f64);

    assert_eq!(inferred, normalized);
}

#[test]
fn zero_power_produces_a_dimensionless_unit() {
    type MeterToZero = UnitPower<Meter, Z0>;

    let quantity = Dimensionless::from_unit::<MeterToZero>(4.0_f64);

    assert_eq!(quantity.into_base().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn composed_unit_converts_complex_values_componentwise() {
    type WattsPerSquareCentimeter = Quotient<Watt, UnitPower<Centimeter, P2>>;

    let phasor = Complex64::new(12.0, -3.0);
    let intensity = Intensity::<Complex64>::from_unit::<WattsPerSquareCentimeter>(phasor);

    assert_eq!(
        intensity.in_unit::<WattPerSquareMeter>(),
        Complex64::new(120_000.0, -30_000.0)
    );
}

#[test]
fn small_power_scale_round_trips_native_single_precision() {
    type Tiny = UnitPower<Centimeter, P20>;

    for value in [1.0_f32, 0.0_f32] {
        let recovered = Quantity::from_unit::<Tiny>(value).in_unit::<Tiny>();
        assert_eq!(recovered.to_bits(), value.to_bits());
    }

    for value in [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)] {
        let recovered = Quantity::from_unit::<Tiny>(value).in_unit::<Tiny>();
        assert_eq!(recovered.re.to_bits(), value.re.to_bits());
        assert_eq!(recovered.im.to_bits(), value.im.to_bits());
    }
}

#[test]
fn small_power_scale_round_trips_native_double_precision() {
    type Tiny = UnitPower<Nanometer, P35>;

    for value in [1.0_f64, 0.0_f64] {
        let recovered = Quantity::from_unit::<Tiny>(value).in_unit::<Tiny>();
        assert_eq!(recovered.to_bits(), value.to_bits());
    }
}

#[test]
fn signed_32_bit_power_bounds_evaluate_and_display() {
    type Minimum = UnitPower<Meter, N2147483648>;
    type Maximum = UnitPower<Meter, P2147483647>;

    let minimum = Quantity::from_unit::<Minimum>(1.0_f64);
    let maximum = Quantity::from_unit::<Maximum>(1.0_f64);

    assert_eq!(minimum.in_unit::<Minimum>().to_bits(), 1.0_f64.to_bits());
    assert_eq!(maximum.in_unit::<Maximum>().to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        format!("{}", UnitDisplay::new(&minimum, Minimum::default())),
        "1 (m^-2147483648)"
    );
    assert_eq!(
        format!("{}", UnitDisplay::new(&maximum, Maximum::default())),
        "1 (m^2147483647)"
    );
}

#[test]
fn composed_celsius_rate_is_a_kelvin_difference_rate() {
    type CelsiusPerSecond = Quotient<DegreeCelsius, Second>;
    type KelvinPerSecond = Quotient<Kelvin, Second>;

    let celsius_rate = Quantity::from_unit::<CelsiusPerSecond>(12.5_f64);
    let kelvin_rate = Quantity::from_unit::<KelvinPerSecond>(12.5_f64);

    assert_eq!(celsius_rate, kelvin_rate);
    assert_eq!(
        celsius_rate.in_unit::<KelvinPerSecond>().to_bits(),
        12.5_f64.to_bits()
    );
}

#[test]
fn composed_unit_display_renders_the_expression() {
    type Composed = Quotient<Watt, Product<Meter, Meter>>;

    let intensity = Intensity::from_unit::<Composed>(12.0_f64);
    let rendered = format!("{}", UnitDisplay::new(&intensity, Composed::default()));

    assert_eq!(rendered, "12 (W/(m*m))");
}
