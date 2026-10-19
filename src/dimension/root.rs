//! Type-level rational powers of physical dimensions.
//!
//! [`SqrtDimension`] halves every SI base exponent (the square root of a
//! squared quantity), and [`CbrtDimension`] thirds them (the cube root of a
//! cubed quantity). These let `Quantity::sqrt()` and `Quantity::cbrt()` carry
//! the correct half/third dimension at the type level — e.g. `sqrt(area)` is a
//! length and `cbrt(volume)` is a length. As in `uom` 0.38, the resulting
//! exponents remain integers; roots require exact divisibility.
//!
//! Both traits are implemented once, generically over every typenum integer
//! exponent. Each base exponent is divided through [`typenum::PartialDiv`],
//! which is defined only when the division leaves no remainder. A dimension
//! with any exponent that is odd (for `sqrt`) or not a multiple of three (for
//! `cbrt`) therefore has no impl, and trait resolution rejects it for every
//! shape, not only the shapes the SI inventory ships. Plain `typenum::Div` is
//! not used because it truncates: `P1 / P2` would be zero, which would admit
//! `sqrt(length)` silently.
//!
//! A rejected dimension is reported through typenum's `Rem` requirement, not
//! through the `on_unimplemented` message below. The generic impl matches
//! every shape by its head, so rustc reports the nested exponent obligation
//! and never the trait-level message. That message appears only when no impl
//! matches at all: a non-dimension type shows it on a direct trait bound
//! (E0277), but not through a method call, which reports only the unmet
//! `SqrtDimension` bound.
//!
//! The root output normalizes the semantic marker to `BaseSemantics`, matching
//! `MultiplyDimension` / `DivideDimension`. `Angle` has all exponents zero, so
//! it roots to dimensionless under both traits, matching the Python `Angle`
//! surface. `ReciprocalVolume` roots through `CbrtDimension` to a reciprocal
//! length.

use typenum::{Integer, P2, P3, PartialDiv};

use super::{BaseSemantics, Dimension};

/// Type-level square root of a physical dimension.
#[diagnostic::on_unimplemented(
    message = "this type has no even-exponent square root",
    note = "SqrtDimension requires every SI exponent to be divisible by two"
)]
pub trait SqrtDimension {
    /// Dimension produced by the square root.
    type Output;
}

/// Type-level cube root of a physical dimension.
#[diagnostic::on_unimplemented(
    message = "this type has no third-exponent cube root",
    note = "CbrtDimension requires every SI exponent to be divisible by three"
)]
pub trait CbrtDimension {
    /// Dimension produced by the cube root.
    type Output;
}

impl<L, M, T, I, Th, N, J, Semantics> SqrtDimension for Dimension<L, M, T, I, Th, N, J, Semantics>
where
    L: Integer + PartialDiv<P2>,
    M: Integer + PartialDiv<P2>,
    T: Integer + PartialDiv<P2>,
    I: Integer + PartialDiv<P2>,
    Th: Integer + PartialDiv<P2>,
    N: Integer + PartialDiv<P2>,
    J: Integer + PartialDiv<P2>,
    <L as PartialDiv<P2>>::Output: Integer,
    <M as PartialDiv<P2>>::Output: Integer,
    <T as PartialDiv<P2>>::Output: Integer,
    <I as PartialDiv<P2>>::Output: Integer,
    <Th as PartialDiv<P2>>::Output: Integer,
    <N as PartialDiv<P2>>::Output: Integer,
    <J as PartialDiv<P2>>::Output: Integer,
{
    type Output = Dimension<
        <L as PartialDiv<P2>>::Output,
        <M as PartialDiv<P2>>::Output,
        <T as PartialDiv<P2>>::Output,
        <I as PartialDiv<P2>>::Output,
        <Th as PartialDiv<P2>>::Output,
        <N as PartialDiv<P2>>::Output,
        <J as PartialDiv<P2>>::Output,
        BaseSemantics,
    >;
}

impl<L, M, T, I, Th, N, J, Semantics> CbrtDimension for Dimension<L, M, T, I, Th, N, J, Semantics>
where
    L: Integer + PartialDiv<P3>,
    M: Integer + PartialDiv<P3>,
    T: Integer + PartialDiv<P3>,
    I: Integer + PartialDiv<P3>,
    Th: Integer + PartialDiv<P3>,
    N: Integer + PartialDiv<P3>,
    J: Integer + PartialDiv<P3>,
    <L as PartialDiv<P3>>::Output: Integer,
    <M as PartialDiv<P3>>::Output: Integer,
    <T as PartialDiv<P3>>::Output: Integer,
    <I as PartialDiv<P3>>::Output: Integer,
    <Th as PartialDiv<P3>>::Output: Integer,
    <N as PartialDiv<P3>>::Output: Integer,
    <J as PartialDiv<P3>>::Output: Integer,
{
    type Output = Dimension<
        <L as PartialDiv<P3>>::Output,
        <M as PartialDiv<P3>>::Output,
        <T as PartialDiv<P3>>::Output,
        <I as PartialDiv<P3>>::Output,
        <Th as PartialDiv<P3>>::Output,
        <N as PartialDiv<P3>>::Output,
        <J as PartialDiv<P3>>::Output,
        BaseSemantics,
    >;
}
