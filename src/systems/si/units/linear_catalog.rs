//! Shared declarations for linearly scaled SI unit tokens.

macro_rules! linear_units {
    ($($(#[$doc:meta])* $name:ident, $dimension:ident, $symbol:literal, $scale:expr;)*) => {
        $(
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, Default)]
            pub struct $name;
            impl crate::unit::private::Sealed for $name {}
            impl crate::unit::private::Named for $name {}
            impl crate::unit::UnitDimension for $name {
                type Dimension = crate::systems::si::dimensions::$dimension;
            }
            impl crate::unit::LinearUnit<crate::systems::si::dimensions::$dimension> for $name {
                const SYMBOL: &'static str = $symbol;
                const SCALE: f64 = $scale;
            }
        )*
    };
}
