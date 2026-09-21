//! Unit-aware display for physical quantities.
//!
//! [`UnitDisplay`] formats a quantity's value in a chosen linear unit together
//! with that unit's symbol or compound expression, e.g. `"5.2 m/s"`.
//! The value is materialized via [`Quantity::in_unit`] so the printed number is
//! expressed in the requested unit rather than the canonical SI base unit.
//!
//! ```
//! use aequitas::quantity::UnitDisplay;
//! use aequitas::systems::si::{quantities::Velocity, units::MeterPerSecond};
//! let v = Velocity::from_base(2.5);
//! assert_eq!(format!("{}", UnitDisplay::new(&v, MeterPerSecond)), "2.5 m/s");
//! ```

use core::fmt;

use eunomia::UnitScalar;

use crate::unit::Unit;

use super::Quantity;

/// Formats a quantity's value in unit `U` with that unit's symbol.
///
/// The wrapper borrows the quantity and carries the unit type without storing
/// a marker value.
pub struct UnitDisplay<'a, T, D, U> {
    quantity: &'a Quantity<T, D>,
    unit: core::marker::PhantomData<U>,
}

impl<'a, T, D, U> UnitDisplay<'a, T, D, U> {
    /// Create a display wrapper for `quantity` expressed in unit `U`.
    #[inline]
    #[must_use]
    pub fn new(quantity: &'a Quantity<T, D>, _unit: U) -> Self {
        Self {
            quantity,
            unit: core::marker::PhantomData,
        }
    }
}

impl<T, D, U> fmt::Display for UnitDisplay<'_, T, D, U>
where
    T: UnitScalar + fmt::Display,
    U: Unit<D>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.quantity.in_unit::<U>();
        write!(formatter, "{value} ")?;
        U::fmt_symbol(formatter)
    }
}

impl<T, D, U> fmt::Debug for UnitDisplay<'_, T, D, U>
where
    T: UnitScalar + fmt::Display,
    U: Unit<D>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}
