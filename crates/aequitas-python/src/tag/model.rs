//! The runtime dimension tag: its value, its accessors, and its rendering.
//!
//! The algebra lives in [`super::algebra`] and the two failures it can report
//! in [`super::error`], so this leaf is the tag's identity and nothing else.

use core::fmt;

use super::semantics::SemanticTag;

/// Number of SI base axes carried by a dimension tag.
pub const AXES: usize = 7;

/// Symbols for the SI base axes, in tag order.
pub const AXIS_SYMBOLS: [&str; AXES] = ["m", "kg", "s", "A", "K", "mol", "cd"];

/// Runtime image of an Aequitas type-level dimension.
///
/// Carries the seven SI base exponents plus the semantic discriminant. Two
/// tags are the same dimension only when both halves agree, so `Stress` and
/// `Pressure` — identical exponents, different markers — do not unify.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DimensionTag {
    exponents: [i8; AXES],
    semantics: SemanticTag,
}

impl DimensionTag {
    /// Construct a tag from its exponent vector and semantic discriminant.
    #[must_use]
    pub const fn new(exponents: [i8; AXES], semantics: SemanticTag) -> Self {
        Self {
            exponents,
            semantics,
        }
    }

    /// The dimensionless tag with no semantic distinction.
    pub const DIMENSIONLESS: Self = Self::new([0; AXES], SemanticTag::Base);

    /// SI base exponents in length, mass, time, current, temperature, amount,
    /// luminosity order.
    #[must_use]
    pub const fn exponents(&self) -> [i8; AXES] {
        self.exponents
    }

    /// Semantic discriminant.
    #[must_use]
    pub const fn semantics(&self) -> SemanticTag {
        self.semantics
    }

    /// Equality usable in constant evaluation, where `PartialEq` is not.
    ///
    /// The generated class inventory asserts at compile time that every alias
    /// grouped under a class has the class's dimension, and that no two
    /// classes share one; both comparisons happen in `const` items.
    #[must_use]
    pub const fn same_as(self, other: Self) -> bool {
        let mut axis = 0;
        while axis < AXES {
            if self.exponents[axis] != other.exponents[axis] {
                return false;
            }
            axis += 1;
        }
        self.semantics.discriminant() == other.semantics.discriminant()
    }

    /// True when every exponent is zero, whatever the semantics.
    ///
    /// An angle is dimensionless in this sense while remaining distinct from a
    /// bare scalar, which is why the semantic half is not consulted.
    #[must_use]
    pub fn is_dimensionless(&self) -> bool {
        self.exponents.iter().all(|&e| e == 0)
    }
}

impl fmt::Display for DimensionTag {
    /// Renders the tag as a unit-algebra string, e.g. `kg m^-1 s^-2`.
    ///
    /// A non-base semantic marker is appended in brackets so stress and
    /// pressure are distinguishable in a message, not only in a comparison.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for (axis, &exponent) in self.exponents.iter().enumerate() {
            if exponent == 0 {
                continue;
            }
            if wrote {
                formatter.write_str(" ")?;
            }
            wrote = true;
            formatter.write_str(AXIS_SYMBOLS[axis])?;
            if exponent != 1 {
                write!(formatter, "^{exponent}")?;
            }
        }
        if !wrote {
            formatter.write_str("dimensionless")?;
        }
        if self.semantics != SemanticTag::Base {
            write!(formatter, " [{}]", self.semantics.name())?;
        }
        Ok(())
    }
}
