//! Zero-sized SI and non-SI unit markers.
//!
//! [`Celsius`], [`DegreeCelsius`] and [`DegreeFahrenheit`] use affine
//! conversions for absolute temperatures and linear conversions for intervals.

#[macro_use]
mod linear_catalog;

mod base;
mod brochure;
mod derived;
mod non_si;
mod prefix_bases;
mod scaled;
mod temperature;

pub use base::{Ampere, Candela, Kelvin, Kilogram, Meter, Mole, Radian, Second};
pub use derived::*;
pub use temperature::{Celsius, DegreeCelsius, DegreeFahrenheit, Rankine};

pub use scaled::{
    Bar, Centimeter, CubicMillimeter, Degree, ElectronVolt, FootPerSecond, Gram,
    GramPerCubicCentimeter, JoulePerMilliliter, Kilohertz, Kilometer, KilometerPerHour, Kilopascal,
    Knot, MegaElectronVolt, Megahertz, Megapascal, MicromolePerLiter, Microsecond, MilePerHour,
    Millimeter, MillimeterOfMercury, MillipascalSecond, Millisecond, MolePerLiter, Nanometer,
    PerCentimeter, PoundPerSquareInch, SquareCentimeter, SquareCentimeterPerGram,
    StandardAtmosphere,
};

mod physical_constants;
pub use physical_constants::{
    FaradPerMeter, HenryPerMeter, JouleSecond, PerMole, WattPerSquareMeterKelvinToFourth,
};

pub use brochure::{
    AmperePerMeter, AmperePerSquareMeter, CandelaPerSquareMeter, CubicMeterPerKilogram, Henry,
    Katal, KatalPerCubicMeter, KilogramPerSquareMeter, Lumen, LumenPerWatt, Lux, NewtonMeter, One,
    RadianPerSecond, RadianPerSecondSquared, Steradian, VoltPerMeter, WattPerSquareMeterSteradian,
    WattPerSteradian, Weber,
};
pub use non_si::{
    Angstrom, Arcminute, Arcsecond, ArcsecondAs, Are, AstronomicalUnit, AtomicMassUnit, Barn, Bit,
    Dalton, Day, DegreeOfArc, Gal, Hectare, Hour, Liter, LiterLowercase, Minute, NauticalMile,
    PartsPerMillion, Percent, Tonne, Var, VoltAmpere,
};
