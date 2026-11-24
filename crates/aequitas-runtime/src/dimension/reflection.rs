use super::{Exponent, RuntimeDimension};
use aequitas::dimension::Dimension;
use num_bigint::BigInt;
use num_rational::BigRational;
use typenum::{Bit, NInt, NonZero, PInt, UInt, UTerm, Unsigned, Z0};

// Read the full type-level integer. A fixed-width typenum projection could
// discard high bits before the runtime dimensional algebra ever sees them.
trait IntegerValue {
    fn value() -> BigInt;
}
impl IntegerValue for UTerm {
    fn value() -> BigInt {
        0.into()
    }
}
impl IntegerValue for Z0 {
    fn value() -> BigInt {
        0.into()
    }
}
impl<U: IntegerValue, B: Bit> IntegerValue for UInt<U, B> {
    fn value() -> BigInt {
        (U::value() << 1) + B::U8
    }
}
impl<U: IntegerValue + Unsigned + NonZero> IntegerValue for PInt<U> {
    fn value() -> BigInt {
        U::value()
    }
}
impl<U: IntegerValue + Unsigned + NonZero> IntegerValue for NInt<U> {
    fn value() -> BigInt {
        -U::value()
    }
}

/// Reflection of a static dimension's SI powers. Static semantic markers do
/// not participate in multiplicative dimensional algebra.
pub trait StaticDimension {
    /// Construct the exact runtime exponent vector.
    fn runtime_dimension() -> RuntimeDimension;
}

impl<L, M, T, I, K, N, J, S> StaticDimension for Dimension<L, M, T, I, K, N, J, S>
where
    L: IntegerValue,
    M: IntegerValue,
    T: IntegerValue,
    I: IntegerValue,
    K: IntegerValue,
    N: IntegerValue,
    J: IntegerValue,
{
    fn runtime_dimension() -> RuntimeDimension {
        RuntimeDimension::new(
            [
                L::value(),
                M::value(),
                T::value(),
                I::value(),
                K::value(),
                N::value(),
                J::value(),
            ]
            .map(|x| Exponent(BigRational::from_integer(x))),
        )
    }
}
