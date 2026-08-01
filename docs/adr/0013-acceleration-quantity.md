# ADR 0013: Add SI acceleration and pressure-rate quantities

Status: Accepted — 2026-08-01

## Context

Kwavers' public therapeutic microbubble state carries wall acceleration in
metres per second squared. Keeping that value as a raw scalar loses the
physical contract at the Aequitas boundary. Existing velocity and time
quantities compose to the required dimension, but the named acceleration
surface is absent. The same consumer also exposes the rate of change of
acoustic pressure, which requires a named pressure-rate dimension.

## Decision

Aequitas owns `Acceleration<T>`, `PressureRate<T>`,
`MeterPerSecondSquared`, and `PascalPerSecond`. The acceleration dimension is
the type-level product `Velocity / Time`; pressure rate composes as
`Pressure / Time`; both coherent unit scales are one. Consumers use both
quantities at public physical boundaries and extract a scalar only at a
numerical formula or storage boundary.

Eunomia remains the scalar-representation owner. `Complex<T>` remains valid
when a consumer models a genuine complex phasor with an existing physical
dimension; acceleration itself does not create an imaginary physical unit.

## Alternatives

- Keep wall acceleration as `f64`: rejected because it leaves a public SI
  metric untyped.
- Add a consumer-local acceleration wrapper: rejected because the SI dimension
  is shared provider vocabulary and would duplicate Aequitas ownership.
- Represent acceleration as a new imaginary unit: rejected because the
  imaginary component is representation data, not a physical dimension.

## Verification

The dimensional-law test proves `Velocity / Time = Acceleration` with the
coherent `m/s²` unit, and `Pressure / Time = PressureRate` with the coherent
`Pa/s` unit and exact value semantics.
