# Aequitas runtime dimensions

Use `RuntimeQuantity` when a scalar or homogeneous buffer acquires its SI
dimension at runtime. A dimension stores seven exact rational powers in
length, mass, time, electric current, temperature, amount of substance and
luminous intensity order. The crate supports `no_std` with an allocator.

```rust
use aequitas::systems::si::{dimensions, quantities::Pressure};
use aequitas_runtime::{Exponent, RuntimeDimension, RuntimeQuantity};

let p = RuntimeQuantity::from(Pressure::from_base([3.0, 4.0]));
let time = RuntimeDimension::of::<dimensions::Time>();
let half = Exponent::ratio(1, 2).expect("nonzero denominator");
let derivative_units = p.dimension().quotient(&time.power(&half));
assert_eq!(derivative_units.axes()[2].to_string(), "-5/2");
```

`Exponent::ratio(1, 3)` constructs an exact third. `Exponent::from_scalar`
captures the exact binary value supplied by a numerical algorithm. Exponents
are shared by every element of a quantity buffer and remain exact through
products, quotients and powers. Static quantity reflection preserves all
seven SI axes. Quantities with distinct meanings can share those axes.
