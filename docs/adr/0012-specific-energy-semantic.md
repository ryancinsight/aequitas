# ADR 0012: Semantic Specific Energy

## Status

Accepted — 2026-07-31

## Decision

Aequitas exposes `SpecificEnergy<T>` and `JoulePerKilogram` for mechanical
specific energy. Its dimension is the existing coherent `J/kg` axis shared by
`AbsorbedDose`, but the quantity alias carries the domain vocabulary required
by mechanical and turbulence consumers.

The alias remains generic over Eunomia scalar values. Real turbulence state
uses Eunomia's real scalar traits; complex values remain available to genuine
phasor and spectral formulas. Specific energy is a real magnitude and does not
introduce an imaginary physical unit.

## Rejected alternative

Using `AbsorbedDose` for turbulent kinetic energy would preserve the numerical
dimension but conflate radiation-dose and mechanical-energy contracts at public
call sites. Adding a new physical exponent would duplicate the same SI axis
and break dimensional arithmetic.

## Verification

Provider dimensional-law tests construct `SpecificEnergy` from `JoulePerKilogram`
and assert its canonical value and zero-sized dimension marker. CFDrs consumes
this vocabulary at the turbulence-state migration boundary.
