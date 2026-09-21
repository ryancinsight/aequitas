//! Shared fixtures: the exactness oracle and two base-unit quantities.
//!
//! `assert_exact` is the oracle every leaf asserts through, so it lives
//! once here rather than once per leaf.

use aequitas::systems::si::dimensions;

use crate::quantity::PyQuantity;
use crate::tag::TaggedDimension;

/// Assert two `f64` values are bitwise identical.
///
/// The oracle here is exact equality, not a tolerance: the binding performs
/// the same `f64` operation on the same operands that Aequitas performs, so
/// evaluation order provably matches and any difference is a defect rather
/// than rounding: no reordering, no widening, no separate accumulation.
#[expect(
    clippy::float_cmp,
    reason = "bitwise equality is the correct oracle for an identical operation"
)]
#[track_caller]
pub(super) fn assert_exact(actual: f64, expected: f64) {
    assert_eq!(actual, expected);
}

pub(super) fn length(meters: f64) -> PyQuantity {
    PyQuantity::from_base(meters, <dimensions::Length as TaggedDimension>::TAG)
}

pub(super) fn time(seconds: f64) -> PyQuantity {
    PyQuantity::from_base(seconds, <dimensions::Time as TaggedDimension>::TAG)
}
