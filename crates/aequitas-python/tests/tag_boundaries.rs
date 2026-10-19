//! Exhaustive reference arithmetic uses a wider exact integer in the oracle.

use _pyaequitas::tag::{DimensionTag, SemanticTag};
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

#[test]
fn every_runtime_root_is_exact_or_rejected_without_panicking() {
    let _trace = trace();
    let span = info_span!("arrange").entered();
    let cases = i8::MIN..=i8::MAX;
    drop(span);
    let _span = info_span!("act").entered();
    let _assert = info_span!("assert").entered();
    for axis in 0..7 {
        for exponent in cases.clone() {
            for root in cases.clone() {
                let mut exponents = [0; 7];
                exponents[axis] = exponent;
                let tag = DimensionTag::new(exponents, SemanticTag::Angle);
                let expected = i16::from(exponent)
                    .checked_div(i16::from(root))
                    .filter(|_| i16::from(exponent).checked_rem(i16::from(root)) == Some(0))
                    .and_then(|quotient| i8::try_from(quotient).ok());
                let actual = tag.root(root);
                match (expected, actual) {
                    (Some(quotient), Ok(actual)) => {
                        exponents[axis] = quotient;
                        assert_eq!(actual, DimensionTag::new(exponents, SemanticTag::Base));
                    }
                    (None, Err(error)) => {
                        assert_eq!(error.tag, tag);
                        assert_eq!(error.root, root);
                        assert_eq!(error.axis, if root == 0 { 0 } else { axis });
                    }
                    _ => panic!(
                        "axis {axis}, exponent {exponent}, root {root}: expected {expected:?}, got {actual:?}"
                    ),
                }
            }
        }
    }
}

#[test]
fn signed_byte_endpoints_survive_type_derivation() {
    use _pyaequitas::tag::TaggedDimension;
    use aequitas::dimension::Dimension;
    use typenum::{N128, P127, Z0};

    type Endpoints = Dimension<N128, P127, N128, P127, N128, P127, Z0>;
    let _trace = trace();
    let _arrange = info_span!("arrange").entered();
    let _act = info_span!("act").entered();
    let actual = <Endpoints as TaggedDimension>::TAG.exponents();
    let _assert = info_span!("assert").entered();
    assert_eq!(actual, [-128, 127, -128, 127, -128, 127, 0],);
}

#[test]
fn root_overflow_diagnostic_names_the_range_failure() {
    let _trace = trace();
    let _arrange = info_span!("arrange").entered();
    let tag = DimensionTag::new([0, 0, i8::MIN, 0, 0, 0, 0], SemanticTag::Base);
    let _act = info_span!("act").entered();
    let error = tag
        .root(-1)
        .expect_err("the positive quotient 128 cannot fit");
    let _assert = info_span!("assert").entered();
    assert_eq!(error.axis, 2);
    assert_eq!(
        error.to_string(),
        "cannot take root -1 of `s^-128`: the s exponent quotient 128 is outside the representable range"
    );
}
