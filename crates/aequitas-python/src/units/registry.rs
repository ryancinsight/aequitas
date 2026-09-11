//! Static inventory of quantities and their units.

use crate::tag::DimensionTag;

/// One unit of a quantity, linear or affine.
///
/// `symbol` and `scale` are the Rust associated constants, not copies: the
/// generated inventory expands to `<Unit as LinearUnit<D>>::SYMBOL` and
/// `::SCALE`, so a scale corrected upstream reaches Python by recompilation.
#[derive(Clone, Copy, Debug)]
pub struct Unit {
    /// Rust marker name, e.g. `Millimeter`.
    pub name: &'static str,
    /// Unit abbreviation, e.g. `mm`.
    pub symbol: &'static str,
    /// Multiplicative factor to the canonical SI base unit.
    pub scale: f64,
    /// The base-unit value this unit reads as zero, for an affine unit.
    ///
    /// `None` for a linear unit, and deliberately not `Some(0.0)`: adding a
    /// zero offset is not the identity in IEEE arithmetic, since `-0.0 + 0.0`
    /// is `+0.0`, and the linear conversions keep the sign of zero exactly as
    /// the law crate does. Read from `AffineUnit::OFFSET` for the units the
    /// inventory lists under `affine`.
    pub offset: Option<f64>,
}

impl Unit {
    /// Convert a value in this unit to canonical SI base units.
    ///
    /// One multiplication by `scale`: the operation `LinearUnit::to_base`
    /// performs, on the same operands, so the result is bitwise Aequitas's.
    #[inline]
    #[must_use]
    pub fn to_base(&self, value: f64) -> f64 {
        // `AffineUnit::to_base` adds `T::ONE.scale_by_f64(OFFSET)`, which in
        // `f64` is `1.0 * OFFSET` -- exactly `OFFSET`.
        match self.offset {
            None => value * self.scale,
            Some(offset) => value * self.scale + offset,
        }
    }

    /// Convert a canonical SI base value into this unit.
    ///
    /// Multiplies by the reciprocal rather than dividing, because that is what
    /// `LinearUnit::from_base` does -- `value.scale_by_f64(1.0 / SCALE)`, which
    /// Eunomia implements as a multiplication. Division rounds differently
    /// whenever `1 / scale` is inexact, which is almost every decimal scale: a
    /// seeded sweep of 20,009 values found 24 of the 89 units disagreeing with
    /// the law crate by an ulp, millimetres in an eighth of all values, while
    /// the seven hand-picked points the tests used happened to agree.
    #[inline]
    #[must_use]
    pub fn from_base(&self, base: f64) -> f64 {
        // `AffineUnit::from_base` subtracts the offset first, then scales by
        // the reciprocal, like its linear sibling.
        match self.offset {
            None => base * (1.0 / self.scale),
            Some(offset) => (base - offset) * (1.0 / self.scale),
        }
    }
}

/// One named quantity and the units it admits.
#[derive(Clone, Copy, Debug)]
pub struct Quantity {
    /// Python attribute name, e.g. `mass_density`.
    pub name: &'static str,
    /// Rust alias name, e.g. `MassDensity`.
    pub alias: &'static str,
    /// Runtime image of the quantity's dimension.
    pub tag: DimensionTag,
    /// Units implementing `LinearUnit` for this dimension.
    pub units: &'static [Unit],
}

impl Quantity {
    /// Look up one of this quantity's units by symbol or marker name.
    ///
    /// Resolution is scoped to the quantity because a symbol does not identify
    /// a unit on its own: `Pa` belongs to both `Pressure` and `Stress`, `K` to
    /// both an absolute temperature and a temperature difference, and `J` to
    /// both `Energy` and `FlexuralRigidity`. Those pairs differ only in their
    /// semantic marker, which is exactly what a bare symbol cannot carry.
    #[must_use]
    pub fn unit(&self, needle: &str) -> Option<&'static Unit> {
        self.units
            .iter()
            .find(|unit| unit.symbol == needle || unit.name == needle)
    }

    /// Symbols this quantity accepts, for diagnostics.
    pub fn symbols(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.units.iter().map(|unit| unit.symbol)
    }
}

/// The complete generated inventory.
pub trait Quantities {
    /// Every quantity Aequitas names, in declaration order.
    const ALL: &'static [Quantity];
}

/// Expand the generated inventory into static registry data.
///
/// The macro reads every value from the Rust type system: exponents through
/// [`crate::tag::TaggedDimension`], symbol and scale through `LinearUnit` --
/// or through `AffineUnit`, with its offset, for the units listed under
/// `affine`.
/// Nothing numeric is written in the generated file, so the inventory cannot
/// disagree with the law crate about what a unit means -- only about which
/// units exist, which the generator's `check` mode gates.
#[macro_export]
macro_rules! quantity_inventory {
    ($($py_name:ident => $alias:ident, $dimension:ty { $($unit_name:ident: $unit:ty,)* } $(affine { $($affine_name:ident: $affine_unit:ty,)* })?),+ $(,)?) => {
        /// Generated inventory of every Aequitas quantity.
        pub struct Inventory;

        impl Quantities for Inventory {
            const ALL: &'static [$crate::units::registry::Quantity] = &[
                $(
                    $crate::units::registry::Quantity {
                        name: stringify!($py_name),
                        alias: stringify!($alias),
                        tag: <$dimension as $crate::tag::TaggedDimension>::TAG,
                        units: &[
                            $(
                                $crate::units::registry::Unit {
                                    name: stringify!($unit_name),
                                    symbol: <$unit as ::aequitas::unit::LinearUnit<$dimension>>::SYMBOL,
                                    scale: <$unit as ::aequitas::unit::LinearUnit<$dimension>>::SCALE,
                                    offset: None,
                                },
                            )*
                            $($(
                                $crate::units::registry::Unit {
                                    name: stringify!($affine_name),
                                    symbol: <$affine_unit as ::aequitas::unit::AffineUnit<$dimension>>::SYMBOL,
                                    scale: <$affine_unit as ::aequitas::unit::AffineUnit<$dimension>>::SCALE,
                                    offset: Some(<$affine_unit as ::aequitas::unit::AffineUnit<$dimension>>::OFFSET),
                                },
                            )*)?
                        ],
                    },
                )+
            ];
        }
    };
}
