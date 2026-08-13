//! Type-level rational powers of physical dimensions.
//!
//! [`SqrtDimension`] halves every SI base exponent (the square root of a
//! squared quantity), and [`CbrtDimension`] thirds them (the cube root of a
//! cubed quantity). These let `Quantity::sqrt()` and `Quantity::cbrt()` carry
//! the correct half/third dimension at the type level — e.g. `sqrt(area)` is a
//! length and `cbrt(volume)` is a length. The general `uom` crate cannot
//! express fractional dimensions, so this is a strict capability extension.
//!
//! Implementations are generated for the concrete exponent combinations used
//! by the SI dimension inventory. A generic impl over arbitrary typenum
//! integers would need an explicit `Div` bound for the `Quot` projection
//! (typenum `PartialDiv` is exact-only but does not expose `Div`), so the
//! tuples enumerate the shipped exponent shapes explicitly — all exponents
//! even for `SqrtDimension`, divisible by three for `CbrtDimension` — to
//! keep the trait-resolution surface small.

use typenum::{N2, N3, N4, P2, P3, Quot, Z0};

use super::{BaseSemantics, Dimension};

/// Type-level square root of a physical dimension.
#[diagnostic::on_unimplemented(
    message = "this Aequitas dimension has no even-exponent square root",
    note = "SqrtDimension requires every SI exponent to be divisible by two"
)]
pub trait SqrtDimension {
    /// Dimension produced by the square root.
    type Output;
}

/// Type-level cube root of a physical dimension.
#[diagnostic::on_unimplemented(
    message = "this Aequitas dimension has no third-exponent cube root",
    note = "CbrtDimension requires every SI exponent to be divisible by three"
)]
pub trait CbrtDimension {
    /// Dimension produced by the cube root.
    type Output;
}

/// Square root of a single concrete type-level integer exponent.
type SqrtExp<Exp> = Quot<Exp, P2>;
/// Cube root of a single concrete type-level integer exponent.
type CbrtExp<Exp> = Quot<Exp, P3>;

macro_rules! impl_sqrt {
    ($(($l:ty, $m:ty, $t:ty, $i:ty, $th:ty, $n:ty, $j:ty)),* $(,)?) => {
        $(
            impl SqrtDimension for Dimension<$l, $m, $t, $i, $th, $n, $j, BaseSemantics> {
                type Output = Dimension<
                    SqrtExp<$l>,
                    SqrtExp<$m>,
                    SqrtExp<$t>,
                    SqrtExp<$i>,
                    SqrtExp<$th>,
                    SqrtExp<$n>,
                    SqrtExp<$j>,
                    BaseSemantics,
                >;
            }
        )*
    };
}

macro_rules! impl_cbrt {
    ($(($l:ty, $m:ty, $t:ty, $i:ty, $th:ty, $n:ty, $j:ty)),* $(,)?) => {
        $(
            impl CbrtDimension for Dimension<$l, $m, $t, $i, $th, $n, $j, BaseSemantics> {
                type Output = Dimension<
                    CbrtExp<$l>,
                    CbrtExp<$m>,
                    CbrtExp<$t>,
                    CbrtExp<$i>,
                    CbrtExp<$th>,
                    CbrtExp<$n>,
                    CbrtExp<$j>,
                    BaseSemantics,
                >;
            }
        )*
    };
}

// Concrete dimension exponent tuples that support square roots (all exponents
// even). Z0 halves to Z0; P2/N2 to P1/N1; P4/N4 to P2/N2.
impl_sqrt!(
    (P2, Z0, Z0, Z0, Z0, Z0, Z0), // area -> length
    (Z0, P2, Z0, Z0, Z0, Z0, Z0),
    (P2, Z0, N2, Z0, Z0, Z0, Z0), // energy/mass (speed-squared) -> velocity
    (P2, Z0, N4, Z0, Z0, Z0, Z0), // acceleration-squared -> acceleration
    (Z0, Z0, N2, Z0, Z0, Z0, Z0), // reciprocal-time-squared -> reciprocal-time
    (Z0, Z0, N4, Z0, Z0, Z0, Z0),
    (Z0, Z0, Z0, Z0, N2, Z0, Z0), // reciprocal-temperature-squared -> reciprocal-temperature
    (Z0, Z0, Z0, Z0, Z0, Z0, Z0), // dimensionless
);

// Concrete dimension exponent tuples that support cube roots (all exponents
// divisible by three).
impl_cbrt!(
    (P3, Z0, Z0, Z0, Z0, Z0, Z0), // volume -> length
    (N3, Z0, Z0, Z0, Z0, Z0, Z0), // number-density -> reciprocal-length
    (Z0, Z0, Z0, Z0, Z0, Z0, Z0), // dimensionless
);
