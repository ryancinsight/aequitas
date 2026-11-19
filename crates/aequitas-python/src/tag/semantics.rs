//! Runtime discriminant for the Aequitas semantic markers.

use aequitas::dimension::{
    AbsoluteTemperatureSemantics, ActivityConcentrationSemantics, ActivitySemantics,
    AngleSemantics, AngularAccelerationSemantics, AngularVelocitySemantics, BaseSemantics,
    DoseEquivalentRateSemantics, DoseEquivalentSemantics, FlexuralRigiditySemantics,
    InformationSemantics, LuminanceSemantics, LuminousFluxSemantics, MechanicalImpedanceSemantics,
    MolarConcentrationSemantics, RadianceSemantics, RadiantIntensitySemantics,
    RadiationExposureSemantics, ReactivePowerSemantics, ReciprocalVolumeSemantics,
    SolidAngleSemantics, SpringStiffnessSemantics, StressSemantics, SurfaceTensionSemantics,
    TemperatureDifferenceSemantics, TorqueSemantics,
};

/// Runtime image of a type-level semantic marker.
///
/// The SI exponent vector cannot separate stress from pressure, an absolute
/// temperature from a temperature difference, or surface tension from energy
/// per area. Those distinctions live in the marker, so a runtime tag that
/// carried exponents alone would merge quantities Aequitas deliberately keeps
/// apart. This enum is that marker's value-level counterpart.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemanticTag {
    /// No semantic distinction beyond the exponent vector.
    #[default]
    Base,
    /// Absolute thermodynamic temperature.
    AbsoluteTemperature,
    /// Thermodynamic temperature difference.
    TemperatureDifference,
    /// Interfacial or surface tension.
    SurfaceTension,
    /// Mechanical spring stiffness.
    SpringStiffness,
    /// Force-per-velocity mechanical impedance.
    MechanicalImpedance,
    /// Flexural rigidity.
    FlexuralRigidity,
    /// Plane or rotational angle.
    Angle,
    /// Geometric reciprocal-volume coefficient.
    ReciprocalVolume,
    /// Mechanical stress.
    Stress,
    /// Amount-of-substance concentration per volume.
    MolarConcentration,
    /// Radionuclide activity.
    Activity,
    /// Radionuclide activity per volume.
    ActivityConcentration,
    /// Radiation-weighted dose.
    DoseEquivalent,
    /// Radiation-weighted dose per time.
    DoseEquivalentRate,
    /// Ionizing charge liberated per mass of air.
    RadiationExposure,
    /// Angular acceleration in radians per second squared.
    AngularAcceleration,
    /// Angular velocity or angular frequency in radians per second.
    AngularVelocity,
    /// An amount of information.
    Information,
    /// Luminous intensity per projected area.
    Luminance,
    /// Luminous flux in lumens.
    LuminousFlux,
    /// Radiant flux per projected area per solid angle.
    Radiance,
    /// Radiant flux per solid angle.
    RadiantIntensity,
    /// Reactive power in var.
    ReactivePower,
    /// Solid angle in steradians.
    SolidAngle,
    /// Moment of force.
    Torque,
}

