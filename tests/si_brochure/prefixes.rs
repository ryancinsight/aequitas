use aequitas::{
    quantity::UnitDisplay,
    systems::si::{quantities::*, units::*},
    unit::{Power, Prefixed, prefixes::*},
};
use tracing::info_span;
use typenum::P3;

#[test]
fn prefix_applies_before_power_and_kelvin_offset() {
    let _trace = crate::support::trace_phases();
    let (volume, absolute, delta, mass, info) = info_span!("act").in_scope(|| {
        (
            Volume::from_unit::<Power<Prefixed<Centi, Meter>, P3>>(2.3_f64),
            ThermodynamicTemperature::from_affine_unit::<Prefixed<Milli, DegreeCelsius>>(
                1000.0_f64,
            ),
            TemperatureDifference::from_unit::<Prefixed<Milli, DegreeCelsius>>(1000.0_f64),
            Mass::from_unit::<Prefixed<Kilo, Gram>>(3.0_f64),
            Information::from_unit::<Prefixed<Quebi, Bit>>(2.0_f64),
        )
    });
    info_span!("assert").in_scope(|| {
        assert!((volume.into_base() - 2.3e-6).abs() <= 2.3e-6 * 3.0 * f64::EPSILON);
        assert_eq!(absolute.in_unit::<Kelvin>().to_bits(), 274.15_f64.to_bits());
        assert_eq!(delta.in_unit::<Kelvin>().to_bits(), 1.0_f64.to_bits());
        assert_eq!(mass.in_unit::<Kilogram>().to_bits(), 3.0_f64.to_bits());
        assert_eq!(
            info.in_unit::<Bit>().to_bits(),
            (2.0_f64.powi(101)).to_bits()
        );
        assert_eq!(
            format!(
                "{}",
                UnitDisplay::new(&delta, Prefixed::<Milli, DegreeCelsius>::default())
            ),
            "1000 m°C"
        );
    });
}

#[test]
fn all_decimal_prefix_metadata_matches_table_seven() {
    use aequitas::unit::LinearUnit;
    let _trace = crate::support::trace_phases();
    macro_rules! check {
        ($($prefix:ident, $symbol:literal, $scale:literal;)+) => {
            $(
                assert_eq!(<Prefixed<$prefix, Meter> as LinearUnit<_>>::SYMBOL, $symbol);
                assert_eq!(<Prefixed<$prefix, Meter> as LinearUnit<_>>::SCALE.to_bits(), f64::to_bits($scale));
            )+
        };
    }
    info_span!("assert").in_scope(|| {
        check! {
            Quetta, "Qm", 1e30; Ronna, "Rm", 1e27; Yotta, "Ym", 1e24; Zetta, "Zm", 1e21;
            Exa, "Em", 1e18; Peta, "Pm", 1e15; Tera, "Tm", 1e12; Giga, "Gm", 1e9;
            Mega, "Mm", 1e6; Kilo, "km", 1e3; Hecto, "hm", 1e2; Deca, "dam", 1e1;
            Deci, "dm", 1e-1; Centi, "cm", 1e-2; Milli, "mm", 1e-3; Micro, "µm", 1e-6;
            Nano, "nm", 1e-9; Pico, "pm", 1e-12; Femto, "fm", 1e-15; Atto, "am", 1e-18;
            Zepto, "zm", 1e-21; Yocto, "ym", 1e-24; Ronto, "rm", 1e-27; Quecto, "qm", 1e-30;
        }
    });
}

#[test]
fn binary_prefix_metadata_matches_section_three() {
    use aequitas::unit::LinearUnit;
    let _trace = crate::support::trace_phases();
    let factors = info_span!("act").in_scope(|| {
        [
            <Prefixed<Kibi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Mebi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Gibi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Tebi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Pebi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Exbi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Zebi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Yobi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Robi, Bit> as LinearUnit<_>>::SCALE,
            <Prefixed<Quebi, Bit> as LinearUnit<_>>::SCALE,
        ]
    });
    info_span!("assert").in_scope(|| {
        for (factor, exponent) in factors.into_iter().zip((10..=100).step_by(10)) {
            assert_eq!(factor.to_bits(), 2.0_f64.powi(exponent).to_bits());
        }
    });
}
