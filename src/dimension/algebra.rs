use core::ops::{Add, Sub};
use typenum::{Diff, Integer, Sum};

use super::Dimension;

/// Type-level multiplication of physical dimensions.
///
/// Multiplication adds the corresponding SI base exponents.
#[diagnostic::on_unimplemented(
    message = "these Aequitas dimensions cannot be multiplied",
    note = "use Dimension<...> with Typenum integer exponents"
)]
pub trait MultiplyDimension<Rhs> {
    /// Dimension produced by multiplication.
    type Output;
}

/// Type-level division of physical dimensions.
///
/// Division subtracts the right-hand SI base exponents from the left-hand
/// exponents.
#[diagnostic::on_unimplemented(
    message = "these Aequitas dimensions cannot be divided",
    note = "use Dimension<...> with Typenum integer exponents"
)]
pub trait DivideDimension<Rhs> {
    /// Dimension produced by division.
    type Output;
}

impl<Ll, Ml, Tl, Il, Thl, Nl, Jl, Lr, Mr, Tr, Ir, Thr, Nr, Jr>
    MultiplyDimension<Dimension<Lr, Mr, Tr, Ir, Thr, Nr, Jr>>
    for Dimension<Ll, Ml, Tl, Il, Thl, Nl, Jl>
where
    Ll: Integer + Add<Lr>,
    Ml: Integer + Add<Mr>,
    Tl: Integer + Add<Tr>,
    Il: Integer + Add<Ir>,
    Thl: Integer + Add<Thr>,
    Nl: Integer + Add<Nr>,
    Jl: Integer + Add<Jr>,
    Lr: Integer,
    Mr: Integer,
    Tr: Integer,
    Ir: Integer,
    Thr: Integer,
    Nr: Integer,
    Jr: Integer,
    Sum<Ll, Lr>: Integer,
    Sum<Ml, Mr>: Integer,
    Sum<Tl, Tr>: Integer,
    Sum<Il, Ir>: Integer,
    Sum<Thl, Thr>: Integer,
    Sum<Nl, Nr>: Integer,
    Sum<Jl, Jr>: Integer,
{
    type Output = Dimension<
        Sum<Ll, Lr>,
        Sum<Ml, Mr>,
        Sum<Tl, Tr>,
        Sum<Il, Ir>,
        Sum<Thl, Thr>,
        Sum<Nl, Nr>,
        Sum<Jl, Jr>,
    >;
}

impl<Ll, Ml, Tl, Il, Thl, Nl, Jl, Lr, Mr, Tr, Ir, Thr, Nr, Jr>
    DivideDimension<Dimension<Lr, Mr, Tr, Ir, Thr, Nr, Jr>>
    for Dimension<Ll, Ml, Tl, Il, Thl, Nl, Jl>
where
    Ll: Integer + Sub<Lr>,
    Ml: Integer + Sub<Mr>,
    Tl: Integer + Sub<Tr>,
    Il: Integer + Sub<Ir>,
    Thl: Integer + Sub<Thr>,
    Nl: Integer + Sub<Nr>,
    Jl: Integer + Sub<Jr>,
    Lr: Integer,
    Mr: Integer,
    Tr: Integer,
    Ir: Integer,
    Thr: Integer,
    Nr: Integer,
    Jr: Integer,
    Diff<Ll, Lr>: Integer,
    Diff<Ml, Mr>: Integer,
    Diff<Tl, Tr>: Integer,
    Diff<Il, Ir>: Integer,
    Diff<Thl, Thr>: Integer,
    Diff<Nl, Nr>: Integer,
    Diff<Jl, Jr>: Integer,
{
    type Output = Dimension<
        Diff<Ll, Lr>,
        Diff<Ml, Mr>,
        Diff<Tl, Tr>,
        Diff<Il, Ir>,
        Diff<Thl, Thr>,
        Diff<Nl, Nr>,
        Diff<Jl, Jr>,
    >;
}
