use core::{fmt, marker::PhantomData};

use crate::dimension::DivideDimension;

use super::{Unit, UnitDimension, expression::checked_scale, private::Sealed};

/// Quotient of two linear units, with divided scales and subtracted dimensions.
///
/// ```
/// use aequitas::{systems::si::{quantities::Intensity,
///     units::{Watt, Meter, WattPerSquareMeter}}, unit::{Quotient, Product}};
/// let composed = Intensity::from_unit::<Quotient<Watt, Product<Meter, Meter>>>(12.0);
/// assert_eq!(composed, Intensity::from_unit::<WattPerSquareMeter>(12.0));
/// ```
///
/// Incompatible dimensions are rejected:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Length, units::{Watt, Meter}},
///     unit::Quotient};
/// let invalid = Length::from_unit::<Quotient<Watt, Meter>>(12.0_f64);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Quotient<L, R>(PhantomData<(L, R)>);

impl<L, R> Sealed for Quotient<L, R> {}

impl<L, R> UnitDimension for Quotient<L, R>
where
    L: UnitDimension,
    R: UnitDimension,
    L::Dimension: DivideDimension<R::Dimension>,
{
    type Dimension = <L::Dimension as DivideDimension<R::Dimension>>::Output;
}

impl<L, R> Unit<<Self as UnitDimension>::Dimension> for Quotient<L, R>
where
    L: UnitDimension + Unit<L::Dimension>,
    R: UnitDimension + Unit<R::Dimension>,
    L::Dimension: DivideDimension<R::Dimension>,
{
    const SCALE: f64 = checked_scale(L::SCALE / R::SCALE);

    fn fmt_symbol(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("(")?;
        L::fmt_symbol(formatter)?;
        formatter.write_str("/")?;
        R::fmt_symbol(formatter)?;
        formatter.write_str(")")
    }
}
