# ADR 0011: Electrical Conductivity for SAR Contracts

## Status

Accepted — 2026-07-31

## Decision

Aequitas owns `ElectricalConductivity` with coherent SI unit `SiemensPerMeter`
(`S/m`). Its dimension is conductance per length:

```text
σ · |E|² / ρ = (S/m) · (V/m)² / (kg/m³) = W/kg
```

The quantity is generic over Eunomia scalar values, so a complex conductivity
or field phasor can cross the provider boundary when a model requires it.
Specific absorption rate is the real power-dissipation magnitude; it does not
introduce an imaginary physical unit. Consumers apply the Hermitian magnitude
at their numerical field boundary before constructing the real SAR quantity.

## Rejected alternative

Keeping conductivity as an untyped scalar would leave the RF deposition law
outside the provider-owned dimensional algebra and allow `S/m` to be confused
with conductance `S`.

## Verification

The provider dimensional-law test constructs `σ·E²/ρ` from typed quantities and
asserts its coherent `W/kg` value. Consumer tests must additionally verify the
spatial `σ·|E|²/ρ` law and the real-valued Eunomia boundary.
