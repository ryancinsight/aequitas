//! Release-codegen fixture for transparent quantity arithmetic.

use aequitas::systems::si::quantities::{Length, Time, Velocity};

#[inline(never)]
/// Divide two raw scalar values.
#[must_use]
pub extern "C" fn raw_velocity(distance: f64, duration: f64) -> f64 {
    distance / duration
}

#[inline(never)]
/// Divide typed length and time quantities and return the canonical scalar.
#[must_use]
pub extern "C" fn typed_velocity(distance: f64, duration: f64) -> f64 {
    let distance = Length::from_base(distance);
    let duration = Time::from_base(duration);
    let velocity: Velocity = distance / duration;
    velocity.into_base()
}

#[test]
fn typed_and_raw_velocity_have_identical_value_semantics() {
    let distance = std::hint::black_box(12.0);
    let duration = std::hint::black_box(3.0);
    assert_eq!(
        std::hint::black_box(typed_velocity(distance, duration)).to_bits(),
        std::hint::black_box(raw_velocity(distance, duration)).to_bits()
    );
}
