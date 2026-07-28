# ADR 0009: Represent Complex Physical Phasors With Eunomia

- Status: Accepted
- Date: 2026-07-28
- Class: [minor]

## Context

Kwavers exposes complex pressure phasors in pascals and complex electrical
impedance in ohms. Eunomia owns the `Complex<T>` representation and its
`ComplexField` operations, but Aequitas previously limited unit conversion to
`FloatElement`, so those public contracts could not carry their physical units
without a consumer-local wrapper. Eunomia now owns the `UnitScalar` conversion
seam used by both real storage types and complex phasors.

The imaginary component is quadrature data for the same physical quantity. It
is not an additional SI dimension and does not receive a separate unit marker.
CFDrs and Helios currently use complex values only inside numerical formulas or
do not expose complex contracts, respectively.

## Decision

Aequitas supports `Quantity<T, D>` for every Eunomia `UnitScalar` implementation,
including `Complex32` and `Complex64`. Linear unit conversion scales the real
and imaginary components by the same real coefficient through Eunomia's
provider-owned `UnitScalar::scale_by_f64` operation. The existing
`FloatElement` arithmetic path remains unchanged for real quantities and
reduced-precision storage types.

Aequitas adds `ElectricalImpedance` and its coherent `Ohm` unit. Consumers use
that dimension for complex electrical impedance; acoustic impedance remains a
separate dimension.

## Rejected alternatives

- Add a consumer-local complex-pressure or complex-impedance wrapper: rejected
  because Eunomia already owns the complex representation and Aequitas owns
  dimensional semantics.
- Model the imaginary component as a separate unit or dimension: rejected
  because it represents phase/quadrature of the same dimensional observable.
- Widen all Aequitas storage bounds to `ComplexField`: rejected because that
  field seam does not cover Eunomia's reduced-precision real storage types;
  `UnitScalar` preserves the complete shipped storage set without an
  overlapping real/complex implementation.

## Verification

- Aequitas tests round-trip a scaled complex length and derive complex
  electrical impedance from complex potential/current quantities.
- Kwavers tests retain the complex value semantics while the public fields
  carry `Pressure<Complex64>` and `ElectricalImpedance<Complex64>`.
- CFDrs and Helios audits record no public complex-unit gap.
