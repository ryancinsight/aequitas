//! Zero-sized SI unit markers.
//!
//! Linear throughout except [`DegreeCelsius`] and [`DegreeFahrenheit`], whose
//! degrees carry an offset when they name a temperature and none when they
//! name a difference.

mod base;
mod derived;
mod scaled;
mod temperature;

pub use base::{Ampere, Candela, Kelvin, Kilogram, Meter, Mole, Radian, Second};
pub use derived::{
    Coulomb, CoulombPerCubicMeter, CoulombPerSquareMeter, CubicMeter, CubicMeterPerSecond, Farad,
    FaradSquareMeter, Gray, GrayPerSecond, Hertz, Joule, JoulePerCubicMeter, JoulePerKelvin,
    JoulePerKilogram, JoulePerKilogramKelvin, JoulePerMole, JoulePerMoleKelvin,
    JoulePerSquareMeter, KilogramPerCubicMeter, KilogramPerCubicMeterKelvin,
    KilogramPerCubicMeterSecond, KilogramPerSecond, MeterPerSecond, MeterPerSecondKelvin,
    MeterPerSecondSquared, MeterPerVolt, MolePerCubicMeter, MolePerCubicMeterSecond,
    MolePerSquareMeterSecond, Newton, NewtonPerMeter, Ohm, Pascal, PascalPerSecond, PascalPerVolt,
    PascalSecond, PerCubicMeter, PerKelvin, PerMeter, PerMeterKelvin, PerSecond, PerSquareKelvin,
    Rayl, Siemens, SiemensPerMeter, SquareMeter, SquareMeterPerKilogram, SquareMeterPerSecond,
    Volt, VoltPerPascal, Watt, WattPerCubicMeter, WattPerKilogram, WattPerMeterFourth,
    WattPerMeterKelvin, WattPerSquareMeter,
};
pub use temperature::{DegreeCelsius, DegreeFahrenheit, Rankine};

pub use scaled::{
    Bar, Centimeter, CubicMillimeter, Degree, ElectronVolt, FootPerSecond, Gram,
    GramPerCubicCentimeter, JoulePerMilliliter, Kilohertz, Kilometer, KilometerPerHour, Kilopascal,
    Knot, MegaElectronVolt, Megahertz, Megapascal, MicromolePerLiter, Microsecond, MilePerHour,
    Millimeter, MillimeterOfMercury, MillipascalSecond, Millisecond, MolePerLiter, Nanometer,
    PerCentimeter, PoundPerSquareInch, SquareCentimeter, SquareCentimeterPerGram,
    StandardAtmosphere,
};
