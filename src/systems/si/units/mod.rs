//! Zero-sized SI and non-SI unit markers.
//!
//! [`Celsius`], [`DegreeCelsius`] and [`DegreeFahrenheit`] use affine
//! conversions for absolute temperatures and linear conversions for intervals.

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

mod base;
mod brochure;
mod derived;
mod non_si;
mod prefix_bases;
mod scaled;
mod temperature;

pub use base::{Ampere, Candela, Kelvin, Kilogram, Meter, Mole, Radian, Second};
pub use derived::{
    Becquerel, BecquerelPerCubicMeter, BecquerelPerMilliliter, Centigray, Coulomb,
    CoulombPerCubicMeter, CoulombPerKilogram, CoulombPerSquareMeter, CubicMeter,
    CubicMeterPerSecond, Curie, Farad, FaradSquareMeter, Gigabecquerel, Gray, GrayPerSecond, Hertz,
    Joule, JoulePerCubicMeter, JoulePerKelvin, JoulePerKilogram, JoulePerKilogramKelvin,
    JoulePerMole, JoulePerMoleKelvin, JoulePerSquareMeter, Kilobecquerel,
    KilobecquerelPerMilliliter, KilogramPerCubicMeter, KilogramPerCubicMeterKelvin,
    KilogramPerCubicMeterSecond, KilogramPerSecond, Megabecquerel, MeterPerSecond,
    MeterPerSecondKelvin, MeterPerSecondSquared, MeterPerVolt, Microcurie, Microgray, Microsievert,
    MicrosievertPerHour, Millicurie, Milligray, Millisievert, MolePerCubicMeter,
    MolePerCubicMeterSecond, MolePerSquareMeterSecond, Newton, NewtonPerMeter, Ohm, Pascal,
    PascalPerSecond, PascalPerVolt, PascalSecond, PerCubicMeter, PerKelvin, PerMeter,
    PerMeterKelvin, PerSecond, PerSquareKelvin, Rad, Rayl, Rem, Roentgen, Siemens, SiemensPerMeter,
    Sievert, SievertPerSecond, SquareMeter, SquareMeterPerKilogram, SquareMeterPerSecond, Tesla,
    Volt, VoltPerPascal, Watt, WattPerCubicMeter, WattPerKilogram, WattPerMeterFourth,
    WattPerMeterKelvin, WattPerSquareMeter,
};
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
pub use physical_constants::{FaradPerMeter, HenryPerMeter, JouleSecond, PerMole};

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
