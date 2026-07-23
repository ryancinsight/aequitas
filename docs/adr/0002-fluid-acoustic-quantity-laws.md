# ADR 0002: Add fluid and acoustic transport quantity laws

- Status: Accepted
- Date: 2026-07-23
- Class: [arch] [minor]

## Context

CFDrs computes kinematic viscosity, Reynolds inputs, and inlet velocity from
raw scalar material and flow values. Kwavers computes acoustic heating,
streaming velocity, and streaming power from raw attenuation, intensity,
density, and sound-speed values. These equations have stable SI contracts but
the current Aequitas vocabulary does not represent all of their result
dimensions. Helios already has the required energy-per-area quantity for its
portal fluence boundary but does not retain it through transmission.

## Decision

Extend Aequitas with one coherent fluid/acoustic quantity family:

- dynamic viscosity and kinematic viscosity;
- volumetric flow rate;
- acoustic impedance;
- intensity (power per area); and
- volumetric power density.

Use canonical SI unit markers, with millipascal-second as the one scaled
viscosity unit required by consumer material data. Consumer crates retain raw
arrays, serialization formats, and domain validation at their existing
boundaries. They use Aequitas quantities inside equations and convert back
only at those representation boundaries.

## Rationale

This is a lower common owner for dimensions and unit conversion, not a new
fluid or acoustic model. CFDrs retains solver policy and fluid models; Kwavers
retains acoustic constitutive and therapy models; Helios retains dose and
delivery policy. Typed equations prevent mixing flow, viscosity, density,
attenuation, and intensity without changing storage layout or adding runtime
metadata.

## Verification

- Aequitas dimension-law tests prove the fluid and acoustic compositions.
- Aequitas property tests cover millipascal-second round trips.
- CFDrs tests exercise typed kinematic viscosity, Reynolds, and cascade inlet
  velocity calculations through their existing public scalar contracts.
- Helios portal tests preserve energy-per-area transmission values.
- Kwavers thermal-coupling tests preserve acoustic heating and streaming
  values while the production equations use typed quantities.
