//! Native scalar operations are the oracle at IEEE representational boundaries.

use aequitas::{Quantity, systems::si::dimensions};
use eunomia::{Complex, RealField};
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

fn trace() -> tracing::subscriber::DefaultGuard {
    tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
            .finish(),
    )
}

fn same_scalar<T: RealField>(actual: T, expected: T) {
    let _span = info_span!("assert", ?actual, ?expected).entered();
    if expected.is_nan() {
        assert!(actual.is_nan());
    } else {
        assert_eq!(actual, expected);
        assert_eq!(actual.is_sign_negative(), expected.is_sign_negative());
    }
}

fn square_root<T: RealField>(value: T) {
    let _span = info_span!("arrange", ?value).entered();
    let area = Quantity::<T, dimensions::Area>::from_base(value);
    let expected = value.sqrt();
    let actual: Quantity<T, dimensions::Length> = {
        let _span = info_span!("act").entered();
        area.sqrt()
    };
    same_scalar(actual.into_base(), expected);
}

#[test]
fn square_root_preserves_special_values() {
    let _trace = trace();
    for value in [
        0.0_f32,
        -0.0,
        1.0,
        -1.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ] {
        square_root(value);
    }
    for value in [
        0.0_f64,
        -0.0,
        1.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ] {
        square_root(value);
    }
}

#[test]
fn square_root_matches_native_rounding_across_binades() {
    let _trace = trace();
    // Every finite binade, including subnormals, at its endpoints and the
    // adjacent representable values on either side of the significand midpoint.
    for exponent in 0..255_u32 {
        for mantissa in [0, 1, 0x003f_ffff, 0x0040_0000, 0x007f_ffff] {
            square_root(f32::from_bits((exponent << 23) | mantissa));
        }
    }
    for exponent in 0..2047_u64 {
        for mantissa in [
            0,
            1,
            0x0007_ffff_ffff_ffff,
            0x0008_0000_0000_0000,
            0x000f_ffff_ffff_ffff,
        ] {
            square_root(f64::from_bits((exponent << 52) | mantissa));
        }
    }
}

fn complex_division<T: RealField>(values: &[T]) {
    for &real in values {
        for &divisor in values {
            let _span = info_span!("arrange", ?real, ?divisor).entered();
            let value = Complex::new(real, -real);
            let quantity = Quantity::<Complex<T>, dimensions::Length>::from_base(value);
            let (divided, assigned) = {
                let _span = info_span!("act").entered();
                let divided = quantity / divisor;
                let mut assigned = quantity;
                assigned /= divisor;
                (divided.into_base(), assigned.into_base())
            };
            for actual in [divided, assigned] {
                same_scalar(actual.re, value.re / divisor);
                same_scalar(actual.im, value.im / divisor);
            }
        }
    }
}

#[test]
fn complex_scalar_division_uses_native_component_quotients() {
    let _trace = trace();
    complex_division(&[
        0.0_f32,
        -0.0,
        f32::from_bits(1),
        -f32::from_bits(1),
        f32::MIN_POSITIVE,
        f32::MAX,
        -f32::MAX,
        3.0,
        7.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ]);
    complex_division(&[
        0.0_f64,
        -0.0,
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::MIN_POSITIVE,
        f64::MAX,
        -f64::MAX,
        3.0,
        7.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ]);
}

#[test]
fn signed_integer_power_endpoints_keep_value_and_dimension() {
    use typenum::{N2147483648, P2147483647, Z0};

    type Highest = aequitas::dimension::Dimension<P2147483647, Z0, Z0, Z0, Z0, Z0, Z0>;
    type Lowest = aequitas::dimension::Dimension<N2147483648, Z0, Z0, Z0, Z0, Z0, Z0>;

    let _trace = trace();
    let _arrange = info_span!("arrange").entered();
    let length = Quantity::<f64, dimensions::Length>::from_base(-1.0);
    let _act = info_span!("act").entered();
    let highest: Quantity<f64, Highest> = length.powi::<P2147483647>();
    let lowest: Quantity<f64, Lowest> = length.powi::<N2147483648>();
    let _assert = info_span!("assert").entered();
    same_scalar(highest.into_base(), -1.0);
    same_scalar(lowest.into_base(), 1.0);
}

fn min_max<T: RealField>(values: &[T]) {
    for &left in values {
        for &right in values {
            let a = Quantity::<T, dimensions::Length>::from_base(left);
            let b = Quantity::<T, dimensions::Length>::from_base(right);
            same_scalar(a.min(b).into_base(), left.min_scalar(right));
            same_scalar(a.max(b).into_base(), left.max_scalar(right));
        }
    }
}

#[test]
fn extrema_follow_provider_nan_and_signed_zero_contract() {
    let _trace = trace();
    let _arrange = info_span!("arrange").entered();
    let _act = info_span!("act").entered();
    min_max(&[
        f32::NAN,
        f32::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        1.0,
        f32::INFINITY,
    ]);
    min_max(&[
        f64::NAN,
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        1.0,
        f64::INFINITY,
    ]);
}