impl SemanticTag {
    /// Stable wire name for this marker.
    ///
    /// The name crosses the FFI boundary inside the dimension tag, so a
    /// consumer compares against it directly. Renaming a variant's string is a
    /// breaking protocol change.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::AbsoluteTemperature => "absolute_temperature",
            Self::TemperatureDifference => "temperature_difference",
            Self::SurfaceTension => "surface_tension",
            Self::SpringStiffness => "spring_stiffness",
            Self::MechanicalImpedance => "mechanical_impedance",
            Self::FlexuralRigidity => "flexural_rigidity",
            Self::Angle => "angle",
            Self::ReciprocalVolume => "reciprocal_volume",
            Self::Stress => "stress",
            Self::MolarConcentration => "molar_concentration",
            Self::Activity => "activity",
            Self::ActivityConcentration => "activity_concentration",
            Self::DoseEquivalent => "dose_equivalent",
            Self::DoseEquivalentRate => "dose_equivalent_rate",
            Self::RadiationExposure => "radiation_exposure",
            Self::AngularAcceleration => "angular_acceleration",
            Self::AngularVelocity => "angular_velocity",
            Self::Information => "information",
            Self::Luminance => "luminance",
            Self::LuminousFlux => "luminous_flux",
            Self::Radiance => "radiance",
            Self::RadiantIntensity => "radiant_intensity",
            Self::ReactivePower => "reactive_power",
            Self::SolidAngle => "solid_angle",
            Self::Torque => "torque",
        }
    }

    /// The marker a wire name names, or `None` when it names none.
    ///
    /// The inverse of [`Self::name`], kept beside it so both directions of one
    /// vocabulary live in one place: renaming a variant's string moves this
    /// mapping too, and the round-trip test that pairs them covers both.
    ///
    /// A `match` rather than a scan of [`Self::ALL`] because this resolves on
    /// the read path of every Python-passed quantity, where a scan compares
    /// every earlier name before it reaches the right one.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "base" => Some(Self::Base),
            "absolute_temperature" => Some(Self::AbsoluteTemperature),
            "temperature_difference" => Some(Self::TemperatureDifference),
            "surface_tension" => Some(Self::SurfaceTension),
            "spring_stiffness" => Some(Self::SpringStiffness),
            "mechanical_impedance" => Some(Self::MechanicalImpedance),
            "flexural_rigidity" => Some(Self::FlexuralRigidity),
            "angle" => Some(Self::Angle),
            "reciprocal_volume" => Some(Self::ReciprocalVolume),
            "stress" => Some(Self::Stress),
            "molar_concentration" => Some(Self::MolarConcentration),
            "activity" => Some(Self::Activity),
            "activity_concentration" => Some(Self::ActivityConcentration),
            "dose_equivalent" => Some(Self::DoseEquivalent),
            "dose_equivalent_rate" => Some(Self::DoseEquivalentRate),
            "radiation_exposure" => Some(Self::RadiationExposure),
            "angular_acceleration" => Some(Self::AngularAcceleration),
            "angular_velocity" => Some(Self::AngularVelocity),
            "information" => Some(Self::Information),
            "luminance" => Some(Self::Luminance),
            "luminous_flux" => Some(Self::LuminousFlux),
            "radiance" => Some(Self::Radiance),
            "radiant_intensity" => Some(Self::RadiantIntensity),
            "reactive_power" => Some(Self::ReactivePower),
            "solid_angle" => Some(Self::SolidAngle),
            "torque" => Some(Self::Torque),
            _ => None,
        }
    }

    /// Small stable integer image, for hashing and compact encodings.
    ///
    /// Distinct per variant; the wire form remains [`Self::name`], so this
    /// value is never serialized and may be renumbered.
    #[must_use]
    pub const fn discriminant(self) -> u8 {
        self as u8
    }

    /// Every marker in declaration order.
    ///
    /// Exhaustive by construction: adding a variant without extending this
    /// array fails the round-trip test in this module's test module.
    pub const ALL: [Self; 26] = [
        Self::Base,
        Self::AbsoluteTemperature,
        Self::TemperatureDifference,
        Self::SurfaceTension,
        Self::SpringStiffness,
        Self::MechanicalImpedance,
        Self::FlexuralRigidity,
        Self::Angle,
        Self::ReciprocalVolume,
        Self::Stress,
        Self::MolarConcentration,
        Self::Activity,
        Self::ActivityConcentration,
        Self::DoseEquivalent,
        Self::DoseEquivalentRate,
        Self::RadiationExposure,
        Self::AngularAcceleration,
        Self::AngularVelocity,
        Self::Information,
        Self::Luminance,
        Self::LuminousFlux,
        Self::Radiance,
        Self::RadiantIntensity,
        Self::ReactivePower,
        Self::SolidAngle,
        Self::Torque,
    ];
}

/// Type-level marker with a runtime discriminant.
///
/// Implemented for every semantic marker Aequitas exports. The trait is local
/// and the markers are foreign, so these impls are coherent; a marker added
/// upstream without an impl here fails to compile at its first use in a
/// quantity registration.
pub trait TaggedSemantics {
    /// Runtime image of this marker.
    const TAG: SemanticTag;
}

macro_rules! tagged_semantics {
    ($($marker:ty => $tag:ident),+ $(,)?) => {
        $(
            impl TaggedSemantics for $marker {
                const TAG: SemanticTag = SemanticTag::$tag;
            }
        )+
    };
}

tagged_semantics! {
    BaseSemantics => Base,
    AbsoluteTemperatureSemantics => AbsoluteTemperature,
    TemperatureDifferenceSemantics => TemperatureDifference,
    SurfaceTensionSemantics => SurfaceTension,
    SpringStiffnessSemantics => SpringStiffness,
    MechanicalImpedanceSemantics => MechanicalImpedance,
    FlexuralRigiditySemantics => FlexuralRigidity,
    AngleSemantics => Angle,
    ReciprocalVolumeSemantics => ReciprocalVolume,
    StressSemantics => Stress,
    MolarConcentrationSemantics => MolarConcentration,
    ActivitySemantics => Activity,
    ActivityConcentrationSemantics => ActivityConcentration,
    DoseEquivalentSemantics => DoseEquivalent,
    DoseEquivalentRateSemantics => DoseEquivalentRate,
    RadiationExposureSemantics => RadiationExposure,
    AngularAccelerationSemantics => AngularAcceleration,
    AngularVelocitySemantics => AngularVelocity,
    InformationSemantics => Information,
    LuminanceSemantics => Luminance,
    LuminousFluxSemantics => LuminousFlux,
    RadianceSemantics => Radiance,
    RadiantIntensitySemantics => RadiantIntensity,
    ReactivePowerSemantics => ReactivePower,
    SolidAngleSemantics => SolidAngle,
    TorqueSemantics => Torque,
}
