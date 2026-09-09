//! Whether a parameter admits an infinite magnitude.
//!
//! A physical magnitude is finite. `NaN` is never one, and infinity is one only
//! where a call site publishes it as a sentinel — a focus distance of `inf`
//! meaning "no focusing" is the case this exists for. The default is therefore
//! strict, and a site that wants the sentinel says so in its parameter type
//! rather than in a comment.

mod sealed {
    /// Closes [`Finiteness`](super::Finiteness) to this module.
    ///
    /// The trait's contract is the pair of policies below, and `checked` reads
    /// `ALLOWS_INFINITE` to decide whether a magnitude extracts. Left open, a
    /// downstream crate could implement it with `ALLOWS_INFINITE = true` and
    /// admit infinity into a parameter that never declared the sentinel --
    /// which is the defect this module exists to prevent, reintroduced through
    /// the type parameter.
    pub trait Sealed {}
}

/// The finiteness a [`Dimensioned`](super::Dimensioned) parameter enforces.
///
/// Implemented by [`Finite`] and [`MayBeInfinite`] only, and sealed so that
/// stays true: a consumer selects one through the parameter type and cannot
/// add a third. A policy declared outside this crate does not compile, so the
/// sentinel cannot be re-admitted through the type parameter:
///
/// ```compile_fail
/// use aequitas_python::Finiteness;
///
/// struct Permissive;
///
/// impl Finiteness for Permissive {
///     const ALLOWS_INFINITE: bool = true;
///     const ADMITS: &'static str = "anything";
/// }
/// ```
pub trait Finiteness: sealed::Sealed {
    /// Whether `±inf` extracts successfully.
    const ALLOWS_INFINITE: bool;

    /// What the parameter admits, for the rejection message.
    const ADMITS: &'static str;
}

/// The default: the magnitude must be finite.
#[derive(Debug, Clone, Copy)]
pub struct Finite;

impl sealed::Sealed for Finite {}

impl Finiteness for Finite {
    const ALLOWS_INFINITE: bool = false;
    const ADMITS: &'static str = "a finite magnitude";
}

/// Admits `±inf`, for a parameter that publishes infinity as a sentinel.
///
/// `NaN` stays rejected: it is not a sentinel, it is the absence of a value.
#[derive(Debug, Clone, Copy)]
pub struct MayBeInfinite;

impl sealed::Sealed for MayBeInfinite {}

impl Finiteness for MayBeInfinite {
    const ALLOWS_INFINITE: bool = true;
    const ADMITS: &'static str = "a finite magnitude or an infinite sentinel";
}
