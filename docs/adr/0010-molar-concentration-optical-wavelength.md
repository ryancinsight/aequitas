# ADR 0010: Add molar concentration and optical wavelength units

- **Status:** Proposed
- **Date:** 2026-07-31
- **Driver:** Kwavers `KWAVERS-AEQ-MET-43`

## Context

Kwavers' blood-oxygenation workflow exposes hemoglobin concentration in molar
units and optical wavelengths in nanometres. Aequitas currently provides
amount of substance, volume, and length, but no semantically distinct molar
concentration or nanometre unit. Reusing `NumberDensity` would conflate entity
counts with amount of substance, while consumer-local wrappers would duplicate
provider-owned dimensional vocabulary.

## Decision

Add a semantic `MolarConcentration` dimension for amount of substance per
volume, with coherent `MolePerCubicMeter` and scaled `MicromolePerLiter` units.
Add the scaled `Nanometer` length unit. Both remain zero-cost aliases over
Eunomia scalar storage. Molar concentration and optical wavelength are
real-valued physical quantities; no imaginary or complex physical unit is
introduced. Complex values remain at existing Eunomia formula/storage
boundaries.

## Alternatives

- Reuse `NumberDensity`: rejected because molar amount and entity count have
  different domain semantics despite sharing inverse-volume exponents.
- Keep raw nanometre and molar scalars: rejected because unit conversion and
  dimensional intent remain implicit at public contracts.
- Add consumer-local wrappers: rejected because Aequitas is the provider-owned
  quantity SSOT.

## Verification plan

- Add unit-level conversion and layout regressions for both units and the
  semantic distinction from `NumberDensity`.
- Run Aequitas format, warning-denied Clippy, Nextest, doctests, and RustDoc.
- Migrate Kwavers blood-oxygenation contracts and verify real-valued
  Eunomia-compatible behavior at the diagnostics boundary.

