use crate::unit::private::Sealed;

/// Kind of a logarithmic amplitude ratio.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmplitudeRatio;

/// Kind of a logarithmic power ratio.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PowerRatio;

/// Unit for a logarithmic ratio of a specified kind.
///
/// A reading multiplied by `FACTOR` gives the natural logarithm of the
/// underlying ratio. This contract operates in logarithmic coordinates.
pub trait LogarithmicUnit: Sealed {
    /// The kind of signal ratio expressed by this unit.
    type Kind;

    /// The unit symbol.
    const SYMBOL: &'static str;

    /// Natural-log coordinate corresponding to a reading of one unit.
    const FACTOR: f64;
}

macro_rules! logarithmic_unit {
    ($(#[$doc:meta])* $name:ident, $kind:ident, $symbol:literal, $factor:expr) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name;

        impl Sealed for $name {}
        impl LogarithmicUnit for $name {
            type Kind = $kind;
            const SYMBOL: &'static str = $symbol;
            const FACTOR: f64 = $factor;
        }
    };
}

logarithmic_unit!(
    /// Neper, the natural logarithm of an amplitude ratio.
    Neper, AmplitudeRatio, "Np", 1.0
);
logarithmic_unit!(
    /// Bel, the base-10 logarithm of a power ratio.
    Bel, PowerRatio, "B", core::f64::consts::LN_10
);
logarithmic_unit!(
    /// Decibel, one tenth of a bel.
    Decibel, PowerRatio, "dB", Bel::FACTOR / 10.0
);
