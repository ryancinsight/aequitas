# ADR 0003: Add energy density and temperature-difference semantics

- Status: Accepted
- Date: 2026-07-23
- Class: [arch] [minor]

## Context

CFDrs and Kwavers still expose energy-per-volume and temperature-rise values at
raw-scalar boundaries. Aequitas already owns the corresponding SI dimensions
for energy, volume, and absolute temperature, but it did not provide an
energy-density alias or distinguish an absolute temperature from a temperature
difference. That gap permits a temperature rise to be represented as an
absolute temperature and leaves consumer energy-density metrics without a
provider-owned unit contract.

## Decision

Add provider-owned `EnergyPerVolume` and `TemperatureDifference` quantities.
`JoulePerCubicMeter` is the coherent unit and `JoulePerMilliliter` is the
scaled clinical/acoustic unit. `Kelvin` implements both absolute-temperature
and temperature-difference unit contracts, while the type-level semantic
markers keep the quantities distinct.

Temperature arithmetic follows the affine distinction:

- absolute minus absolute produces a temperature difference;
- absolute plus or minus a difference produces an absolute temperature;
- differences add and subtract as differences;
- multiplication and division normalize semantic markers to ordinary base
  dimensions for dimensional composition.

Consumer crates retain their serialization and domain validation boundaries.
They adopt these quantities inside equations and convert at the existing
boundary only.

## Rejected alternative

Keeping `TemperatureDifference` as an alias of absolute temperature would
preserve source compatibility but would not encode the distinction the
consumer metrics require. Adding consumer-local energy-density wrappers would
duplicate Aequitas's dimension and unit ownership.

## Verification

- Dimension-law tests prove energy divided by volume and temperature-response
  compositions.
- Arithmetic tests prove absolute-temperature subtraction and offsetting by a
  difference preserve the semantic output type.
- Layout tests prove new dimensions and unit markers remain zero-sized.
- README examples compile against the public quantity and unit surface.
