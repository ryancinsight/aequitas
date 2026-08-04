# Example: Acoustic Impedance

**Crate**: `aequitas`
**Source**: `examples/book_acoustic_impedance.rs`

Characteristic acoustic impedance Z₀ = ρ × c (Rayl = kg m⁻² s⁻¹) is the
fundamental tissue parameter in ultrasound imaging.  The reflection coefficient
at a normal-incidence interface is R = (Z₂ − Z₁)/(Z₂ + Z₁); the intensity
reflection coefficient is R².

This example shows that multiplying a `MassDensity<f64>` by a `Velocity<f64>`
*statically* produces an `AcousticImpedance<f64>` — the dimension algebra is
enforced at compile time, not at runtime.

## Source

```rust
{{#include ../../../examples/book_acoustic_impedance.rs}}
```

## Output

```text
Medium                  ρ (g/cm³)      c (m/s)     Z₀ (MRayl)
--------------------------------------------------------------
water (20 °C)               0.998       1482.0          1.479
soft tissue                 1.050       1540.0          1.617
blood                       1.060       1570.0          1.664
bone (cortical)             1.900       3500.0          6.650

water→tissue normal-incidence intensity reflection: 0.0020 (0.20 %)
```

## What to notice

- The `characteristic_impedance` function signature is `(MassDensity<f64>,
  Velocity<f64>) -> AcousticImpedance<f64>`.  The return type is not
  declared explicitly inside the function body — it is inferred from the
  `Mul` impl whose `Output` associated type resolves
  `dimensions::MassDensity × dimensions::Velocity` to
  `dimensions::AcousticImpedance` through the typenum exponent algebra.

- Passing a `Pressure` where `MassDensity` is expected is a compile error,
  even though both wrap `f64`.  This is the primary safety guarantee.

- The reflection calculation operates on plain `f64` values obtained via
  `in_unit::<Rayl>()`.  Subtraction and division of raw scalars do not need
  the dimensional system once the values are at the same boundary.

- Water→soft-tissue intensity reflection is 0.20 %, consistent with
  published values (~0.1–0.5 %).  The `assert!` at the end locks this in as
  a regression guard.

- Bone's impedance (6.65 MRayl) vs water's (1.48 MRayl) would give a
  reflection coefficient of roughly 63 %, explaining why bone produces
  strong acoustic shadowing in clinical imaging.
