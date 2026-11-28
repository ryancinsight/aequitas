use core::marker::PhantomData;

/// Default semantic marker for dimensions without an affine distinction.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct BaseSemantics;

/// Semantic marker for absolute thermodynamic temperature.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AbsoluteTemperatureSemantics;

/// Semantic marker for a thermodynamic temperature difference.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TemperatureDifferenceSemantics;

/// Semantic marker for interfacial or surface tension.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct SurfaceTensionSemantics;

/// Semantic marker for mechanical spring stiffness.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct SpringStiffnessSemantics;

/// Semantic marker for mass diffusivity.
///
/// Fickian mass diffusion and Fourier heat conduction share the `m²/s` SI
/// exponents; the marker keeps a mass-diffusion coefficient from being
/// substituted for a thermal diffusivity at public boundaries.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MassDiffusivitySemantics;
/// Semantic marker for force-per-velocity mechanical impedance.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MechanicalImpedanceSemantics;

/// Semantic marker for flexural rigidity.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FlexuralRigiditySemantics;

/// Semantic marker for plane and rotational angles measured in radians.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AngleSemantics;

/// Semantic marker for geometric reciprocal-volume coefficients.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReciprocalVolumeSemantics;

/// Semantic marker for mechanical stress.
///
/// Stress shares the pressure dimension but is a distinct physical concept: a
/// second-order tensor component describing internal force per unit area on an
/// oriented surface, where pressure is the isotropic scalar part of that state
/// and an elastic modulus is a material property. Separating them keeps a
/// modulus from being assigned where a stress is meant.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct StressSemantics;

/// Semantic marker for amount-of-substance concentration per volume.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MolarConcentrationSemantics;

/// Seven-axis SI dimension vector with an optional semantic marker.
///
/// The type parameters are type-level integer exponents in this order:
/// length, mass, time, electric current, thermodynamic temperature, amount of
/// substance, and luminous intensity. The semantic marker occupies no storage
/// and is normalized by dimensional multiplication and division.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Dimension<
    Length,
    Mass,
    Time,
    Current,
    Temperature,
    Amount,
    Luminosity,
    Semantics = BaseSemantics,
>(
    #[expect(
        clippy::type_complexity,
        reason = "one zero-sized marker carries all SI axes and semantic identity"
    )]
    PhantomData<(
        Length,
        Mass,
        Time,
        Current,
        Temperature,
        Amount,
        Luminosity,
        Semantics,
    )>,
);

/// Semantic marker for activity.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ActivitySemantics;

/// Semantic marker for activity concentration.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ActivityConcentrationSemantics;

/// Semantic marker for dose equivalent.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DoseEquivalentSemantics;

/// Semantic marker for dose equivalent rate.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DoseEquivalentRateSemantics;

/// Semantic marker for ionizing radiation exposure.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct RadiationExposureSemantics;

/// Semantic marker for angular acceleration in radians per second squared.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AngularAccelerationSemantics;

/// Semantic marker for angular velocity or angular frequency in radians per second.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AngularVelocitySemantics;

/// Semantic marker for an amount of information.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct InformationSemantics;

/// Semantic marker for luminous intensity per projected area.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LuminanceSemantics;

/// Semantic marker for luminous flux in lumens.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LuminousFluxSemantics;

/// Semantic marker for radiant flux per projected area per solid angle.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct RadianceSemantics;

/// Semantic marker for radiant flux per solid angle.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct RadiantIntensitySemantics;

/// Semantic marker for reactive power in var.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReactivePowerSemantics;

/// Semantic marker for solid angle in steradians.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct SolidAngleSemantics;

/// Semantic marker for moment of force.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct TorqueSemantics;
