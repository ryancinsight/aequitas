use core::{fmt, marker::PhantomData};

use typenum::{B1, Integer, IsGreaterOrEqual, IsLessOrEqual, N2147483648, P2147483647};

use crate::dimension::PowDimension;

use super::{Unit, UnitDimension, expression::checked_scale, private::Sealed};

/// Integer power of a linear unit, using a Typenum integer exponent.
/// The exponent must fit a signed 32-bit integer; larger type-level values
/// are rejected rather than truncated during scale computation.
///
/// `Power<Meter, typenum::P2>` is compatible with square metres;
/// `Power<Second, typenum::N1>` is compatible with hertz.
///
/// Normalized exponents do not bypass semantic distinctions:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Stress, units::Pascal}, unit::Power};
/// let invalid = Stress::from_unit::<Power<Pascal, typenum::P1>>(1.0_f64);
/// ```
///
/// Temperature degrees compose as intervals, never affine points:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::ThermodynamicTemperature,
///     units::DegreeCelsius}, unit::Power};
/// let invalid = ThermodynamicTemperature::from_unit::<Power<DegreeCelsius, typenum::P1>>(20.0_f64);
/// ```
///
/// Exponents cannot silently truncate:
///
/// ```compile_fail
/// use aequitas::{Quantity, systems::si::units::Kilometer, unit::Power};
/// let invalid = Quantity::from_unit::<Power<Kilometer, typenum::P4294967296>>(1.0_f64);
/// ```
///
/// Scales must be representable as positive finite metadata:
///
/// ```compile_fail
/// use aequitas::{systems::si::units::Nanometer, unit::{Power, Unit, UnitDimension}};
/// type Tiny = Power<Nanometer, typenum::P40>;
/// const SCALE: f64 = <Tiny as Unit<<Tiny as UnitDimension>::Dimension>>::SCALE;
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Power<U, P>(PhantomData<(U, P)>);

impl<U, P> Sealed for Power<U, P> {}

impl<U, P> UnitDimension for Power<U, P>
where
    U: UnitDimension,
    U::Dimension: PowDimension<P>,
{
    type Dimension = <U::Dimension as PowDimension<P>>::Output;
}

impl<U, P> Unit<<Self as UnitDimension>::Dimension> for Power<U, P>
where
    U: UnitDimension + Unit<U::Dimension>,
    U::Dimension: PowDimension<P>,
    P: Integer
        + IsGreaterOrEqual<N2147483648, Output = B1>
        + IsLessOrEqual<P2147483647, Output = B1>,
{
    const SCALE: f64 = scale_power(U::SCALE, P::I32);

    fn fmt_symbol(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("(")?;
        U::fmt_symbol(formatter)?;
        write!(formatter, "^{})", P::I32)
    }
}

// Exponentiation by squaring takes logarithmic const-evaluation work. Invert
// first for negative powers so a representable reciprocal does not overflow.
const fn scale_power(mut base: f64, exponent: i32) -> f64 {
    let mut remaining = exponent.unsigned_abs();
    if exponent < 0 {
        base = checked_scale(1.0 / base);
    }
    let mut result = 1.0;
    while remaining != 0 {
        if remaining & 1 == 1 {
            result = checked_scale(result * base);
        }
        remaining >>= 1;
        if remaining != 0 {
            base = checked_scale(base * base);
        }
    }
    result
}
