//! Named coherent units and remaining examples from SI Brochure Tables 4–6.

linear_units! {
    /// Unit one, the identity in the algebra of SI units.
    One, Dimensionless, "1", 1.0;
    /// Steradian, the solid angle subtended by an area equal to the radius squared.
    Steradian, SolidAngle, "sr", 1.0;
    /// Weber, one volt second.
    Weber, MagneticFlux, "Wb", 1.0;
    /// Henry, one weber per ampere.
    Henry, Inductance, "H", 1.0;
    /// Lumen, one candela steradian.
    Lumen, LuminousFlux, "lm", 1.0;
    /// Lux, one lumen per square metre.
    Lux, Illuminance, "lx", 1.0;
    /// Katal, one mole per second of catalytic activity.
    Katal, CatalyticActivity, "kat", 1.0;
    /// Kilogram per square metre, the coherent surface density unit.
    KilogramPerSquareMeter, SurfaceDensity, "kg/m²", 1.0;
    /// Cubic metre per kilogram, the coherent specific volume unit.
    CubicMeterPerKilogram, SpecificVolume, "m³/kg", 1.0;
    /// Ampere per square metre, the coherent current density unit.
    AmperePerSquareMeter, CurrentDensity, "A/m²", 1.0;
    /// Ampere per metre, the coherent magnetic field strength unit.
    AmperePerMeter, MagneticFieldStrength, "A/m", 1.0;
    /// Candela per square metre, the coherent luminance unit.
    CandelaPerSquareMeter, Luminance, "cd/m²", 1.0;
    /// Newton metre, the coherent unit of torque.
    NewtonMeter, Torque, "N·m", 1.0;
    /// Radian per second, the coherent unit of angular velocity and angular frequency.
    RadianPerSecond, AngularVelocity, "rad/s", 1.0;
    /// Radian per second squared, the coherent unit of angular acceleration.
    RadianPerSecondSquared, AngularAcceleration, "rad/s²", 1.0;
    /// Volt per metre, the coherent electric field strength unit.
    VoltPerMeter, ElectricFieldStrength, "V/m", 1.0;
    /// Watt per steradian, the coherent radiant intensity unit.
    WattPerSteradian, RadiantIntensity, "W/sr", 1.0;
    /// Watt per square metre steradian, the coherent radiance unit.
    WattPerSquareMeterSteradian, Radiance, "W/(m²·sr)", 1.0;
    /// Katal per cubic metre, the coherent catalytic activity concentration unit.
    KatalPerCubicMeter, CatalyticActivityConcentration, "kat/m³", 1.0;
    /// Lumen per watt, the coherent luminous efficacy unit.
    LumenPerWatt, LuminousEfficacy, "lm/W", 1.0;
}
