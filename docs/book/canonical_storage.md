# 2. Canonical SI Storage

Aequitas stores every quantity in the coherent SI base unit for its dimension.
A `Length` contains metres, a `Pressure` contains pascals, and a
`MassDensity` contains kilograms per cubic metre. The representation never
carries a runtime unit tag or a hidden scale factor.

```rust,ignore
use aequitas::systems::si::{quantities::Length, units::{Meter, Kilometer}};

let distance = Length::from_unit::<Kilometer>(1.25_f64);
assert_eq!(distance.into_base(), 1_250.0);
assert_eq!(Length::from_unit::<Meter>(1_250.0).in_unit::<Kilometer>(), 1.25);
```

`from_unit` performs the conversion once at the input boundary. `in_unit`
converts a stored base value for presentation or serialization. Arithmetic
therefore has one unambiguous representation: a solver can multiply and divide
base values without carrying unit-specific branches through its hot path.

The conversion is expressed through Eunomia's `UnitScalar::scale_by_f64`
seam. It works for the provider's supported real and complex scalar types and
is monomorphized with the quantity. For compile-time unit constants, ordinary
optimization can fold the scale multiplication; there is no dynamic dispatch
or allocation.

The quantity fields are private. A caller can construct a value only with
`from_base` or `from_unit`, and both constructors establish the same invariant:
the stored value is already in base units. Extract the scalar explicitly with
`as_base` or `into_base` when crossing into a numerical kernel or storage layer.
