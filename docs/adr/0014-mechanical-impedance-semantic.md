# ADR 0014: Name the SI mechanical-impedance quantity

## Status

Accepted — 2026-08-05.

## Context

Kwavers' mutual radiation impedance formula computes force per membrane
velocity. Its coherent SI unit is `kg/s`. That exponent vector is shared with
mechanical damping, but the public contracts describe different physical
roles. Labeling the result as `AcousticImpedance` is dimensionally incorrect:
acoustic impedance is pressure per particle velocity and has unit `kg/(m²·s)`.

The value is a Eunomia complex phasor. Its real and quadrature components are
components of one mechanical-impedance observation and do not require separate
or imaginary SI dimensions.

## Decision

Aequitas owns `MechanicalImpedance` with a dedicated semantic marker and the
coherent `KilogramPerSecond` unit. The quantity is generic over Eunomia scalar
types so `MechanicalImpedance<Complex64>` preserves provider-native phasors.
Consumers extract the scalar only inside numerical formulas and use the typed
quantity at public boundaries.

## Alternatives rejected

- Reuse `AcousticImpedance`: rejected because its SI exponent vector is
  `kg/(m²·s)`, not `kg/s`.
- Reuse `DampingCoefficient`: rejected because the exponent vector matches but
  the semantic role is different at the public API.
- Add an imaginary unit: rejected because a complex value's quadrature
  component retains the same observable mechanical-impedance unit.

## Verification

`dimension_laws` verifies complex conversion through `KilogramPerSecond` and
the downstream Kwavers crosstalk suite verifies the closed-form magnitude,
phase, reciprocity, inverse-distance scaling, zero diagonal, and degenerate
input behavior.
