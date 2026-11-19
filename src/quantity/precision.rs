use eunomia::FloatElement;

use super::Quantity;

impl<D> Quantity<f64, D> {
    /// Materialize a binary64 reference quantity in the destination precision.
    ///
    /// The dimension and SI unit stay fixed. Rounding, overflow, and
    /// underflow follow the destination scalar's `FloatElement::from_f64`.
    /// For example, a standard atmosphere is exact in binary32:
    ///
    /// ```
    /// use aequitas::systems::si::{
    ///     constants::STANDARD_ATMOSPHERIC_PRESSURE, quantities::Pressure,
    /// };
    ///
    /// let pressure: Pressure<f32> = STANDARD_ATMOSPHERIC_PRESSURE.cast();
    /// assert_eq!(pressure.into_base(), 101_325.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn cast<T: FloatElement>(self) -> Quantity<T, D> {
        Quantity::from_base(T::from_f64(self.into_base()))
    }
}
