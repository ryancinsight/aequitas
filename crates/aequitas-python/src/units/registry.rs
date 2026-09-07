//! Static inventory of quantities and their linear units.

use crate::tag::DimensionTag;

/// One linear unit of a quantity.
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
/// [`crate::tag::TaggedDimension`], symbol and scale through `LinearUnit`.
/// Nothing numeric is written in the generated file, so the inventory cannot
/// disagree with the law crate about what a unit means -- only about which
/// units exist, which the generator's `check` mode gates.
#[macro_export]
macro_rules! quantity_inventory {
    ($($py_name:ident => $alias:ident, $dimension:ty { $($unit_name:ident: $unit:ty,)* }),+ $(,)?) => {
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
                                },
                            )*
                        ],
                    },
                )+
            ];
        }
    };
}
