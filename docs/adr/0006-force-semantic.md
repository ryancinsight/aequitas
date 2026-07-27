# ADR 0006: Name the SI Force Quantity

- Status: Accepted
- Date: 2026-07-27

## Context

CFDrs cell-separation equilibrium results expose lateral position and force
balance values at a public boundary. Aequitas already derives force through
pressure multiplied by area, but the dimension has no named quantity or
Newton unit. Leaving those results as `f64` loses the contract at the exact
boundary where callers interpret the values.

## Decision

Aequitas defines the `Force` quantity and coherent `Newton` unit. The quantity
uses the existing SI dimensional algebra (`kg·m/s²`) and remains transparent
over the Eunomia scalar. Consumers convert to a scalar only inside numerical
formulas or explicit serialization boundaries.

## Alternatives

- Reuse `Pressure * Area` at every consumer boundary: rejected because it
  leaves named result fields without a stable semantic alias.
- Add a consumer-local force wrapper: rejected because Aequitas owns the
  shared SI dimension and conversion law.
- Store a runtime unit tag: rejected because Aequitas dimensions are compile-
  time and zero-cost.

## Verification

`tests/dimension_laws.rs` proves pressure-area composition and exact Newton
conversion; `tests/layout.rs` proves the marker and unit remain zero-sized.
CFDrs consumes the named quantity in its cell-separation equilibrium result.
