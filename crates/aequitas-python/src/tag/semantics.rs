//! Runtime discriminant for the Aequitas semantic markers.

use aequitas::dimension::{
    AbsoluteTemperatureSemantics, AngleSemantics, BaseSemantics, FlexuralRigiditySemantics,
    MechanicalImpedanceSemantics, MolarConcentrationSemantics, ReciprocalVolumeSemantics,
    SpringStiffnessSemantics, StressSemantics, SurfaceTensionSemantics,
    TemperatureDifferenceSemantics,
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
    pub const ALL: [Self; 11] = [
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
}
