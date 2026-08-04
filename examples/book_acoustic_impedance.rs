//! Derive acoustic impedance from mass density and speed of sound.
//!
//! Acoustic impedance Z₀ = ρ × c, where ρ is mass density (kg/m³) and
//! c is velocity (m/s).  The product dimension is kg/(m²·s) = Rayl.
//!
//! This example shows how Aequitas enforces the dimensional algebra at compile
//! time: multiplying a [`MassDensity`] by a [`Velocity`] produces an
//! [`AcousticImpedance`] — any other product is a type error.

use aequitas::systems::si::{
    quantities::{AcousticImpedance, MassDensity, Velocity},
    units::{GramPerCubicCentimeter, KilogramPerCubicMeter, MeterPerSecond, Rayl},
};

/// Characteristic acoustic impedance Z₀ = ρ × c.
fn characteristic_impedance(
    density: MassDensity<f64>,
    speed: Velocity<f64>,
) -> AcousticImpedance<f64> {
    density * speed
}

fn main() {
    // Tissue properties (approximate values from the literature).
    struct Medium {
        name: &'static str,
        density_kg_per_m3: f64,
        speed_m_per_s: f64,
    }

    let media = [
        Medium { name: "water (20 °C)",    density_kg_per_m3: 998.2,  speed_m_per_s: 1482.0 },
        Medium { name: "soft tissue",      density_kg_per_m3: 1050.0, speed_m_per_s: 1540.0 },
        Medium { name: "blood",            density_kg_per_m3: 1060.0, speed_m_per_s: 1570.0 },
        Medium { name: "bone (cortical)",  density_kg_per_m3: 1900.0, speed_m_per_s: 3500.0 },
    ];

    println!("{:<20} {:>12} {:>12} {:>14}", "Medium", "ρ (g/cm³)", "c (m/s)", "Z₀ (MRayl)");
    println!("{}", "-".repeat(62));

    for medium in &media {
        let density  = MassDensity::from_unit::<KilogramPerCubicMeter>(medium.density_kg_per_m3);
        let speed    = Velocity::from_unit::<MeterPerSecond>(medium.speed_m_per_s);
        let z0       = characteristic_impedance(density, speed);
        let z0_mrayl = z0.in_unit::<Rayl>() * 1e-6; // MRayl for display

        println!(
            "{:<20} {:>12.3} {:>12.1} {:>14.3}",
            medium.name,
            density.in_unit::<GramPerCubicCentimeter>(),
            speed.in_unit::<MeterPerSecond>(),
            z0_mrayl,
        );
    }

    // Reflection coefficient at a water–soft-tissue interface.
    let rho_water   = MassDensity::from_unit::<KilogramPerCubicMeter>(998.2);
    let c_water     = Velocity::from_unit::<MeterPerSecond>(1482.0);
    let rho_tissue  = MassDensity::from_unit::<KilogramPerCubicMeter>(1050.0);
    let c_tissue    = Velocity::from_unit::<MeterPerSecond>(1540.0);

    let z_water     = characteristic_impedance(rho_water,  c_water) .in_unit::<Rayl>();
    let z_tissue    = characteristic_impedance(rho_tissue, c_tissue).in_unit::<Rayl>();

    let r = (z_tissue - z_water) / (z_tissue + z_water);
    let intensity_reflection_coefficient = r * r;
    println!(
        "\nwater→tissue normal-incidence intensity reflection: {:.4} ({:.2} %)",
        intensity_reflection_coefficient,
        intensity_reflection_coefficient * 100.0,
    );

    // Sanity check: soft tissue is only a few tenths of a percent reflection.
    assert!(intensity_reflection_coefficient < 0.01);
}
