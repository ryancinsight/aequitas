//! The tag is derived from the law crate's type parameters.
//!
//! Aequitas keeps no runtime dimension metadata, so every tag here is read
//! back through a `TaggedDimension` impl rather than transcribed beside it. A
//! transcription that drifted would otherwise pass while disagreeing with the
//! law it claims to reproduce.

use aequitas::systems::si::dimensions;

use crate::tag::TaggedDimension;

#[test]
fn base_dimensions_carry_their_own_axis() {
    assert_eq!(
        <dimensions::Length as TaggedDimension>::TAG.exponents(),
        [1, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Mass as TaggedDimension>::TAG.exponents(),
        [0, 1, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Time as TaggedDimension>::TAG.exponents(),
        [0, 0, 1, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::LuminousIntensity as TaggedDimension>::TAG.exponents(),
        [0, 0, 0, 0, 0, 0, 1]
    );
}

#[test]
fn negative_exponents_survive_derivation() {
    // Pressure is M L^-1 T^-2; a sign lost in `typenum`'s `I8` would show here.
    assert_eq!(
        <dimensions::Pressure as TaggedDimension>::TAG.exponents(),
        [-1, 1, -2, 0, 0, 0, 0]
    );
    assert_eq!(
        <dimensions::Capacitance as TaggedDimension>::TAG.exponents(),
        [-2, -1, 4, 2, 0, 0, 0]
    );
}
