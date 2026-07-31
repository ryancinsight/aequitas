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

/// Semantic marker for plane and rotational angles measured in radians.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AngleSemantics;

/// Semantic marker for geometric reciprocal-volume coefficients.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReciprocalVolumeSemantics;

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
