use aequitas::systems::si::{levels::*, quantities::*, units::*};
use tracing::info_span;

#[test]
fn table_eight_conversions_match_reference_values() {
    let _trace = crate::support::trace_phases();
    let values = info_span!("act").in_scope(|| {
        [
            (
                Time::from_unit::<Minute>(2.0_f64).in_unit::<Second>(),
                120.0,
            ),
            (Time::from_unit::<Hour>(2.0).in_unit::<Minute>(), 120.0),
            (Time::from_unit::<Day>(2.0).in_unit::<Hour>(), 48.0),
            (
                Angle::from_unit::<Degree>(2.0).in_unit::<Arcminute>(),
                120.0,
            ),
            (
                Angle::from_unit::<Arcminute>(2.0).in_unit::<Arcsecond>(),
                120.0,
            ),
            (Area::from_unit::<Are>(2.0).in_unit::<SquareMeter>(), 200.0),
            (Area::from_unit::<Hectare>(2.0).in_unit::<Are>(), 200.0),
            (Area::from_unit::<Barn>(2.0).in_unit::<SquareMeter>(), 2e-28),
            (
                Volume::from_unit::<Liter>(2.0).in_unit::<CubicMeter>(),
                0.002,
            ),
            (Mass::from_unit::<Tonne>(2.0).in_unit::<Kilogram>(), 2000.0),
            (Length::from_unit::<Angstrom>(2.0).in_unit::<Meter>(), 2e-10),
            (
                Acceleration::from_unit::<Gal>(2.0).in_unit::<MeterPerSecondSquared>(),
                0.02,
            ),
            (
                Pressure::from_unit::<Bar>(2.0).in_unit::<Pascal>(),
                200_000.0,
            ),
            (
                Mass::from_unit::<Dalton>(2.0).in_unit::<Kilogram>(),
                3.321_078_137_84e-27,
            ),
            (
                Length::from_unit::<AstronomicalUnit>(2.0).in_unit::<Meter>(),
                299_195_741_400.0,
            ),
            (
                Length::from_unit::<NauticalMile>(2.0).in_unit::<Meter>(),
                3704.0,
            ),
            (
                Velocity::from_unit::<Knot>(2.0).in_unit::<MeterPerSecond>(),
                3704.0 / 3600.0,
            ),
            (
                Energy::from_unit::<ElectronVolt>(2.0).in_unit::<Joule>(),
                3.204_353_268e-19,
            ),
            (
                ReactivePower::from_unit::<Var>(2.0).in_unit::<VoltAmpere>(),
                2.0,
            ),
        ]
    });
    info_span!("assert").in_scope(|| {
        for (actual, reference) in values {
            // Each conversion uses at most two correctly rounded coefficients
            // and one operation in each direction (combined relative error γ₄).
            assert!((actual - reference).abs() <= reference * (4.0 * f64::EPSILON));
        }
    });
}

#[test]
fn logarithmic_units_require_the_signal_kind_and_reference() {
    let _trace = crate::support::trace_phases();
    let (signal, reference) = info_span!("arrange").in_scope(|| {
        (
            ElectricPotential::from_unit::<Volt>(1.2),
            ElectricPotential::from_unit::<Volt>(0.12),
        )
    });
    let (amplitude, power) = info_span!("act").in_scope(|| {
        let amplitude = AmplitudeLevel::from_ratio(signal, reference).expect("positive amplitudes");
        let power = amplitude.square_law_power();
        (amplitude, power)
    });
    info_span!("assert").in_scope(|| {
        assert!((amplitude.in_unit::<Neper>() - 10.0_f64.ln()).abs() <= 2.0 * f64::EPSILON);
        assert!((power.in_unit::<Bel>() - 2.0).abs() <= 2.0 * f64::EPSILON);
        assert!((power.in_unit::<Decibel>() - 20.0).abs() <= 20.0 * f64::EPSILON);
        assert!(AmplitudeLevel::from_ratio(reference, ElectricPotential::from_base(0.0)).is_none());
    });
}

#[test]
fn extreme_ratios_and_adjacent_values_keep_their_logarithmic_levels() {
    let _trace = crate::support::trace_phases();
    let (smallest, largest, adjacent) = info_span!("arrange").in_scope(|| {
        (
            Length::from_base(f64::from_bits(1)),
            Length::from_base(f64::MAX),
            Length::from_base(1.0_f32.next_up()),
        )
    });
    let (wide, one_ulp) = info_span!("act").in_scope(|| {
        (
            AmplitudeLevel::from_ratio(largest, smallest).expect("both operands are finite"),
            AmplitudeLevel::from_ratio(adjacent, Length::from_base(1.0_f32))
                .expect("both operands are positive"),
        )
    });
    info_span!("assert").in_scope(|| {
        // MAX / least subnormal is (2 - 2^-52) × 2^2097, so the log
        // differs from 2098 ln 2 by less than one output ULP.
        let reference = 2098.0 * core::f64::consts::LN_2;
        assert!(
            (wide.in_unit::<Neper>() - reference).abs() <= 2.0 * (reference.next_up() - reference)
        );
        assert_eq!(
            one_ulp.in_unit::<Neper>().to_bits(),
            f32::EPSILON.ln_1p().to_bits()
        );
        let reciprocal = AmplitudeLevel::from_ratio(smallest, largest).expect("positive ratio");
        assert_eq!(
            reciprocal.in_unit::<Neper>().to_bits(),
            (-wide.in_unit::<Neper>()).to_bits()
        );
        for invalid in [0.0, -0.0, -1.0, f64::INFINITY, f64::NAN] {
            assert!(AmplitudeLevel::from_ratio(Length::from_base(invalid), largest).is_none());
            assert!(PowerLevel::from_ratio(largest, Length::from_base(invalid)).is_none());
        }
    });
}
