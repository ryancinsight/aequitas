//! Canonical SI dimension aliases.

use typenum::{N1, N2, N3, N4, N7, P1, P2, P3, P4, Z0};

use crate::dimension::{
    AbsoluteTemperatureSemantics, AngleSemantics, Dimension, ReciprocalVolumeSemantics,
    SpringStiffnessSemantics, SurfaceTensionSemantics, TemperatureDifferenceSemantics,
};

/// Dimensionless quantity.
pub type Dimensionless = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Plane or rotational angle, stored in radians.
pub type Angle = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0, AngleSemantics>;
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
/// Electric charge, current multiplied by time.
pub type ElectricCharge = Dimension<Z0, Z0, P1, P1, Z0, Z0, Z0>;
/// Electric potential, energy per electric charge.
pub type ElectricPotential = Dimension<P2, P1, N3, N1, Z0, Z0, Z0>;
/// Electric conductance, electric current per electric potential.
pub type ElectricConductance = Dimension<N2, N1, P3, P2, Z0, Z0, Z0>;
/// Electrical impedance, electric potential per electric current.
pub type ElectricalImpedance = Dimension<P2, P1, N3, N2, Z0, Z0, Z0>;
/// Capacitance, electric charge per electric potential.
pub type Capacitance = Dimension<N2, N1, P4, P2, Z0, Z0, Z0>;
/// Electric polarizability, dipole moment per electric field.
pub type Polarizability = Dimension<Z0, N1, P4, P2, Z0, Z0, Z0>;
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
/// Number density, expressed as entities per volume.
pub type NumberDensity = Dimension<N3, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Reciprocal volume without an entity-count semantic.
pub type ReciprocalVolume = Dimension<N3, Z0, Z0, Z0, Z0, Z0, Z0, ReciprocalVolumeSemantics>;
/// Velocity.
pub type Velocity = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
/// Frequency.
pub type Frequency = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
/// Reciprocal time.
pub type ReciprocalTime = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
/// Pressure.
pub type Pressure = Dimension<N1, P1, N2, Z0, Z0, Z0, Z0>;
/// Force.
pub type Force = Dimension<P1, P1, N2, Z0, Z0, Z0, Z0>;
/// Pressure per electric current, used for pressure transducer gain.
pub type PressurePerElectricCurrent = Dimension<N1, P1, N2, N1, Z0, Z0, Z0>;
/// Pressure per electric potential, used for voltage-driven transducer gain.
pub type PressurePerElectricPotential = Dimension<N3, Z0, P1, P1, Z0, Z0, Z0>;
/// Electric potential per pressure, the reciprocal receive sensitivity.
pub type ElectricPotentialPerPressure = Dimension<P3, Z0, N1, N1, Z0, Z0, Z0>;
/// Length per electric potential, used for piezoelectric deflection.
pub type LengthPerElectricPotential = Dimension<N1, N1, P3, P1, Z0, Z0, Z0>;
/// Surface electric-charge density.
pub type SurfaceChargeDensity = Dimension<N2, Z0, P1, P1, Z0, Z0, Z0>;
/// Mechanical spring stiffness, force per length.
pub type SpringStiffness = Dimension<Z0, P1, N2, Z0, Z0, Z0, Z0, SpringStiffnessSemantics>;
/// Mechanical damping coefficient, force times time per length.
pub type DampingCoefficient = Dimension<Z0, P1, N1, Z0, Z0, Z0, Z0>;
/// Pressure gradient, pressure per length.
pub type PressureGradient = Dimension<N2, P1, N2, Z0, Z0, Z0, Z0>;
/// Hydraulic resistance, pressure per volumetric flow rate.
pub type HydraulicResistance = Dimension<N4, P1, N1, Z0, Z0, Z0, Z0>;
/// Hydraulic inertance, pressure per flow-rate derivative.
pub type HydraulicInertance = Dimension<N4, P1, Z0, Z0, Z0, Z0, Z0>;
/// Hydraulic compliance, volume per pressure.
pub type Compliance = Dimension<P4, N1, P2, Z0, Z0, Z0, Z0>;
/// Quadratic hydraulic resistance, pressure per squared volumetric flow rate.
pub type QuadraticHydraulicResistance = Dimension<N7, P1, Z0, Z0, Z0, Z0, Z0>;
/// Hydraulic conductance, volumetric flow rate per pressure.
pub type HydraulicConductance = Dimension<P4, N1, P1, Z0, Z0, Z0, Z0>;
/// Energy.
pub type Energy = Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>;
/// Energy per area.
pub type EnergyPerArea = Dimension<Z0, P1, N2, Z0, Z0, Z0, Z0>;
/// Surface or interfacial tension, force per length.
pub type SurfaceTension = Dimension<Z0, P1, N2, Z0, Z0, Z0, Z0, SurfaceTensionSemantics>;
/// Absorbed dose or specific energy.
pub type AbsorbedDose = Dimension<P2, Z0, N2, Z0, Z0, Z0, Z0>;
/// Absorbed dose rate, equivalently power per mass.
pub type AbsorbedDoseRate = Dimension<P2, Z0, N3, Z0, Z0, Z0, Z0>;
/// Specific absorption rate, the radiofrequency-dosimetry name for absorbed
/// dose rate. `W/kg` and `Gy/s` are the same coherent SI dimension, so this is
/// an alias rather than a distinct axis.
pub type SpecificAbsorptionRate = AbsorbedDoseRate;
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
pub type ThermalDiffusivity = AreaPerTime;
/// Area per time, including planar flow rate per unit width.
pub type AreaPerTime = Dimension<P2, Z0, N1, Z0, Z0, Z0, Z0>;
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
