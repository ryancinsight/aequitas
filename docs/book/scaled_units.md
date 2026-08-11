# 5. Scaled Units

Scaled units are ordinary Aequitas linear units whose `SCALE` expresses a
prefix or an exact relationship to an SI base unit. The scale always points
from the named unit to the canonical stored value.

```rust,ignore
use aequitas::systems::si::{quantities::{Length, Frequency}, units::{Millimeter, Megahertz}};

assert_eq!(Length::from_unit::<Millimeter>(2.5).into_base(), 2.5e-3);
assert_eq!(Frequency::from_unit::<Megahertz>(3.0).into_base(), 3.0e6);
```

The provider keeps these markers in `systems::si::units::scaled`. Examples
include millimetres, nanometres, kilometres, milliseconds, kilopascals,
megahertz, and electronvolts. An electronvolt is not an SI base unit, but its
exact joule relationship can still be represented by a constant scale.

A new provider-owned scaled unit needs only a zero-sized marker, a sealed
implementation, and a `LinearUnit<D>` implementation with the right `SCALE`
and `SYMBOL`. External callers cannot add implementations because the trait is
sealed; this is intentional, since an incorrect scale would corrupt every
quantity constructed with that unit.

Different names can share numerical exponents without being interchangeable
semantically. For example, `GrayPerSecond` and `WattPerKilogram` describe the
same coherent SI axis while serving radiation-dose and power-deposition
vocabularies. Aequitas preserves those names at the API boundary even though
the stored base scalar and conversion factor are the same.
