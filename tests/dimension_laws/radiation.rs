//! Radiation identities: dose from energy per mass, the molar gas constant,
//! and the attenuation chain that closes to an optical depth.

use aequitas::systems::si::{
    quantities::{
        AbsorbedDose, Area, AreaPerMass, Dimensionless, Energy, EnergyPerArea, Length, Mass,
        MassDensity, MolarEnergy, MolarHeatCapacity, ReciprocalLength, ReciprocalTime,
        ThermodynamicTemperature, Time,
    },
    units::{
        Gray, JoulePerMole, JoulePerMoleKelvin, Kelvin, Kilogram, Meter, PerSecond, Second,
        SquareMeter,
    },
};

#[test]
fn biological_response_dimensions_close() {
    let energy = Energy::from_base(12.0_f64);
    let mass = Mass::from_unit::<Kilogram>(3.0_f64);
    let dose: AbsorbedDose = energy / mass;
    assert_eq!(dose.in_unit::<Gray>().to_bits(), 4.0_f64.to_bits());

    let activation = MolarEnergy::from_unit::<JoulePerMole>(8_314.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(1_000.0_f64);
    let gas_constant: MolarHeatCapacity = activation / temperature;
    assert_eq!(
        gas_constant.in_unit::<JoulePerMoleKelvin>().to_bits(),
        8.314_f64.to_bits()
    );

    let rate = ReciprocalTime::from_unit::<PerSecond>(2.0_f64);
    let duration = Time::from_unit::<Second>(0.5_f64);
    let events: Dimensionless = rate * duration;
    assert_eq!(events.into_base().to_bits(), 1.0_f64.to_bits());
}

#[test]
fn photon_interaction_dimensions_close() {
    let area = Area::from_unit::<SquareMeter>(2.0_f64);
    let mass = Mass::from_unit::<Kilogram>(4.0_f64);
    let specific_area: AreaPerMass = area / mass;
    let density = MassDensity::from_base(6.0_f64);
    let attenuation: ReciprocalLength = specific_area * density;
    let path = Length::from_unit::<Meter>(3.0_f64);
    let optical_depth: Dimensionless = attenuation * path;
    let energy = Energy::from_base(8.0_f64);
    let exposure: EnergyPerArea = energy / area;

    assert_eq!(specific_area.into_base().to_bits(), 0.5_f64.to_bits());
    assert_eq!(attenuation.into_base().to_bits(), 3.0_f64.to_bits());
    assert_eq!(optical_depth.into_base().to_bits(), 9.0_f64.to_bits());
    assert_eq!(exposure.into_base().to_bits(), 4.0_f64.to_bits());
}
