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

#[test]
fn radioactivity_and_dose_units_have_their_defined_scales() {
    use aequitas::systems::si::{
        quantities::{
            Activity, ActivityConcentration, DoseEquivalent, DoseEquivalentRate, RadiationExposure,
        },
        units::{
            Becquerel, BecquerelPerCubicMeter, BecquerelPerMilliliter, Centigray, Curie,
            Megabecquerel, MicrosievertPerHour, Millicurie, Millisievert, Rem, Roentgen, Sievert,
            SievertPerSecond,
        },
    };
    assert_eq!(
        Activity::from_unit::<Curie>(1.0_f64)
            .in_unit::<Becquerel>()
            .to_bits(),
        37_000_000_000.0_f64.to_bits()
    );
    assert_eq!(
        Activity::from_unit::<Millicurie>(10.0_f64)
            .in_unit::<Megabecquerel>()
            .to_bits(),
        370.0_f64.to_bits()
    );
    assert_eq!(
        ActivityConcentration::from_unit::<BecquerelPerMilliliter>(52.0_f64)
            .in_unit::<BecquerelPerCubicMeter>()
            .to_bits(),
        52_000_000.0_f64.to_bits()
    );
    assert_eq!(
        AbsorbedDose::from_unit::<Centigray>(100.0_f64)
            .in_unit::<Gray>()
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        DoseEquivalent::from_unit::<Rem>(100.0_f64)
            .in_unit::<Sievert>()
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        DoseEquivalent::from_unit::<Millisievert>(1000.0_f64)
            .into_base()
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        DoseEquivalentRate::from_unit::<MicrosievertPerHour>(3600.0_f64)
            .in_unit::<SievertPerSecond>()
            .to_bits(),
        1e-6_f64.to_bits()
    );
    assert_eq!(
        RadiationExposure::from_unit::<Roentgen>(1.0_f64)
            .into_base()
            .to_bits(),
        2.58e-4_f64.to_bits()
    );
}
