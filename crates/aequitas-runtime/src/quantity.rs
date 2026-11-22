use crate::{RuntimeDimension, StaticDimension};
use aequitas::Quantity;
use core::ops::Deref;

/// A scalar or homogeneous buffer in canonical SI units, with one exact
/// dimension shared by all its values.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeQuantity<V> {
    value: V,
    dimension: RuntimeDimension,
}

impl<V> RuntimeQuantity<V> {
    /// Attach a runtime dimension to data expressed in its coherent SI units.
    pub const fn from_base(value: V, dimension: RuntimeDimension) -> Self {
        Self { value, dimension }
    }

    /// Physical dimension of the value or every element in the buffer.
    pub const fn dimension(&self) -> &RuntimeDimension {
        &self.dimension
    }

    /// Borrow the coherent SI value.
    pub const fn as_base(&self) -> &V {
        &self.value
    }

    /// Move the coherent SI value out of the quantity.
    pub fn into_base(self) -> V {
        self.value
    }

    /// Borrow the underlying slice or other dereference target with its units.
    pub fn as_deref(&self) -> RuntimeQuantity<&V::Target>
    where
        V: Deref,
    {
        RuntimeQuantity::from_base(&self.value, self.dimension.clone())
    }
}

impl<V, D: StaticDimension> From<Quantity<V, D>> for RuntimeQuantity<V> {
    fn from(value: Quantity<V, D>) -> Self {
        Self::from_base(value.into_base(), RuntimeDimension::of::<D>())
    }
}
