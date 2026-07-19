//! Differential verification against `uom` 0.38.0.

use aequitas::systems::si::{
    quantities::{
        Energy as AequitasEnergy, Length as AequitasLength, Pressure as AequitasPressure,
    },
    units::{Joule, Kilometer, Megapascal},
};
use uom::si::{
    energy::joule,
    f64::{Energy as UomEnergy, Length as UomLength, Pressure as UomPressure},
    length::kilometer,
    pressure::megapascal,
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
