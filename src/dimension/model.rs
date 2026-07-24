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
