//! The runtime dimension tag and its algebra.

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

    /// Tag of the product of two quantities.
    ///
    /// Exponents add and the semantic discriminant normalizes to
    /// [`SemanticTag::Base`], matching `MultiplyDimension`, whose output type
    /// is always `Dimension<..., BaseSemantics>`.
    ///
    /// # Errors
    ///
    /// Returns [`TagOverflow`] when an exponent leaves `i8`. Aequitas' own
    /// inventory spans -4 to 4, so this is reachable only by repeated
    /// multiplication of already-extreme tags.
    pub fn multiply(self, rhs: Self) -> Result<Self, TagOverflow> {
        self.combine(rhs, i8::checked_add)
    }

    /// Tag of the quotient of two quantities.
    ///
    /// Exponents subtract and the semantic discriminant normalizes to
    /// [`SemanticTag::Base`], matching `DivideDimension`.
    ///
    /// # Errors
    ///
    /// Returns [`TagOverflow`] when an exponent leaves `i8`.
    pub fn divide(self, rhs: Self) -> Result<Self, TagOverflow> {
        self.combine(rhs, i8::checked_sub)
    }

    /// Tag of the reciprocal.
    ///
    /// # Errors
    ///
    /// Returns [`TagOverflow`] when negating an exponent leaves `i8`.
    pub fn reciprocal(self) -> Result<Self, TagOverflow> {
        Self::DIMENSIONLESS.divide(self)
    }

    /// Tag of this dimension raised to an integer power.
    ///
    /// # Errors
    ///
    /// Returns [`TagOverflow`] when an exponent product leaves `i8`.
    pub fn powi(self, exponent: i8) -> Result<Self, TagOverflow> {
        let mut scaled = [0_i8; AXES];
        for (axis, slot) in scaled.iter_mut().enumerate() {
            *slot = self.exponents[axis]
                .checked_mul(exponent)
                .ok_or(TagOverflow)?;
        }
        // A power normalizes semantics for the same reason a product does: the
        // marker describes one quantity's role, not that of its powers.
        Ok(Self::new(scaled, SemanticTag::Base))
    }

    /// Tag of the `root`-th root, when every exponent divides exactly.
    ///
    /// Mirrors `SqrtDimension`/`CbrtDimension`, which exist only for
    /// dimensions whose exponents divide exactly — the type-level surface has
    /// no impl for the others, and this is the runtime image of that absence.
    ///
    /// # Errors
    ///
    /// Returns [`TagNotDivisible`] when any exponent is not a multiple of
    /// `root`.
    pub fn root(self, root: i8) -> Result<Self, TagNotDivisible> {
        let mut divided = [0_i8; AXES];
        for (axis, slot) in divided.iter_mut().enumerate() {
            let exponent = self.exponents[axis];
            if root == 0 || exponent % root != 0 {
                return Err(TagNotDivisible {
                    tag: self,
                    root,
                    axis,
                });
            }
            *slot = exponent / root;
        }
        Ok(Self::new(divided, SemanticTag::Base))
    }

    fn combine(self, rhs: Self, op: fn(i8, i8) -> Option<i8>) -> Result<Self, TagOverflow> {
        let mut combined = [0_i8; AXES];
        for (axis, slot) in combined.iter_mut().enumerate() {
            *slot = op(self.exponents[axis], rhs.exponents[axis]).ok_or(TagOverflow)?;
        }
        Ok(Self::new(combined, SemanticTag::Base))
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

/// An exponent left the `i8` range during tag arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagOverflow;

impl fmt::Display for TagOverflow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("dimension exponent left the representable range")
    }
}

/// A root was requested of a dimension whose exponents do not divide exactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagNotDivisible {
    /// The dimension the root was requested of.
    pub tag: DimensionTag,
    /// The requested root.
    pub root: i8,
    /// Index of the first axis that does not divide.
    pub axis: usize,
}

impl fmt::Display for TagNotDivisible {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot take root {} of `{}`: the {} exponent {} is not a multiple of {}",
            self.root, self.tag, AXIS_SYMBOLS[self.axis], self.tag.exponents[self.axis], self.root,
        )
    }
}
