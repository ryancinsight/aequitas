# aequitas — Physical Quantities for the Atlas Stack

`aequitas` is the physical-quantity and dimensional-analysis layer of the
Atlas multiphysics stack.  It sits directly above `eunomia` (numeric scalars)
and below every domain crate that handles real physical values.

## Design goals

- **Zero overhead** — `Quantity<T, D>` is `#[repr(transparent)]`; the
  dimension `D` is a `PhantomData` ZST that occupies no storage and
  disappears after monomorphization.
- **Canonical SI storage** — values are always stored in the coherent SI base
  unit for their dimension; conversion happens at input and output, not inside
  formulas.
- **Type-level dimension safety** — multiplying a `Velocity` by a `MassDensity`
  produces an `AcousticImpedance`; adding a `Pressure` to a `Velocity` is a
  compile error.
- **No dynamic dispatch, no allocation** — the unit conversion factor is a
  compile-time constant materialized through the `LinearUnit` trait.

## What this book covers

1. The `Quantity<T, D>` model and why canonical SI storage matters.
2. The type-level dimension system built on `typenum` integer parameters.
3. Linear units, conversion, and the `LinearUnit` trait.
4. Derived quantities through quantity multiplication and division.
5. Where `aequitas` sits in the Atlas crate stack and how domain crates use it.
