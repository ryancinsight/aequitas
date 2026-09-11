# 4. Linear Units and Conversion

A linear unit is a zero-sized type implementing `LinearUnit<D>`, where `D` is
its dimension. The trait supplies a compile-time `SYMBOL` and `SCALE`; the
conversion law is simply `base = value × SCALE`.

```rust
extern crate aequitas;

use aequitas::systems::si::{dimensions, quantities::Pressure, units::{Kilopascal, Pascal}};
use aequitas::unit::LinearUnit;

let pressure = Pressure::from_unit::<Kilopascal>(12.0_f64);
assert_eq!(pressure.in_unit::<Pascal>(), 12_000.0);
assert_eq!(<Kilopascal as LinearUnit<dimensions::Pressure>>::SYMBOL, "kPa");
```

`Quantity::from_unit::<U>` applies `U::to_base`, and `in_unit::<U>` applies
`U::from_base`. Both methods require the unit's dimension to match the
quantity, so a length unit cannot be selected for a pressure.

A unit can serve more than one dimension: the kilopascal is both a pressure
and a stress. Its symbol is therefore named through the dimension it is read
for, as above, because a bare `Kilopascal::SYMBOL` cannot say which
implementation it means and does not compile.

The unit trait is sealed. Only Aequitas-owned unit markers can implement it,
which keeps the positive finite scale invariant and the symbol vocabulary in
one provider. Adding a unit is therefore a provider change: define its
zero-sized marker, implement the private sealing trait, and provide the
correct dimension, `SCALE`, and `SYMBOL`.

`SCALE` is an associated constant, not runtime configuration. The operation is
monomorphized and a compiler can fold a constant conversion into the numeric
kernel. Symbols are `&'static str`, so reporting a unit needs no allocation.
Affine units such as Celsius need an offset, so they have their own contract,
`AffineUnit`, rather than being forced into this linear API: `base = value *
SCALE + OFFSET`. A degree Celsius is affine for `ThermodynamicTemperature` and
linear for `TemperatureDifference`. The same marker implements both, and the
quantity's dimension selects the conversion, so a temperature difference never
picks up the 273.15 K offset that belongs only to a temperature.
