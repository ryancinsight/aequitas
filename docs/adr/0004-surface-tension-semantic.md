# ADR 0004: Distinguish Surface Tension from Energy per Area

- Status: Accepted
- Date: 2026-07-24

## Context

Surface tension has SI base exponents `kg·s⁻²`, the same exponents as energy
per area. Atlas cavitation and interfacial models use the value as force per
length (`N/m`), so exposing only the exponent vector allows a caller to pass
an areal-energy value at a semantically wrong boundary.

## Decision

Aequitas defines a `SurfaceTension` quantity with a dedicated semantic marker
and the coherent `NewtonPerMeter` unit. The marker is zero-sized and does not
alter the stored scalar or dimensional multiplication and division results.
Arithmetic that produces a surface tension remains explicit at the boundary;
the provider does not add an implicit conversion from `EnergyPerArea`.

## Alternatives

- Alias `SurfaceTension` to `EnergyPerArea`: rejected because it preserves the
  semantic confusion at typed consumer boundaries.
- Add a runtime unit tag: rejected because Aequitas dimensions are compile-time
  and zero-cost; runtime metadata would weaken the existing representation.

## Verification

`tests/dimension_laws.rs` constructs surface tension in `N/m`, divides by a
length, and requires the resulting type to be `Pressure`. The test also
checks the exact binary value of the unit conversion. The full Aequitas native
suite and doctests run for this change.
