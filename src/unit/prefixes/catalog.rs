//! Prefix eligibility and scale catalog.

use super::super::private::{BinaryPrefixBase, DecimalPrefixBase, Sealed};

/// Prefix metadata for an eligible unit token.
///
/// The unit parameter encodes eligibility. Decimal and binary prefixes use
/// disjoint eligibility contracts, with bit admitting both.
pub trait Prefix<U>: Sealed {
    /// The prefix's contribution to the unit symbol.
    const SYMBOL: &'static str;

    /// The spelling of the root unit symbol when the prefix is attached.
    const ROOT_SYMBOL: &'static str;

    /// Multiply the unprefixed unit scale by this factor.
    const FACTOR: f64;
}

macro_rules! prefixes {
    ($base:ident; $($name:ident, $symbol:literal, $factor:expr;)*) => {
        $(
            #[doc = concat!(stringify!($name), " prefix (", $symbol, ").")]
            #[derive(Clone, Copy, Debug, Default)]
            pub struct $name;

            impl Sealed for $name {}
            impl<U: $base> Prefix<U> for $name {
                const SYMBOL: &'static str = $symbol;
                const ROOT_SYMBOL: &'static str = U::SYMBOL;
                const FACTOR: f64 = $factor;
            }
        )*
    };
}

prefixes! {
    DecimalPrefixBase;
    Quetta, "Q", 1e30;
    Ronna, "R", 1e27;
    Yotta, "Y", 1e24;
    Zetta, "Z", 1e21;
    Exa, "E", 1e18;
    Peta, "P", 1e15;
    Tera, "T", 1e12;
    Giga, "G", 1e9;
    Mega, "M", 1e6;
    Kilo, "k", 1e3;
    Hecto, "h", 1e2;
    Deca, "da", 1e1;
    Deci, "d", 1e-1;
    Centi, "c", 1e-2;
    Milli, "m", 1e-3;
    Micro, "µ", 1e-6;
    Nano, "n", 1e-9;
    Pico, "p", 1e-12;
    Femto, "f", 1e-15;
    Atto, "a", 1e-18;
    Zepto, "z", 1e-21;
    Yocto, "y", 1e-24;
    Ronto, "r", 1e-27;
    Quecto, "q", 1e-30;
}

// A normal binary64 power of two has a zero fraction and a biased exponent.
prefixes! {
    BinaryPrefixBase;
    Kibi, "Ki", f64::from_bits((1023 + 10) << 52);
    Mebi, "Mi", f64::from_bits((1023 + 20) << 52);
    Gibi, "Gi", f64::from_bits((1023 + 30) << 52);
    Tebi, "Ti", f64::from_bits((1023 + 40) << 52);
    Pebi, "Pi", f64::from_bits((1023 + 50) << 52);
    Exbi, "Ei", f64::from_bits((1023 + 60) << 52);
    Zebi, "Zi", f64::from_bits((1023 + 70) << 52);
    Yobi, "Yi", f64::from_bits((1023 + 80) << 52);
    Robi, "Ri", f64::from_bits((1023 + 90) << 52);
    Quebi, "Qi", f64::from_bits((1023 + 100) << 52);
}
