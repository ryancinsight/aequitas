# ADR 0015: Name MEMS charge-density and flexural-rigidity quantities

## Status

Accepted — 2026-08-05.

## Context

Kwavers' PMUT and shared plate models expose piezoelectric charge density and
flexural rigidity at typed Rust boundaries. The existing formulas use
`|e₃₁,f|/t_p` in `C/m³` and `E h³/(12(1−ν²))` in `J`; generic pressure or
dimensionless labels would hide those contracts.

The values are real for the current MEMS models. Aequitas quantities remain
generic over Eunomia scalar types so a genuine complex phasor preserves one
observable SI unit for its real and quadrature components; no imaginary unit is
introduced.

## Decision

Aequitas owns `VolumeChargeDensity` with `CoulombPerCubicMeter` and
`FlexuralRigidity` with `Joule`. Kwavers uses those quantities at the PMUT and
shared plate boundaries and extracts scalars only inside formulas, assertions,
or the Python serialization boundary.

## Alternatives rejected

- Reuse `SurfaceChargeDensity`: rejected because dividing the piezoelectric
  surface coefficient by film thickness produces `C/m³`.
- Reuse `Energy`: rejected because flexural rigidity has a distinct mechanical
  role even though its coherent SI unit is `J`.
- Add an imaginary SI dimension: rejected because complex quadrature is part of
  one physical observation and retains its unit.

## Verification

`dimension_laws` verifies real and complex conversion through both coherent
units. Kwavers' MEMS tests verify the typed CMUT, PMUT, plate, comparison, and
Python boundary contracts.
