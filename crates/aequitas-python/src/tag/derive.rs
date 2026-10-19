//! Derivation of a runtime tag from a type-level Aequitas dimension.

use aequitas::dimension::Dimension;
use typenum::{B1, Integer, IsGreaterOrEqual, IsLessOrEqual, N128, P127};

use super::model::DimensionTag;
use super::semantics::TaggedSemantics;

/// A type-level dimension with a runtime image.
///
/// The blanket impl covers every `Dimension` whose exponents fit `i8`, so
/// the tag is read off the type parameters rather than transcribed. A
/// dimension added upstream needs no change here: its exponents arrive through
/// `typenum`'s associated constants and its marker through
/// [`TaggedSemantics`].
///
/// Exponents outside the wire format's signed-byte range are rejected:
///
/// ```compile_fail,E0277
/// use aequitas::dimension::Dimension;
/// use _pyaequitas::tag::TaggedDimension;
/// use typenum::{P128, Z0};
/// let _ = <Dimension<P128, Z0, Z0, Z0, Z0, Z0, Z0> as TaggedDimension>::TAG;
/// ```
///
/// ```compile_fail,E0277
/// use aequitas::dimension::Dimension;
/// use _pyaequitas::tag::TaggedDimension;
/// use typenum::{N257, Z0};
/// let _ = <Dimension<Z0, Z0, Z0, Z0, Z0, Z0, N257> as TaggedDimension>::TAG;
/// ```
pub trait TaggedDimension {
    /// Runtime image of this dimension.
    const TAG: DimensionTag;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Luminosity, Semantics> TaggedDimension
    for Dimension<Length, Mass, Time, Current, Temperature, Amount, Luminosity, Semantics>
where
    Length: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Mass: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Time: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Current: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Temperature: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Amount: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
    Luminosity: Integer + IsGreaterOrEqual<N128, Output = B1> + IsLessOrEqual<P127, Output = B1>,
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
