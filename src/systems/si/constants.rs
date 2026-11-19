//! SI defining constants and the dalton conversion uncertainty.
//!
//! The SI Brochure, 9th edition, version 4.01, gives the seven fixed values
//! in section 2.2 and the CODATA 2022 dalton uncertainty in Table 8.

use super::{
    dimensions,
    quantities::{
        Acceleration, Action, ElectricCharge, ElectricalImpedance, Frequency, HeatCapacity,
        LuminousEfficacy, MagneticPermeability, Mass, MolarHeatCapacity, Permittivity, Pressure,
        ReciprocalAmountOfSubstance, StefanBoltzmannCoefficient, Velocity,
    },
    units::StandardAtmosphere,
};
use crate::unit::LinearUnit;

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

/// Pressure defined by one standard atmosphere.
pub const STANDARD_ATMOSPHERIC_PRESSURE: Pressure =
    Pressure::from_base(<StandardAtmosphere as LinearUnit<dimensions::Pressure>>::SCALE);

/// Conventional standard acceleration of gravity.
pub const STANDARD_GRAVITY: Acceleration = Acceleration::from_base(9.806_65);

/// Molar gas constant, the exact SI product of the Avogadro and Boltzmann
/// constants rounded once to binary64.
pub const MOLAR_GAS_CONSTANT: MolarHeatCapacity =
    MolarHeatCapacity::from_base(8.314_462_618_153_24);

/// Reduced Planck constant, Planck's constant divided by 2π.
pub const REDUCED_PLANCK_CONSTANT: Action =
    Action::from_base(*PLANCK_CONSTANT.as_base() / core::f64::consts::TAU);

/// Stefan–Boltzmann constant relating blackbody exitance to absolute
/// temperature raised to the fourth power.
pub const STEFAN_BOLTZMANN_CONSTANT: StefanBoltzmannCoefficient =
    StefanBoltzmannCoefficient::from_base(5.670_374_419_184_43e-8);

/// Vacuum electric permittivity (CODATA 2022 central value).
pub const VACUUM_PERMITTIVITY: Permittivity = Permittivity::from_base(8.854_187_818_8e-12);

/// Vacuum magnetic permeability (CODATA 2022 central value).
pub const VACUUM_PERMEABILITY: MagneticPermeability =
    MagneticPermeability::from_base(1.256_637_061_27e-6);

/// Characteristic impedance of vacuum (CODATA 2022 central value).
pub const VACUUM_IMPEDANCE: ElectricalImpedance = ElectricalImpedance::from_base(376.730_313_412);

/// Electron rest mass (CODATA 2022 central value).
pub const ELECTRON_MASS: Mass = Mass::from_base(9.109_383_713_9e-31);
