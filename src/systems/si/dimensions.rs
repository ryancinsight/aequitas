//! Canonical SI dimension aliases.

use typenum::{N1, N2, N3, P1, P2, P3, Z0};

use crate::dimension::Dimension;

/// Dimensionless quantity.
pub type Dimensionless = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Length.
pub type Length = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Mass.
pub type Mass = Dimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
/// Time.
pub type Time = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
/// Electric current.
pub type ElectricCurrent = Dimension<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
/// Thermodynamic temperature.
pub type ThermodynamicTemperature = Dimension<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
/// Reciprocal thermodynamic temperature.
pub type ReciprocalTemperature = Dimension<Z0, Z0, Z0, Z0, N1, Z0, Z0>;
/// Reciprocal squared thermodynamic temperature.
pub type ReciprocalTemperatureSquared = Dimension<Z0, Z0, Z0, Z0, N2, Z0, Z0>;
/// Amount of substance.
pub type AmountOfSubstance = Dimension<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
/// Luminous intensity.
pub type LuminousIntensity = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
/// Area.
pub type Area = Dimension<P2, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Volume.
pub type Volume = Dimension<P3, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Velocity.
pub type Velocity = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
/// Frequency.
pub type Frequency = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
/// Pressure.
pub type Pressure = Dimension<N1, P1, N2, Z0, Z0, Z0, Z0>;
/// Energy.
pub type Energy = Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>;
/// Power.
pub type Power = Dimension<P2, P1, N3, Z0, Z0, Z0, Z0>;
/// Heat capacity.
pub type HeatCapacity = Dimension<P2, P1, N2, Z0, N1, Z0, Z0>;
/// Specific heat capacity.
pub type SpecificHeatCapacity = Dimension<P2, Z0, N2, Z0, N1, Z0, Z0>;
/// Thermal conductivity.
pub type ThermalConductivity = Dimension<P1, P1, N3, Z0, N1, Z0, Z0>;
/// Thermal diffusivity.
pub type ThermalDiffusivity = Dimension<P2, Z0, N1, Z0, Z0, Z0, Z0>;
/// Mass density.
pub type MassDensity = Dimension<N3, P1, Z0, Z0, Z0, Z0, Z0>;
