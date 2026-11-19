use aequitas::systems::si::{constants::*, quantities::*, units::*};
use aequitas::unit::{Power as UnitPower, Product, Quotient};
use tracing::info_span;
use typenum::{P2, P3};

#[test]
fn defining_constants_solve_for_the_base_units() {
    let _trace = crate::support::trace_phases();
    let (second, meter, kilogram, ampere, kelvin, mole, candela) =
        info_span!("act").in_scope(|| {
            let second: Time = Dimensionless::from_unit::<One>(9_192_631_770.0)
                / CESIUM_HYPERFINE_TRANSITION_FREQUENCY;
            let meter: Length = SPEED_OF_LIGHT * second / 299_792_458.0;
            let kilogram: Mass = PLANCK_CONSTANT * second / (meter * meter) / 6.626_070_15e-34;
            let ampere: ElectricCurrent = ELEMENTARY_CHARGE / second / 1.602_176_634e-19;
            let kelvin =
                (kilogram * meter * meter / (second * second)) / BOLTZMANN_CONSTANT * 1.380_649e-23;
            let mole: AmountOfSubstance =
                Dimensionless::from_base(6.022_140_76e23) / AVOGADRO_CONSTANT;
            let candela: LuminousIntensity =
                LUMINOUS_EFFICACY * (kilogram * meter * meter / (second * second * second)) / 683.0;
            (second, meter, kilogram, ampere, kelvin, mole, candela)
        });
    info_span!("assert").in_scope(|| {
        // All operands used in the substitutions above are identical
        // representable numbers at each cancellation.
        for value in [
            second.in_unit::<Second>(),
            meter.in_unit::<Meter>(),
            kilogram.in_unit::<Kilogram>(),
            ampere.in_unit::<Ampere>(),
            kelvin.into_base(),
            mole.in_unit::<Mole>(),
            candela.in_unit::<Candela>(),
        ] {
            assert_eq!(value.to_bits(), 1.0f64.to_bits());
        }
    });
}

#[test]
fn magnetic_photometric_and_catalytic_units_obey_their_definitions() {
    let _trace = crate::support::trace_phases();
    let x = info_span!("arrange").in_scope(|| 37.25_f64);
    let (weber, henry, lux, concentration) = info_span!("act").in_scope(|| {
        let weber = MagneticFlux::from_unit::<Weber>(x);
        let henry = Inductance::from_unit::<Henry>(x);
        let lux = Illuminance::from_unit::<Lux>(x);
        let concentration = CatalyticActivityConcentration::from_unit::<KatalPerCubicMeter>(x);
        (weber, henry, lux, concentration)
    });
    info_span!("assert").in_scope(|| {
        assert_eq!(
            weber.in_unit::<Product<Volt, Second>>().to_bits(),
            x.to_bits()
        );
        assert_eq!(
            henry.in_unit::<Quotient<Weber, Ampere>>().to_bits(),
            x.to_bits()
        );
        assert_eq!(
            lux.in_unit::<Quotient<Lumen, UnitPower<Meter, P2>>>()
                .to_bits(),
            x.to_bits()
        );
        assert_eq!(
            concentration
                .in_unit::<Quotient<Katal, UnitPower<Meter, P3>>>()
                .to_bits(),
            x.to_bits()
        );
        let flux = LuminousFlux::from_unit::<Lumen>(x);
        let intensity = LuminousIntensity::from_unit::<Candela>(x);
        let solid_angle = SolidAngle::from_unit::<Steradian>(1.0);
        let product: LuminousIntensity = intensity * solid_angle;
        assert_eq!(flux.into_base().to_bits(), product.into_base().to_bits());
    });
}

