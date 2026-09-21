//! Shared fixtures: the exactness oracle and the conversion sweep.
//!
//! `sweep()` is the input set the differential oracle runs over, and
//! `assert_exact_in_context` is the oracle itself, so both live once here
//! rather than once per leaf.

/// Assert two `f64` values are bitwise identical.
///
/// The oracle here is exact equality, not a tolerance: the binding performs
/// the same `f64` operation on the same operands that Aequitas performs, so
/// evaluation order provably matches and any difference is a defect rather
/// than rounding: no reordering, no widening, no separate accumulation.
#[track_caller]
pub(super) fn assert_exact(actual: f64, expected: f64) {
    assert_exact_in_context(actual, expected, "values diverge");
}

/// Assert bitwise identity, naming which case diverged.
///
/// Same oracle as [`assert_exact`]; the context distinguishes one expansion of
/// the conversion macro from the next.
#[expect(
    clippy::float_cmp,
    reason = "bitwise equality is the correct oracle for an identical operation"
)]
#[track_caller]
pub(super) fn assert_exact_in_context(actual: f64, expected: f64, context: &str) {
    assert_eq!(actual, expected, "{context}");
}

/// Values a conversion is checked at: a seeded sweep across many decades,
/// plus the witnesses a probe found where dividing by the scale and
/// multiplying by its reciprocal disagree.
///
/// A handful of round numbers is exactly the input set most likely to hide an
/// ulp-level difference, which is how the earlier seven-point version passed
/// while a quarter of the units disagreed with the law crate.
pub(super) fn sweep() -> Vec<f64> {
    let mut values = vec![
        12.5,
        3.0,
        532.0,
        250.0,
        40.0,
        101.325,
        2.5,
        6.318_609_123_747_463e-11,
        -9.823_474_589_799_991e-2,
        1.097_804_469_310_134_3,
        3.037_926_988_917_369_6e-4,
    ];
    // xorshift64: deterministic, dependency-free, and spread over 24 decades.
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..4096 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let mantissa =
            f64::from(u32::try_from(state >> 40).expect("24 bits fit")) / f64::from(1_u32 << 24);
        let exponent = i32::try_from(state % 25).expect("under 25") - 12;
        let sign = if state & 1 == 0 { 1.0 } else { -1.0 };
        values.push(sign * (1.0 + mantissa) * 10_f64.powi(exponent));
    }
    values
}
