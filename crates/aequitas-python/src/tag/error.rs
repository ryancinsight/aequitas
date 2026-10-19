//! The two ways tag arithmetic can refuse.
//!
//! Both are plain values rather than strings: a refusal has to name the axis
//! and the amount it could not represent, and the caller decides how to word
//! that for Python. `TagNotDivisible` carries the offending axis for the same
//! reason — the message names it, and a consumer could match on it.

use core::fmt;

use super::model::{AXIS_SYMBOLS, DimensionTag};

/// An exponent left the `i8` range during tag arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagOverflow;

impl fmt::Display for TagOverflow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("dimension exponent left the representable range")
    }
}

/// A dimension root has no exact, representable signed-byte exponent vector.
///
/// This includes zero roots, inexact division, and the `-128 / -1` overflow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagNotDivisible {
    /// The dimension the root was requested of.
    pub tag: DimensionTag,
    /// The requested root.
    pub root: i8,
    /// Index of the first axis without an exact, representable quotient.
    pub axis: usize,
}

impl fmt::Display for TagNotDivisible {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.root == -1 && self.tag.exponents()[self.axis] == i8::MIN {
            return write!(
                formatter,
                "cannot take root -1 of `{}`: the {} exponent quotient 128 is outside the representable range",
                self.tag, AXIS_SYMBOLS[self.axis],
            );
        }
        write!(
            formatter,
            "cannot take root {} of `{}`: the {} exponent {} is not a multiple of {}",
            self.root,
            self.tag,
            AXIS_SYMBOLS[self.axis],
            self.tag.exponents()[self.axis],
            self.root,
        )
    }
}
