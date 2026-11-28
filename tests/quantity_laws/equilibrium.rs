//! Equilibrium-law values: K from Gibbs energy, van 't Hoff
//! extrapolation, reaction quotients and the Gibbs driving force.

use aequitas::systems::si::{
    quantities::{Dimensionless, MolarEnergy, ThermodynamicTemperature},
    units::{JoulePerMole, Kelvin},
};

#[test]
fn equilibrium_constant_matches_closed_form() {
    let gibbs = MolarEnergy::from_unit::<JoulePerMole>(-10_000.0_f64);
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(298.15_f64);

    let k: Dimensionless = gibbs.equilibrium_constant(temperature);
    let want = (10_000.0_f64 / (8.314_462_618_153_24_f64 * 298.15_f64)).exp();

    assert!((k.into_base() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn vant_hoff_extrapolation_recovers_known_constant() {
    // Extrapolating from T to T is the identity; the general form is
    // pinned by the closed form below.
    let enthalpy = MolarEnergy::from_unit::<JoulePerMole>(50_000.0_f64);
    let known = Dimensionless::from_base(2.5_f64);
    let t = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);

    let same: Dimensionless = enthalpy.vant_hoff_extrapolate(known, t, t);
    assert_eq!(same.into_base().to_bits(), 2.5_f64.to_bits());

    let t2 = ThermodynamicTemperature::from_unit::<Kelvin>(310.0_f64);
    let moved: Dimensionless = enthalpy.vant_hoff_extrapolate(known, t, t2);
    let want = 2.5_f64
        * (-50_000.0_f64 / 8.314_462_618_153_24_f64 * (1.0_f64 / 310.0_f64 - 1.0_f64 / 300.0_f64))
            .exp();
    assert!(moved.into_base() > known.into_base());
    assert!((moved.into_base() - want).abs() <= 1e-9 * want.abs());
}

#[test]
fn stoichiometric_powers_multiply_to_quotient() {
    let a_product = Dimensionless::from_base(2.0_f64);
    let a_reactant = Dimensionless::from_base(4.0_f64);

    let q = a_product.stoichiometric_power(2.0_f64) * a_reactant.stoichiometric_power(-1.0_f64);

    assert_eq!(q.into_base().to_bits(), 1.0_f64.to_bits());
}

#[test]
fn gibbs_driving_force_changes_sign_at_equilibrium() {
    let temperature = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);

    let lean = Dimensionless::from_base(0.5_f64);
    let rich = Dimensionless::from_base(2.0_f64);
    let unity = Dimensionless::from_base(1.0_f64);

    let g_lean = lean.gibbs_driving_force(temperature);
    let g_rich = rich.gibbs_driving_force(temperature);
    let g_eq = unity.gibbs_driving_force(temperature);

    assert!(g_lean.into_base() > 0.0);
    assert!(g_rich.into_base() < 0.0);
    assert_eq!(g_eq.into_base().to_bits(), 0.0_f64.to_bits());
    let want = -8.314_462_618_153_24_f64 * 300.0_f64 * 0.5_f64.ln();
    assert!((g_lean.into_base() - want).abs() <= 1e-9 * want.abs());
}
