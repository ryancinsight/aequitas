//! SI defining constants and the dalton conversion uncertainty.
//!
//! The SI Brochure, 9th edition, version 4.01, gives the seven fixed values
//! in section 2.2 and the CODATA 2022 dalton uncertainty in Table 8.

use super::quantities::{
    Action, ElectricCharge, Frequency, HeatCapacity, LuminousEfficacy, Mass,
    ReciprocalAmountOfSubstance, Velocity,
};

/// Speed of light in vacuum.
pub const SPEED_OF_LIGHT: Velocity = Velocity::from_base(299_792_458.0);

/// Avogadro constant, the number of specified entities per mole.
pub const AVOGADRO_CONSTANT: ReciprocalAmountOfSubstance =
    ReciprocalAmountOfSubstance::from_base(6.022_140_76e23);

/// Boltzmann constant, thermal energy per kelvin.
pub const BOLTZMANN_CONSTANT: HeatCapacity = HeatCapacity::from_base(1.380_649e-23);

/// Planck constant, action per cycle.
pub const PLANCK_CONSTANT: Action = Action::from_base(6.626_070_15e-34);

/// Elementary charge, the positive magnitude of the electron's charge.
pub const ELEMENTARY_CHARGE: ElectricCharge = ElectricCharge::from_base(1.602_176_634e-19);

/// Caesium-133 unperturbed ground-state hyperfine transition frequency (exact SI).
pub const CESIUM_HYPERFINE_TRANSITION_FREQUENCY: Frequency = Frequency::from_base(9_192_631_770.0);

/// Luminous efficacy of monochromatic radiation at 540 THz (exact SI).
pub const LUMINOUS_EFFICACY: LuminousEfficacy = LuminousEfficacy::from_base(683.0);

/// Standard uncertainty of the Table 8 dalton conversion to kilograms (CODATA 2022).
pub const DALTON_STANDARD_UNCERTAINTY: Mass = Mass::from_base(5.2e-37);
