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

`Quantity::from_unit::<U>` multiplies by `Unit<D>::SCALE`, and `in_unit::<U>`
divides by it in the scalar's native precision. Both methods require the unit's dimension to match the
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

## Composing units

`Unit<D>` is the shared conversion and formatting contract. Named
`LinearUnit<D>` markers implement it automatically. `Product<A, B>`,
`Quotient<A, B>` and `Power<U, P>` implement it by adding, subtracting or
multiplying dimension exponents and composing their scale factors. `P` is
a Typenum integer: `P2` squares, `N1` inverts and `Z0` yields dimensionless.

```rust
use aequitas::{Quantity, systems::si::{quantities::Intensity,
    units::{Watt, Meter, Centimeter, WattPerSquareMeter}},
    unit::{Product, Quotient, Power}};

let intensity = Intensity::from_unit::<Quotient<Watt, Product<Meter, Meter>>>(12.0_f64);
assert_eq!(intensity, Intensity::from_unit::<WattPerSquareMeter>(12.0));
let scaled = Intensity::from_unit::<Quotient<Watt, Power<Centimeter, typenum::P2>>>(12.0);
assert_eq!(scaled.in_unit::<WattPerSquareMeter>(), 120_000.0);

// No named quantity or named compound unit is required.
let unnamed = Quantity::from_unit::<Product<Power<Meter, typenum::P2>, Watt>>(3.0_f64);
assert_eq!(unnamed.into_base(), 3.0);
```

Equivalent dimensions produce compatible quantities even when expressions
have different grouping. Their marker types remain distinct. Scale conversion
uses `UnitScalar`, including real and complex scalars. Composed scales must
fit positive finite f64 metadata; expressions outside
that range fail during const evaluation rather than silently producing invalid
scales. Power exponents must fit a signed 32-bit integer and are checked by
type-level bounds.

Dimension normalization is exact type-level integer arithmetic. Scale factors
use f64 metadata arithmetic, so noncoherent expressions can differ by rounding
when regrouped. The coefficient is then rounded to the selected scalar's
precision; its representable range and IEEE special-value behavior still apply.

`UnitDimension` supplies each marker's dimension for composition. Multiple
semantic readings of a named unit share its exponents. Compound operations
normalize semantic markers just as quantity arithmetic does; they cannot
implicitly construct distinct semantic quantities such as stress or absolute
temperature. Celsius and Fahrenheit in compound expressions mean temperature
intervals, with no offset. Absolute temperatures still use `from_affine_unit`.

`UnitDisplay` writes a parenthesized expression without allocating, such as
`12 (W/(m*m))`. Use `WattPerSquareMeter` with `UnitDisplay` when the named
spelling is desired. Formatting does not search the catalog or change scales.
