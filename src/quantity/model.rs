use core::{fmt, marker::PhantomData};

/// Physical quantity with scalar value `T` and type-level dimension `D`.
///
/// Values are stored in the canonical SI base unit for `D`. `D` is carried by
/// [`PhantomData`] and occupies no storage, so this type has the size and
/// alignment of `T`.
#[repr(transparent)]
pub struct Quantity<T, D> {
    pub(super) value: T,
    pub(super) dimension: PhantomData<D>,
}

impl<T, D> Quantity<T, D> {
    /// Construct a quantity from a value already expressed in its canonical SI
    /// base unit.
    #[inline]
    #[must_use]
    pub const fn from_base(value: T) -> Self {
        Self {
            value,
            dimension: PhantomData,
        }
    }

    /// Borrow the value in its canonical SI base unit without conversion.
    #[inline]
    #[must_use]
    pub const fn as_base(&self) -> &T {
        &self.value
    }

    /// Move the value out in its canonical SI base unit.
    #[inline]
    #[must_use]
    pub fn into_base(self) -> T {
        self.value
    }
}

// Manual structural implementations avoid imposing irrelevant trait bounds on
// the zero-sized dimension parameter.
impl<T: Copy, D> Copy for Quantity<T, D> {}

impl<T: Clone, D> Clone for Quantity<T, D> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_base(self.value.clone())
    }
}

impl<T: Default, D> Default for Quantity<T, D> {
    #[inline]
    fn default() -> Self {
        Self::from_base(T::default())
    }
}

impl<T: fmt::Debug, D> fmt::Debug for Quantity<T, D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Quantity")
            .field(&self.value)
            .finish()
    }
}

impl<T: PartialEq, D> PartialEq for Quantity<T, D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, D> Eq for Quantity<T, D> {}

impl<T: PartialOrd, D> PartialOrd for Quantity<T, D> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord, D> Ord for Quantity<T, D> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
