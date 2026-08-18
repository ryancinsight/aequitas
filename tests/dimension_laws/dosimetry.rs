use aequitas::systems::si::{
    quantities::{
        AbsorbedDose, AbsorbedDoseRate, Mass, MassDensity, Power, SpecificAbsorptionRate,
        TemperatureDifference, ThermodynamicTemperature, Time, VolumetricPowerDensity,
    },
    units::{
        Gray, GrayPerSecond, Kelvin, Kilogram, KilogramPerCubicMeter, Second, Watt,
        WattPerCubicMeter, WattPerKilogram,
    },
};

#[test]
fn absolute_temperature_arithmetic_preserves_affine_semantics() {
    let lower = ThermodynamicTemperature::from_unit::<Kelvin>(290.0_f64);
    let upper = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);
    let delta: TemperatureDifference = upper - lower;

    assert_eq!(delta.in_unit::<Kelvin>().to_bits(), 10.0_f64.to_bits());
    assert_eq!(
        (lower + delta).in_unit::<Kelvin>().to_bits(),
        300.0_f64.to_bits()
    );
    assert_eq!(
        (delta + lower).in_unit::<Kelvin>().to_bits(),
        300.0_f64.to_bits()
    );
    assert_eq!(
        (upper - delta).in_unit::<Kelvin>().to_bits(),
        290.0_f64.to_bits()
    );
}

#[test]
fn power_divided_by_mass_is_specific_absorption_rate() {
    let power = Power::from_unit::<Watt>(12.0_f64);
    let mass = Mass::from_unit::<Kilogram>(3.0_f64);
    let sar: SpecificAbsorptionRate = power / mass;

    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        4.0_f64.to_bits()
    );
}

#[test]
fn absorbed_dose_divided_by_time_is_absorbed_dose_rate() {
    let dose = AbsorbedDose::from_unit::<Gray>(8.0_f64);
    let time = Time::from_unit::<Second>(2.0_f64);
    let rate: AbsorbedDoseRate = dose / time;

    assert_eq!(rate.in_unit::<GrayPerSecond>().to_bits(), 4.0_f64.to_bits());
}

#[test]
fn absorbed_dose_rate_times_time_is_absorbed_dose() {
    let rate = AbsorbedDoseRate::from_unit::<GrayPerSecond>(2.5_f64);
    let time = Time::from_unit::<Second>(4.0_f64);
    let dose: AbsorbedDose = rate * time;

    assert_eq!(dose.in_unit::<Gray>().to_bits(), 10.0_f64.to_bits());
}

#[test]
fn watt_per_kilogram_and_gray_per_second_are_one_coherent_unit() {
    let sar = SpecificAbsorptionRate::from_unit::<WattPerKilogram>(7.5_f64);

    assert_eq!(sar.in_unit::<GrayPerSecond>().to_bits(), 7.5_f64.to_bits());
    assert_eq!(sar.into_base().to_bits(), 7.5_f64.to_bits());
}

#[test]
fn volumetric_power_density_divided_by_mass_density_is_specific_absorption_rate() {
    let deposition = VolumetricPowerDensity::from_unit::<WattPerCubicMeter>(2_000.0_f64);
    let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
    let sar: SpecificAbsorptionRate = deposition / density;

    assert_eq!(
        sar.in_unit::<WattPerKilogram>().to_bits(),
        2.0_f64.to_bits()
    );
}
