# 7. Derived Quantities

Derived quantities are produced by the same type-level algebra as their
physical equations. Multiplying `Quantity<T, LhsDimension>` by
`Quantity<T, RhsDimension>` uses `MultiplyDimension`; division uses
`DivideDimension`. The output scalar is multiplied or divided in the usual
way, while the compiler computes the output dimension.

```rust,ignore
use aequitas::systems::si::{
    quantities::{AcousticImpedance, MassDensity, Velocity},
    units::{KilogramPerCubicMeter, MeterPerSecond},
};

let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
let velocity = Velocity::from_unit::<MeterPerSecond>(1_500.0);
let impedance: AcousticImpedance = density * velocity;
assert_eq!(impedance.into_base(), 1_500_000.0);
```

The equation is `kg/m³ × m/s = kg/(m²·s)`, the Rayl-like acoustic
impedance dimension. Because SI exponent vectors are closed under addition
and subtraction, every valid product or quotient remains representable as a
new Aequitas dimension. A named alias documents the result without a runtime
tag.

Scalar multiplication is separate: multiplying a quantity by its scalar type
changes only the stored value and keeps its dimension. This is useful for
coefficients and nondimensional factors. The practical rule is to convert
inputs with `from_unit`, perform the formula in canonical base units, and
convert the result with `in_unit` only at the reporting or serialization
boundary.
