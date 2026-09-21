//! Tag arithmetic: the four operations and the one refusal they share.
//!
//! Each operation mirrors a law-crate dimension derivative — `MultiplyDimension`,
//! `DivideDimension`, `PowDimension`, `SqrtDimension`/`CbrtDimension` — so the
//! outputs here are the runtime image of what `rustc` computes from the type
//! parameters.

use super::{AXES, DimensionTag, SemanticTag, TagNotDivisible, TagOverflow};

impl DimensionTag {
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
        let axes = self.exponents();
        let mut scaled = [0_i8; AXES];
        for (axis, slot) in scaled.iter_mut().enumerate() {
            *slot = axes[axis].checked_mul(exponent).ok_or(TagOverflow)?;
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
        let axes = self.exponents();
        let mut divided = [0_i8; AXES];
        for (axis, slot) in divided.iter_mut().enumerate() {
            let exponent = axes[axis];
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

    /// Combine two exponent vectors axis by axis under one checked operation.
    ///
    /// `op` is a generic parameter rather than a `fn` pointer: a pointer made
    /// this a non-zero-cost abstraction, forcing an indirect call per axis
    /// through a loop the optimizer cannot then see into. Monomorphized, each
    /// of the two call sites inlines its own `checked_add`/`checked_sub` and
    /// the loop is a straight-line sequence over a fixed seven elements.
    fn combine<F>(self, rhs: Self, op: F) -> Result<Self, TagOverflow>
    where
        F: Fn(i8, i8) -> Option<i8>,
    {
        let left = self.exponents();
        let right = rhs.exponents();
        let mut combined = [0_i8; AXES];
        for (axis, slot) in combined.iter_mut().enumerate() {
            *slot = op(left[axis], right[axis]).ok_or(TagOverflow)?;
        }
        Ok(Self::new(combined, SemanticTag::Base))
    }
}
