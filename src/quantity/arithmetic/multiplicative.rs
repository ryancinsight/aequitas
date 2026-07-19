use core::ops::{Div, Mul};

use crate::{
    dimension::{DivideDimension, MultiplyDimension},
    quantity::Quantity,
};

impl<T, LhsDimension, RhsDimension> Mul<Quantity<T, RhsDimension>> for Quantity<T, LhsDimension>
where
    T: Mul<Output = T>,
    LhsDimension: MultiplyDimension<RhsDimension>,
{
    type Output = Quantity<T, <LhsDimension as MultiplyDimension<RhsDimension>>::Output>;

    #[inline]
    fn mul(self, rhs: Quantity<T, RhsDimension>) -> Self::Output {
        Quantity::from_base(self.value * rhs.value)
    }
}

impl<T, LhsDimension, RhsDimension> Div<Quantity<T, RhsDimension>> for Quantity<T, LhsDimension>
where
    T: Div<Output = T>,
    LhsDimension: DivideDimension<RhsDimension>,
{
    type Output = Quantity<T, <LhsDimension as DivideDimension<RhsDimension>>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<T, RhsDimension>) -> Self::Output {
        Quantity::from_base(self.value / rhs.value)
    }
}
