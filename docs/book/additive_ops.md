# 6. Additive Operations

Additive arithmetic is only defined for compatible quantity meanings. For
ordinary base-semantic dimensions, `Add`, `Sub`, `AddAssign`, and `SubAssign`
operate on the same `Quantity<T, D>` type, and `Neg` changes only the scalar
sign.

```rust,ignore
use aequitas::systems::si::quantities::{Length, Time};

let mut distance = Length::from_base(3.0_f64);
distance += Length::from_base(2.0);
assert_eq!((distance - Length::from_base(1.0)).into_base(), 4.0);
assert_eq!((-distance).into_base(), -5.0);

let _elapsed = Time::from_base(2.0_f64);
```

The dimension parameter is part of the Rust type. A `Length` and a `Time` are
both represented by a scalar plus a marker, but they are different types, so
`length + time` is rejected before a program can run. The same rule prevents
adding a pressure to an energy merely because both happen to use `f64`.

Absolute temperature is the intentional exception to ordinary same-type
addition. A temperature difference can be added to an absolute temperature,
and subtracting two absolute temperatures produces a temperature difference.
Those implementations encode affine semantics instead of weakening the
compiler check.

The current API supports explicit pairwise additive operations. A future
collection-oriented extension may add `Iterator::sum` for quantities; until
then, fold or repeated addition keeps the accumulator type explicit.
