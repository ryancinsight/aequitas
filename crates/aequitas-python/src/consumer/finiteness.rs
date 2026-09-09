//! Whether a parameter admits an infinite magnitude.
//!
//! A physical magnitude is finite. `NaN` is never one, and infinity is one only
//! where a call site publishes it as a sentinel — a focus distance of `inf`
//! meaning "no focusing" is the case this exists for. The default is therefore
//! strict, and a site that wants the sentinel says so in its parameter type
//! rather than in a comment.

/// The finiteness a [`Dimensioned`](super::Dimensioned) parameter enforces.
///
/// Implemented by [`Finite`] and [`MayBeInfinite`] only; a consumer selects one
/// through the parameter type and never implements this itself.
pub trait Finiteness {
    /// Whether `±inf` extracts successfully.
    const ALLOWS_INFINITE: bool;

    /// What the parameter admits, for the rejection message.
    const ADMITS: &'static str;
}

/// The default: the magnitude must be finite.
#[derive(Debug, Clone, Copy)]
pub struct Finite;

impl Finiteness for Finite {
    const ALLOWS_INFINITE: bool = false;
    const ADMITS: &'static str = "a finite magnitude";
}

/// Admits `±inf`, for a parameter that publishes infinity as a sentinel.
///
/// `NaN` stays rejected: it is not a sentinel, it is the absence of a value.
#[derive(Debug, Clone, Copy)]
pub struct MayBeInfinite;

impl Finiteness for MayBeInfinite {
    const ALLOWS_INFINITE: bool = true;
    const ADMITS: &'static str = "a finite magnitude or an infinite sentinel";
}
