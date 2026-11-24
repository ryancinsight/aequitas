use core::marker::PhantomData;

use crate::unit::{
    AffineUnit, LinearUnit, UnitDimension,
    decimal::product_scale,
    expression::checked_scale,
    private::{Named, Sealed},
};

use super::{Prefix, symbol::Symbol};

/// A single prefix `P` applied to an eligible unit token `U`.
///
/// Scale is the prefix factor times the base unit's scale; when they are
/// decades `10^a` and `10^b` with `0 < |a|, |b| <= 30` and `|a+b| <= 22` it is
/// the nearest binary64 to `10^(a+b)`. In compound expressions the entire
/// prefixed token is raised to a power. Celsius retains its affine origin when
/// prefixed as an absolute temperature.
///
/// ```
/// use aequitas::{systems::si::{quantities::Length, units::Meter},
///     unit::{Prefixed, prefixes::Quecto}};
/// assert_eq!(Length::from_unit::<Prefixed<Quecto, Meter>>(2.0).into_base(), 2e-30);
/// ```
///
/// The mass root is gram:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Mass, units::Kilogram},
///     unit::{Prefixed, prefixes::Milli}};
/// let value = Mass::from_unit::<Prefixed<Milli, Kilogram>>(1.0_f64);
/// ```
///
/// A prefix cannot attach to another prefixed token, to unit one or to a
/// compound expression:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Length, units::Meter},
///     unit::{Prefixed, prefixes::{Milli, Micro}}};
/// let value = Length::from_unit::<Prefixed<Milli, Prefixed<Micro, Meter>>>(1.0_f64);
/// ```
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Dimensionless, units::One},
///     unit::{Prefixed, prefixes::Milli}};
/// let value = Dimensionless::from_unit::<Prefixed<Milli, One>>(1.0_f64);
/// ```
///
/// Binary prefixes apply to information units:
///
/// ```compile_fail
/// use aequitas::{systems::si::{quantities::Length, units::Meter},
///     unit::{Prefixed, prefixes::Kibi}};
/// let value = Length::from_unit::<Prefixed<Kibi, Meter>>(1.0_f64);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Prefixed<P, U>(PhantomData<(P, U)>);

impl<P: Prefix<U>, U: Named> Prefixed<P, U> {
    const SYMBOL: Symbol = Symbol::new(P::SYMBOL, P::ROOT_SYMBOL);
}

impl<P: Prefix<U>, U: Named> Sealed for Prefixed<P, U> {}
impl<P: Prefix<U>, U: Named> Named for Prefixed<P, U> {}

impl<P: Prefix<U>, U: Named> UnitDimension for Prefixed<P, U> {
    type Dimension = U::Dimension;
}

impl<D, P: Prefix<U>, U: LinearUnit<D>> LinearUnit<D> for Prefixed<P, U> {
    const SYMBOL: &'static str = Self::SYMBOL.as_str();
    const SCALE: f64 = checked_scale(product_scale(U::SCALE, P::FACTOR));
}

impl<D, P: Prefix<U>, U: Named + AffineUnit<D>> AffineUnit<D> for Prefixed<P, U> {
    const SYMBOL: &'static str = Self::SYMBOL.as_str();
    const SCALE: f64 = checked_scale(product_scale(U::SCALE, P::FACTOR));
    const OFFSET: f64 = U::OFFSET;
}
