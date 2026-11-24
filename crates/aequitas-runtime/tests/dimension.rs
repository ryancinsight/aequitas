//! Exact SI algebra for static and dynamically selected powers.
use aequitas::systems::si::{dimensions, quantities::Pressure};
use aequitas_runtime::{Exponent, RuntimeDimension, RuntimeQuantity};
use proptest::prelude::*;

#[test]
fn pressure_per_root_second() {
    let subscriber = tracing_subscriber::fmt()
        .with_test_writer()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        let pressure = tracing::info_span!("arrange")
            .in_scope(|| RuntimeQuantity::from(Pressure::from_base([3_i32, 4])));
        let time = RuntimeDimension::of::<dimensions::Time>();
        let half = Exponent::ratio(1, 2).expect("nonzero denominator");
        let units = tracing::info_span!("act")
            .in_scope(|| pressure.dimension().quotient(&time.power(&half)));
        tracing::info_span!("assert").in_scope(|| {
            assert_eq!(units.axes()[2], Exponent::ratio(-5, 2).expect("two"));
            assert_eq!(units.product(&time.power(&half)), *pressure.dimension());
            assert_eq!(pressure.as_base(), &[3, 4]);
            assert!(Exponent::ratio(1, 0).is_none());
            assert!(Exponent::from_scalar(f32::INFINITY).is_none());
            assert!(Exponent::from_scalar(f64::NAN).is_none());
        });
    });
}

proptest! {
    #[test]
    fn adding_an_order_and_removing_it_recovers_all_axes(bits: u64) {
        let value = f64::from_bits(bits);
        prop_assume!(value.is_finite());
        let order = Exponent::from_scalar(value).expect("finite binary number");
        prop_assert_eq!(order.to_string(), num_rational::BigRational::from_float(value).expect("finite").to_string());
        let time = RuntimeDimension::of::<dimensions::Time>().power(&order);
        let pressure = RuntimeDimension::of::<dimensions::Pressure>();
        let out = pressure.quotient(&time);
        prop_assert_eq!(out.product(&time), pressure.clone());
        prop_assert_eq!(out == pressure, value == 0.0);
    }
}

#[test]
fn exact_rational_and_native_endpoints() {
    use num_rational::BigRational;
    for x in [
        f32::from_bits(1),
        -f32::MAX,
        0.0,
        -0.0,
        0.5,
        0.5_f32.next_up(),
    ] {
        assert_eq!(
            Exponent::from_scalar(x).expect("finite").to_string(),
            BigRational::from_float(x).expect("finite").to_string()
        );
    }
    for x in [f64::from_bits(1), -f64::MAX, 0.5_f64.next_down()] {
        assert_eq!(
            Exponent::from_scalar(x).expect("finite").to_string(),
            BigRational::from_float(x).expect("finite").to_string()
        );
    }
    let time = RuntimeDimension::of::<dimensions::Time>();
    let third = time.power(&Exponent::ratio(1, 3).expect("three"));
    assert_eq!(third.power(&3.into()), time);
}
