//! Radiation-law values: decay, inverse square, Beer–Lambert, SAR and dose.

use aequitas::systems::si::{
    quantities::{
        AbsorbedDose, Activity, Dimensionless, Intensity, Length, MassDensity, ReciprocalLength,
        Time, VolumetricPowerDensity,
    },
    units::{Becquerel, Gray, KilogramPerCubicMeter, Meter, Second, WattPerSquareMeter},
};

#[test]
fn decayed_activity_matches_closed_form() {
    use aequitas::systems::si::{quantities::ReciprocalTime, units::PerSecond};
    let activity = Activity::from_unit::<Becquerel>(1_000.0_f64);
    let lambda = ReciprocalTime::from_unit::<PerSecond>(0.1_f64);
    let time = Time::from_unit::<Second>(10.0_f64);

    let remaining = activity.decayed(lambda, time);
    let want = 1_000.0 * (-1.0_f64).exp();

    assert!((remaining.into_base() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn inverse_square_matches_closed_form() {
    let intensity = Intensity::from_unit::<WattPerSquareMeter>(100.0_f64);

    let at_two = intensity.inverse_square(Length::from_unit::<Meter>(2.0_f64));

    assert_eq!(at_two.into_base().to_bits(), 25.0_f64.to_bits());
}

#[test]
fn beer_lambert_matches_closed_form() {
    use aequitas::systems::si::units::PerMeter;
    let intensity = Intensity::from_unit::<WattPerSquareMeter>(100.0_f64);
    let attenuation = ReciprocalLength::from_unit::<PerMeter>(0.5_f64);

    let transmitted = intensity.beer_lambert(attenuation, Length::from_unit::<Meter>(2.0_f64));
    let want = 100.0 * (-1.0_f64).exp();

    assert!((transmitted.into_base() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn sar_and_dose_chain_matches_closed_form() {
    use aequitas::systems::si::{
        quantities::SpecificAbsorptionRate,
        units::{GrayPerSecond, Second as SecondUnit, WattPerCubicMeter, WattPerKilogram},
    };
    let deposition = VolumetricPowerDensity::from_unit::<WattPerCubicMeter>(2_000.0_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);

    let sar: SpecificAbsorptionRate = deposition.specific_absorption_rate(density);
    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(sar.in_unit::<GrayPerSecond>().to_bits(), 2.0_f64.to_bits());

    let dose: AbsorbedDose = sar.absorbed_dose(Time::from_unit::<SecondUnit>(60.0_f64));
    assert_eq!(dose.in_unit::<Gray>().to_bits(), 120.0_f64.to_bits());
}

#[test]
fn dose_equivalent_scales_by_quality_factor() {
    use aequitas::systems::si::{quantities::DoseEquivalent, units::Sievert};
    let dose = AbsorbedDose::from_unit::<Gray>(1.0_f64);
    let quality = Dimensionless::from_base(20.0_f64);

    let equivalent = DoseEquivalent::from_absorbed_dose(dose, quality);

    assert_eq!(
        equivalent.in_unit::<Sievert>().to_bits(),
        20.0_f64.to_bits()
    );
}

#[test]
fn optical_depth_is_attenuation_times_path() {
    use aequitas::systems::si::units::PerMeter;
    let attenuation = ReciprocalLength::from_unit::<PerMeter>(0.5_f64);

    let depth: Dimensionless = attenuation.optical_depth(Length::from_unit::<Meter>(4.0_f64));

    assert_eq!(depth.into_base().to_bits(), 2.0_f64.to_bits());
}
