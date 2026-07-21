//! Differential verification against `uom` 0.38.0.

use aequitas::systems::si::{
    quantities::{
        AbsorbedDose as AequitasAbsorbedDose, AreaPerMass as AequitasAreaPerMass,
        Energy as AequitasEnergy, EnergyPerArea as AequitasEnergyPerArea, Length as AequitasLength,
        MolarEnergy as AequitasMolarEnergy, MolarHeatCapacity as AequitasMolarHeatCapacity,
        Pressure as AequitasPressure, ReciprocalLength as AequitasReciprocalLength,
        ThermalDiffusivity as AequitasThermalDiffusivity,
    },
    units::{
        Gray, Joule, JoulePerMole, JoulePerMoleKelvin, JoulePerSquareMeter, Kilometer, Megapascal,
        PerCentimeter, SquareCentimeterPerGram, SquareMeterPerSecond,
    },
};
use uom::si::{
    available_energy::joule_per_kilogram,
    diffusion_coefficient::square_meter_per_second,
    energy::joule,
    f64::{
        AvailableEnergy as UomAvailableEnergy, DiffusionCoefficient as UomDiffusionCoefficient,
        Energy as UomEnergy, Length as UomLength, MolarEnergy as UomMolarEnergy,
        MolarHeatCapacity as UomMolarHeatCapacity, Pressure as UomPressure,
        RadiantExposure as UomRadiantExposure, ReciprocalLength as UomReciprocalLength,
        SpecificArea as UomSpecificArea,
    },
    length::kilometer,
    molar_energy::joule_per_mole,
    molar_heat_capacity::joule_per_kelvin_mole,
    pressure::megapascal,
    radiant_exposure::joule_per_square_meter,
    reciprocal_length::reciprocal_centimeter,
    specific_area::square_centimeter_per_gram,
};

#[test]
fn linear_conversions_match_uom() {
    let length = 12.5_f64;
    let pressure = 3.25_f64;
    let energy = 7.75_f64;

    let aequitas_length = AequitasLength::from_unit::<Kilometer>(length).into_base();
    let uom_length = UomLength::new::<kilometer>(length).get::<uom::si::length::meter>();
    assert_eq!(aequitas_length.to_bits(), uom_length.to_bits());

    let aequitas_pressure = AequitasPressure::from_unit::<Megapascal>(pressure).into_base();
    let uom_pressure = UomPressure::new::<megapascal>(pressure).get::<uom::si::pressure::pascal>();
    assert_eq!(aequitas_pressure.to_bits(), uom_pressure.to_bits());

    let aequitas_energy = AequitasEnergy::from_unit::<Joule>(energy).into_base();
    let uom_energy = UomEnergy::new::<joule>(energy).get::<joule>();
    assert_eq!(aequitas_energy.to_bits(), uom_energy.to_bits());

    let absorbed_dose = 2.75_f64;
    let aequitas_absorbed_dose = AequitasAbsorbedDose::from_unit::<Gray>(absorbed_dose).into_base();
    let uom_specific_energy =
        UomAvailableEnergy::new::<joule_per_kilogram>(absorbed_dose).get::<joule_per_kilogram>();
    assert_eq!(
        aequitas_absorbed_dose.to_bits(),
        uom_specific_energy.to_bits()
    );

    let molar_energy = 284_000.0_f64;
    let aequitas_molar_energy =
        AequitasMolarEnergy::from_unit::<JoulePerMole>(molar_energy).into_base();
    let uom_molar_energy =
        UomMolarEnergy::new::<joule_per_mole>(molar_energy).get::<joule_per_mole>();
    assert_eq!(aequitas_molar_energy.to_bits(), uom_molar_energy.to_bits());

    let molar_heat_capacity = 8.314_462_618_153_24_f64;
    let aequitas_molar_heat_capacity =
        AequitasMolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(molar_heat_capacity).into_base();
    let uom_molar_heat_capacity =
        UomMolarHeatCapacity::new::<joule_per_kelvin_mole>(molar_heat_capacity)
            .get::<joule_per_kelvin_mole>();
    assert_eq!(
        aequitas_molar_heat_capacity.to_bits(),
        uom_molar_heat_capacity.to_bits()
    );

    let diffusivity = 1.43e-7_f64;
    let aequitas_diffusivity =
        AequitasThermalDiffusivity::from_unit::<SquareMeterPerSecond>(diffusivity).into_base();
    let uom_diffusivity = UomDiffusionCoefficient::new::<square_meter_per_second>(diffusivity)
        .get::<square_meter_per_second>();
    assert_eq!(aequitas_diffusivity.to_bits(), uom_diffusivity.to_bits());

    let reciprocal_length = 0.75_f64;
    let aequitas_reciprocal_length =
        AequitasReciprocalLength::from_unit::<PerCentimeter>(reciprocal_length).into_base();
    let uom_reciprocal_length =
        UomReciprocalLength::new::<reciprocal_centimeter>(reciprocal_length)
            .get::<uom::si::reciprocal_length::reciprocal_meter>();
    assert_eq!(
        aequitas_reciprocal_length.to_bits(),
        uom_reciprocal_length.to_bits()
    );

    let area_per_mass = 1.25_f64;
    let aequitas_area_per_mass =
        AequitasAreaPerMass::from_unit::<SquareCentimeterPerGram>(area_per_mass).into_base();
    let uom_area_per_mass = UomSpecificArea::new::<square_centimeter_per_gram>(area_per_mass)
        .get::<uom::si::specific_area::square_meter_per_kilogram>();
    assert_eq!(
        aequitas_area_per_mass.to_bits(),
        uom_area_per_mass.to_bits()
    );

    let energy_per_area = 5.5_f64;
    let aequitas_energy_per_area =
        AequitasEnergyPerArea::from_unit::<JoulePerSquareMeter>(energy_per_area).into_base();
    let uom_radiant_exposure = UomRadiantExposure::new::<joule_per_square_meter>(energy_per_area)
        .get::<joule_per_square_meter>();
    assert_eq!(
        aequitas_energy_per_area.to_bits(),
        uom_radiant_exposure.to_bits()
    );
}

#[test]
fn dimensional_arithmetic_matches_uom() {
    let pressure = AequitasPressure::from_unit::<Megapascal>(2.0_f64);
    let volume = aequitas::systems::si::quantities::Volume::from_base(0.5_f64);
    let energy: AequitasEnergy = pressure * volume;

    let uom_pressure = UomPressure::new::<megapascal>(2.0_f64);
    let uom_volume = uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(0.5_f64);
    let uom_energy: UomEnergy = uom_pressure * uom_volume;

    assert_eq!(
        energy.into_base().to_bits(),
        uom_energy.get::<joule>().to_bits()
    );
}