fn coherent<D, Named, Expression>()
where
    D: aequitas::dimension::MultiplyDimension<aequitas::systems::si::dimensions::Dimensionless>,
    Named: aequitas::unit::LinearUnit<D>,
    Expression: aequitas::unit::Unit<
            <D as aequitas::dimension::MultiplyDimension<
                aequitas::systems::si::dimensions::Dimensionless,
            >>::Output,
        >,
{
    assert_eq!(Named::SCALE.to_bits(), 1.0_f64.to_bits());
    assert_eq!(Named::SCALE.to_bits(), Expression::SCALE.to_bits());
}

#[test]
fn all_special_names_match_their_coherent_expressions() {
    use aequitas::systems::si::dimensions as d;
    use typenum::N1;

    let _trace = crate::support::trace_phases();
    info_span!("assert").in_scope(|| {
        coherent::<d::Angle, Radian, One>();
        coherent::<d::SolidAngle, Steradian, One>();
        coherent::<d::Frequency, Hertz, UnitPower<Second, N1>>();
        coherent::<d::Force, Newton, Product<Kilogram, MeterPerSecondSquared>>();
        coherent::<d::Pressure, Pascal, Quotient<Newton, SquareMeter>>();
        coherent::<d::Energy, Joule, Product<Newton, Meter>>();
        coherent::<d::Power, Watt, Quotient<Joule, Second>>();
        coherent::<d::ElectricCharge, Coulomb, Product<Ampere, Second>>();
        coherent::<d::ElectricPotential, Volt, Quotient<Watt, Ampere>>();
        coherent::<d::Capacitance, Farad, Quotient<Coulomb, Volt>>();
        coherent::<d::ElectricalImpedance, Ohm, Quotient<Volt, Ampere>>();
        coherent::<d::ElectricConductance, Siemens, Quotient<Ampere, Volt>>();
        coherent::<d::MagneticFlux, Weber, Product<Volt, Second>>();
        coherent::<d::MagneticFluxDensity, Tesla, Quotient<Weber, SquareMeter>>();
        coherent::<d::Inductance, Henry, Quotient<Weber, Ampere>>();
        coherent::<d::TemperatureDifference, Celsius, UnitPower<Kelvin, typenum::P1>>();
        coherent::<d::LuminousFlux, Lumen, Product<Candela, Steradian>>();
        coherent::<d::Illuminance, Lux, Quotient<Lumen, SquareMeter>>();
        coherent::<d::Activity, Becquerel, UnitPower<Second, N1>>();
        coherent::<d::AbsorbedDose, Gray, Quotient<Joule, Kilogram>>();
        coherent::<d::DoseEquivalent, Sievert, Quotient<Joule, Kilogram>>();
        coherent::<d::CatalyticActivity, Katal, Quotient<Mole, Second>>();
    });
}

#[test]
fn angular_frequency_and_dose_equivalent_apply_the_quantity_laws() {
    let _trace = crate::support::trace_phases();
    let (cycles, absorbed, quality) = info_span!("arrange").in_scope(|| {
        (
            Frequency::from_unit::<Hertz>(3.25_f64),
            AbsorbedDose::from_unit::<Gray>(0.125_f64),
            Dimensionless::from_unit::<One>(6.0_f64),
        )
    });
    let (phase, equivalent) = info_span!("act").in_scope(|| {
        (
            AngularFrequency::from_frequency(cycles),
            DoseEquivalent::from_absorbed_dose(absorbed, quality),
        )
    });
    info_span!("assert").in_scope(|| {
        assert_eq!(
            phase.in_unit::<RadianPerSecond>().to_bits(),
            (3.25 * core::f64::consts::TAU).to_bits()
        );
        let expected_frequency = (3.25 * core::f64::consts::TAU) / core::f64::consts::TAU;
        assert_eq!(
            phase.frequency().in_unit::<Hertz>().to_bits(),
            expected_frequency.to_bits()
        );
        // Rounded multiplication followed by division has relative error γ₂.
        assert!((expected_frequency - 3.25).abs() <= 3.25 * 2.0 * f64::EPSILON);
        assert_eq!(
            equivalent.in_unit::<Sievert>().to_bits(),
            0.75_f64.to_bits()
        );
    });
}
