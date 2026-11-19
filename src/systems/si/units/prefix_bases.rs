//! Eligible roots for decimal and binary prefixes.

use crate::unit::{
    LinearUnit, UnitDimension,
    private::{BinaryPrefixBase, DecimalPrefixBase},
};

use super::{
    Ampere, ArcsecondAs, AtomicMassUnit, Bar, Barn, Becquerel, Bit, Candela, Celsius, Coulomb,
    Dalton, DegreeCelsius, ElectronVolt, Farad, Gal, Gram, Gray, Henry, Hertz, Joule, Katal,
    Kelvin, Liter, LiterLowercase, Lumen, Lux, Meter, Mole, Newton, Ohm, Pascal, Radian, Second,
    Siemens, Sievert, Steradian, Tesla, Tonne, Var, Volt, Watt, Weber,
};

macro_rules! prefix_bases {
    ($base:ident; $($unit:ident;)*) => {
        $(
            impl $base for $unit {
                const SYMBOL: &'static str = <$unit as LinearUnit<<$unit as UnitDimension>::Dimension>>::SYMBOL;
            }
        )*
    };
}

prefix_bases! {
    DecimalPrefixBase;
    Meter;
    Gram;
    Second;
    Ampere;
    Kelvin;
    Mole;
    Candela;
    Radian;
    Steradian;
    Hertz;
    Newton;
    Pascal;
    Joule;
    Watt;
    Coulomb;
    Volt;
    Farad;
    Ohm;
    Siemens;
    Weber;
    Tesla;
    Henry;
    Celsius;
    Lumen;
    Lux;
    Becquerel;
    Gray;
    Sievert;
    Katal;
    ArcsecondAs;
    AtomicMassUnit;
    Barn;
    Bar;
    Bit;
    Dalton;
    ElectronVolt;
    Gal;
    Liter;
    LiterLowercase;
    Tonne;
    Var;
}

prefix_bases! {
    BinaryPrefixBase;
    Bit;
}

impl DecimalPrefixBase for DegreeCelsius {
    const SYMBOL: &'static str = <Celsius as DecimalPrefixBase>::SYMBOL;
}
