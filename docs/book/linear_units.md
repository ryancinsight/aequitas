# 4. Linear Units and Conversion

A linear unit is a zero-sized type implementing `LinearUnit<D>`, where `D` is
its dimension. The trait supplies a compile-time `SYMBOL` and `SCALE`; the
conversion law is simply `base = value × SCALE`.

```rust
extern crate aequitas;

use aequitas::systems::si::{quantities::Pressure, units::{Kilopascal, Pascal}};
use aequitas::unit::LinearUnit;

let pressure = Pressure::from_unit::<Kilopascal>(12.0_f64);
assert_eq!(pressure.in_unit::<Pascal>(), 12_000.0);
assert_eq!(Kilopascal::SYMBOL, "kPa");
```

`Quantity::from_unit::<U>` applies `U::to_base`, and `in_unit::<U>` applies
`U::from_base`. Both methods require the unit's dimension to match the
quantity, so a length unit cannot be selected for a pressure.

The unit trait is sealed. Only Aequitas-owned unit markers can implement it,
which keeps the positive finite scale invariant and the symbol vocabulary in
one provider. Adding a unit is therefore a provider change: define its
zero-sized marker, implement the private sealing trait, and provide the
correct dimension, `SCALE`, and `SYMBOL`.

`SCALE` is an associated constant, not runtime configuration. The operation is
monomorphized and a compiler can fold a constant conversion into the numeric
kernel. Symbols are `&'static str`, so reporting a unit needs no allocation.
Affine units such as Celsius require an offset and deliberately belong to a
future, separate contract rather than being forced into this linear API.
