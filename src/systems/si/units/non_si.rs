//! Linear non-SI units from SI Brochure Table 8 and sections 3 and 5.4.7.
//!
//! Decimal coefficients are exact definitions apart from the CODATA dalton
//! estimate. The definitions and any irrational ratios are stored as
//! binary64 metadata and converted by the selected scalar's native operations.

use crate::{systems::si::dimensions, unit::LinearUnit};

use super::Degree;

pub(super) const MINUTE_IN_SECONDS: f64 = 60.0;
pub(super) const HOUR_IN_SECONDS: f64 = 60.0 * MINUTE_IN_SECONDS;
pub(super) const NAUTICAL_MILE_IN_METERS: f64 = 1852.0;
pub(super) const LITER_IN_CUBIC_METERS: f64 = 1.0e-3;

linear_units! {
    /// Minute, 60 seconds.
    Minute, Time, "min", MINUTE_IN_SECONDS;
    /// Hour, 60 minutes.
    Hour, Time, "h", HOUR_IN_SECONDS;
    /// Day as a unit of elapsed time, 24 hours.
    Day, Time, "d", 24.0 * HOUR_IN_SECONDS;
    /// Degree of arc with the SI Brochure symbol.
    DegreeOfArc, Angle, "°", <Degree as LinearUnit<dimensions::Angle>>::SCALE;
    /// Arcminute, one sixtieth of a degree.
    Arcminute, Angle, "′", <Degree as LinearUnit<dimensions::Angle>>::SCALE / 60.0;
    /// Arcsecond, one sixtieth of an arcminute. Use the symbol `as` with a prefix.
    Arcsecond, Angle, "″", <Arcminute as LinearUnit<dimensions::Angle>>::SCALE / 60.0;
    /// Arcsecond with its alternative symbol `as`, used with decimal prefixes.
    ArcsecondAs, Angle, "as", <Arcsecond as LinearUnit<dimensions::Angle>>::SCALE;
    /// Are, the area of a square decametre.
    Are, Area, "a", 1.0e2;
    /// Hectare, the area of a square hectometre.
    Hectare, Area, "ha", 1.0e4;
    /// Barn, one hundred square femtometres.
    Barn, Area, "b", 1.0e-28;
    /// Litre with uppercase symbol, the volume of a cubic decimetre.
    Liter, Volume, "L", LITER_IN_CUBIC_METERS;
    /// Litre with its alternative lowercase symbol.
    LiterLowercase, Volume, "l", LITER_IN_CUBIC_METERS;
    /// Tonne, one megagram.
    Tonne, Mass, "t", 1.0e3;
    /// Angstrom, one tenth of a nanometre.
    Angstrom, Length, "Å", 1.0e-10;
    /// Gal, one centimetre per second squared.
    Gal, Acceleration, "Gal", 1.0e-2;
    /// Dalton, one twelfth of a free neutral carbon-12 atom's mass at rest in its ground state.
    ///
    /// The 2022 CODATA estimate is 1.660 539 068 92(52) × 10⁻²⁷ kg.
    Dalton, Mass, "Da", 1.660_539_068_92e-27;
    /// Unified atomic mass unit, the same unit as the dalton.
    AtomicMassUnit, Mass, "u", <Dalton as LinearUnit<dimensions::Mass>>::SCALE;
    /// International nautical mile, exactly 1852 metres. The catalogue uses `nmi`.
    NauticalMile, Length, "nmi", NAUTICAL_MILE_IN_METERS;
    /// Astronomical unit, exactly 149 597 870 700 metres.
    AstronomicalUnit, Length, "au", 149_597_870_700.0;
    /// Volt ampere, expressed here as the coherent unit of reactive power.
    VoltAmpere, ReactivePower, "V·A", 1.0;
    /// Var, the special name of the volt ampere for reactive power.
    Var, ReactivePower, "var", <VoltAmpere as LinearUnit<dimensions::ReactivePower>>::SCALE;
    /// One hundredth of a dimensionless ratio.
    Percent, Dimensionless, "%", 1.0e-2;
    /// One millionth of a dimensionless ratio.
    PartsPerMillion, Dimensionless, "ppm", 1.0e-6;
    /// Bit, the information unit used in the brochure's prefix example.
    Bit, Information, "bit", 1.0;
}
