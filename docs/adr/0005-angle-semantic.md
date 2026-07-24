# ADR 0005: Distinguish Angles from Dimensionless Scalars

- Status: Accepted
- Date: 2026-07-24

## Context

Angles have no SI base exponent, but rotational and plane-wave APIs must not
accept an arbitrary dimensionless score or coefficient at an angle boundary.
Helios and Kwavers expose public angles in radians, so the provider needs one
zero-cost semantic contract for those consumers.

## Decision

Aequitas defines an `Angle` quantity with an `AngleSemantics` marker and the
coherent `Radian` unit. The marker is zero-sized; values remain canonical
radian scalars. Additive angle arithmetic preserves the angle semantic, while
dimensional multiplication and division normalize the result to the ordinary
dimension algebra.

## Alternatives

- Reuse `Dimensionless`: rejected because it permits unrelated scalar metrics at
  public angle boundaries.
- Store a runtime unit tag: rejected because the existing provider contract is
  compile-time and zero-cost.
- Add degree-specific APIs: rejected because radians are the canonical storage
  and trigonometric evaluation unit; callers can convert at their boundary.

## Verification

`tests/dimension_laws.rs` constructs a quarter turn through `Radian` and checks
the exact stored value. Consumer contract tests verify that Helios and Kwavers
angle fields use `Angle` rather than raw scalars.
