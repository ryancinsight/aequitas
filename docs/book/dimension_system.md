# 3. The Dimension Type System

Aequitas describes a physical dimension with seven SI exponents:
length, mass, time, electric current, thermodynamic temperature, amount of
substance, and luminous intensity. `Dimension<L, M, T, I, Θ, N, J, S>` stores
those typenum integers and an optional zero-sized semantic marker `S`.

For example, the length alias is equivalent to an exponent vector of
L¹M⁰T⁰I⁰Θ⁰N⁰J⁰. A velocity has L¹T⁻¹, while pressure has
L⁻¹M¹T⁻². The exponent types are checked during compilation, not stored or
validated at runtime.

Aequitas uses `typenum` so this type-level integer arithmetic works on stable
Rust without nightly const-generic arithmetic. `MultiplyDimension` adds the
corresponding exponent vectors and `DivideDimension` subtracts them. Quantity
`Mul` and `Div` use those associated output dimensions, so an operation can
produce a named result only when its dimensional algebra is valid.

```rust
extern crate aequitas;

use aequitas::systems::si::quantities::{MassDensity, Velocity, AcousticImpedance};

let density = MassDensity::from_base(1_000.0_f64);
let velocity = Velocity::from_base(1_500.0_f64);
let impedance: AcousticImpedance = density * velocity;
assert_eq!(impedance.into_base(), 1_500_000.0);
```

Exponent equality is not always semantic equality. Zero-sized markers keep
important meanings distinct: an angle, an absolute temperature, a temperature
difference, surface tension, spring stiffness, and mechanical impedance each
have their own vocabulary where needed. This prevents a value with the same
numeric exponent vector from silently crossing a domain boundary with a
different physical interpretation.

The semantic marker is normalized away for multiplication and division, while
named SI aliases reintroduce the intended vocabulary at public boundaries.
Thus the compiler checks both the algebraic exponents and the semantic types
without adding bytes to the stored scalar.
