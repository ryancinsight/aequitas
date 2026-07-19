use core::ops::Neg;

use crate::quantity::Quantity;

impl<T, D> Neg for Quantity<T, D>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_base(-self.value)
    }
}
