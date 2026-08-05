//! Construct and convert physical quantities through the Aequitas API.
//!
//! This example builds a [`Length`], a [`Pressure`], and a [`Frequency`] from
//! commonly-used non-SI units, computes a derived value, and converts back to
//! a display unit.  Every conversion factor is a compile-time constant; the
//! actual multiplication instructions are the only runtime cost.

use aequitas::systems::si::{
    quantities::{Frequency, Length, Pressure, Time},
    units::{Hertz, Kilopascal, Megahertz, Meter, Microsecond, Millimeter, Pascal, Second},
};

fn main() {
    // Construct from a clinical measurement in mm; stored in metres.
    let aperture = Length::from_unit::<Millimeter>(38.0_f64);
    assert!((aperture.in_unit::<Meter>() - 0.038).abs() < 1e-12);
    println!(
        "aperture: {:.1} mm = {:.4} m",
        aperture.in_unit::<Millimeter>(),
        aperture.in_unit::<Meter>(),
    );

    // Construct ultrasound centre frequency from MHz; stored in Hz.
    let centre_freq = Frequency::from_unit::<Megahertz>(3.5_f64);
    assert!((centre_freq.in_unit::<Hertz>() - 3.5e6).abs() < 1.0);
    println!(
        "centre frequency: {:.2} MHz = {:.3e} Hz",
        centre_freq.in_unit::<Megahertz>(),
        centre_freq.in_unit::<Hertz>(),
    );

    // Acoustic pressure from kPa; stored in Pa.
    let peak_pressure = Pressure::from_unit::<Kilopascal>(250.0_f64);
    assert!((peak_pressure.in_unit::<Pascal>() - 250_000.0).abs() < 1e-6);
    println!(
        "peak pressure: {:.0} kPa = {:.0} Pa",
        peak_pressure.in_unit::<Kilopascal>(),
        peak_pressure.in_unit::<Pascal>(),
    );

    // Wavelength: λ = c / f.  Speed of sound in tissue ≈ 1540 m/s (as a
    // dimensionless ratio here; a typed Velocity would be the production form).
    let speed_of_sound_m_per_s = 1540.0_f64;
    let wavelength_m = speed_of_sound_m_per_s / centre_freq.in_unit::<Hertz>();
    let wavelength = Length::from_unit::<Meter>(wavelength_m);
    println!(
        "wavelength at {:.1} MHz in tissue: {:.3} mm",
        centre_freq.in_unit::<Megahertz>(),
        wavelength.in_unit::<Millimeter>(),
    );

    // The period is the reciprocal of frequency.  Construct from seconds.
    let period = Time::from_unit::<Second>(1.0 / centre_freq.in_unit::<Hertz>());
    println!("period: {:.4e} µs", period.in_unit::<Microsecond>(),);
}
