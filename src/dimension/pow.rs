//! Type-level arbitrary integer powers of physical dimensions.
//!
//! [`PowDimension`] multiplies every SI base exponent by a type-level integer
//! `P`, so `powi::<P2>()` squares a dimension and `powi::<N1>()` takes the
//! reciprocal. Unlike `uom`, whose `powi` only scales the *value* at runtime
//! and hardcodes a per-quantity dimension, Aequitas raises the dimension
//! itself at the type level — `Length::powi::<P2>()` is an `Area`, and
//! `Time::powi::<N1>()` is a `ReciprocalTime`. This is a strict capability
//! extension over `uom`'s integer-exponent model.
//!
//! The semantics marker is normalized to [`BaseSemantics`], matching
//! [`MultiplyDimension`] / [`DivideDimension`] and the root traits.

use core::ops::Mul;

use typenum::{Integer, Prod};

use super::{BaseSemantics, Dimension};

/// Type-level integer power of a physical dimension.
#[diagnostic::on_unimplemented(
    message = "this Aequitas dimension cannot be raised to the requested power",
    note = "PowDimension requires typenum integer exponents throughout"
)]
pub trait PowDimension<P> {
    /// Dimension produced by raising to the `P`-th power.
    type Output;
}

impl<L, M, T, I, Th, N, J, Semantics, P> PowDimension<P>
    for Dimension<L, M, T, I, Th, N, J, Semantics>
where
    L: Integer + Mul<P>,
    M: Integer + Mul<P>,
    T: Integer + Mul<P>,
    I: Integer + Mul<P>,
    Th: Integer + Mul<P>,
    N: Integer + Mul<P>,
    J: Integer + Mul<P>,
    P: Integer,
    Prod<L, P>: Integer,
    Prod<M, P>: Integer,
    Prod<T, P>: Integer,
    Prod<I, P>: Integer,
    Prod<Th, P>: Integer,
    Prod<N, P>: Integer,
    Prod<J, P>: Integer,
{
    type Output = Dimension<
        Prod<L, P>,
        Prod<M, P>,
        Prod<T, P>,
        Prod<I, P>,
        Prod<Th, P>,
        Prod<N, P>,
        Prod<J, P>,
        BaseSemantics,
    >;
}
