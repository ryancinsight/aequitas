//! Canonical SI dimension aliases.

use typenum::{N1, N2, N3, P1, P2, P3, Z0};

use crate::dimension::{
    AbsoluteTemperatureSemantics, Dimension, SurfaceTensionSemantics,
    TemperatureDifferenceSemantics,
};

/// Dimensionless quantity.
pub type Dimensionless = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Length.
pub type Length = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Reciprocal length.
pub type ReciprocalLength = Dimension<N1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Mass.
pub type Mass = Dimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
/// Time.
pub type Time = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
/// Electric current.
pub type ElectricCurrent = Dimension<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
/// Absolute thermodynamic temperature.
pub type ThermodynamicTemperature =
    Dimension<Z0, Z0, Z0, Z0, P1, Z0, Z0, AbsoluteTemperatureSemantics>;
/// Temperature difference, distinct from absolute thermodynamic temperature.
pub type TemperatureDifference =
    Dimension<Z0, Z0, Z0, Z0, P1, Z0, Z0, TemperatureDifferenceSemantics>;
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
/// Area per mass.
pub type AreaPerMass = Dimension<P2, N1, Z0, Z0, Z0, Z0, Z0>;
/// Volume.
pub type Volume = Dimension<P3, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Velocity.
pub type Velocity = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
/// Frequency.
pub type Frequency = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
/// Reciprocal time.
pub type ReciprocalTime = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
/// Pressure.
pub type Pressure = Dimension<N1, P1, N2, Z0, Z0, Z0, Z0>;
/// Energy.
pub type Energy = Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>;
/// Energy per area.
pub type EnergyPerArea = Dimension<Z0, P1, N2, Z0, Z0, Z0, Z0>;
/// Surface or interfacial tension, force per length.
pub type SurfaceTension = Dimension<Z0, P1, N2, Z0, Z0, Z0, Z0, SurfaceTensionSemantics>;
/// Absorbed dose or specific energy.
pub type AbsorbedDose = Dimension<P2, Z0, N2, Z0, Z0, Z0, Z0>;
/// Energy per amount of substance.
pub type MolarEnergy = Dimension<P2, P1, N2, Z0, Z0, N1, Z0>;
/// Heat capacity per amount of substance.
pub type MolarHeatCapacity = Dimension<P2, P1, N2, Z0, N1, N1, Z0>;
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
/// Mass-density rate, mass per volume per time.
pub type MassDensityRate = Dimension<N3, P1, N1, Z0, Z0, Z0, Z0>;
/// Dynamic viscosity.
pub type DynamicViscosity = Dimension<N1, P1, N1, Z0, Z0, Z0, Z0>;
/// Kinematic viscosity, dimensionally identical to thermal diffusivity.
pub type KinematicViscosity = ThermalDiffusivity;
/// Volumetric flow rate.
pub type VolumetricFlowRate = Dimension<P3, Z0, N1, Z0, Z0, Z0, Z0>;
/// Acoustic impedance.
pub type AcousticImpedance = Dimension<N2, P1, N1, Z0, Z0, Z0, Z0>;
/// Power per area, also called intensity.
pub type Intensity = Dimension<Z0, P1, N3, Z0, Z0, Z0, Z0>;
/// Power per volume.
pub type VolumetricPowerDensity = Dimension<N1, P1, N3, Z0, Z0, Z0, Z0>;
/// Energy per volume.
pub type EnergyPerVolume = Dimension<N1, P1, N2, Z0, Z0, Z0, Z0>;
