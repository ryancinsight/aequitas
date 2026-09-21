use core::{fmt, marker::PhantomData};

use crate::dimension::MultiplyDimension;

use super::{Unit, UnitDimension, expression::checked_scale, private::Sealed};

/// Product of two linear units, with multiplied scales and added dimensions.
#[derive(Clone, Copy, Debug, Default)]
pub struct Product<L, R>(PhantomData<(L, R)>);

impl<L, R> Sealed for Product<L, R> {}

impl<L, R> UnitDimension for Product<L, R>
where
    L: UnitDimension,
    R: UnitDimension,
    L::Dimension: MultiplyDimension<R::Dimension>,
{
    type Dimension = <L::Dimension as MultiplyDimension<R::Dimension>>::Output;
}

impl<L, R> Unit<<Self as UnitDimension>::Dimension> for Product<L, R>
where
    L: UnitDimension + Unit<L::Dimension>,
    R: UnitDimension + Unit<R::Dimension>,
    L::Dimension: MultiplyDimension<R::Dimension>,
{
    const SCALE: f64 = checked_scale(L::SCALE * R::SCALE);

    fn fmt_symbol(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("(")?;
        L::fmt_symbol(formatter)?;
        formatter.write_str("*")?;
        R::fmt_symbol(formatter)?;
        formatter.write_str(")")
    }
}
