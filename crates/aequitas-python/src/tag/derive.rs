//! Derivation of a runtime tag from a type-level Aequitas dimension.

use aequitas::dimension::Dimension;
use typenum::Integer;

use super::model::DimensionTag;
use super::semantics::TaggedSemantics;

/// A type-level dimension with a runtime image.
///
/// The blanket impl below covers every `Dimension` Aequitas can express, so
/// the tag is read off the type parameters rather than transcribed. A
/// dimension added upstream needs no change here: its exponents arrive through
/// `typenum`'s associated constants and its marker through
/// [`TaggedSemantics`].
pub trait TaggedDimension {
    /// Runtime image of this dimension.
    const TAG: DimensionTag;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Luminosity, Semantics> TaggedDimension
    for Dimension<Length, Mass, Time, Current, Temperature, Amount, Luminosity, Semantics>
where
    Length: Integer,
    Mass: Integer,
    Time: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
    Semantics: TaggedSemantics,
{
    const TAG: DimensionTag = DimensionTag::new(
        [
            Length::I8,
            Mass::I8,
            Time::I8,
            Current::I8,
            Temperature::I8,
            Amount::I8,
            Luminosity::I8,
        ],
        Semantics::TAG,
    );
}
