# ADR 0008: Add SI Electrical Quantities for Biophysical Contracts

## Status

Accepted — 2026-07-27

## Context

Kwavers sonogenetics exposes membrane and channel parameters whose physical
meaning is capacitance, electric conductance, electric potential, or electric
charge. Aequitas already provides electric current, but the missing dimensions
force consumers to carry unit-documented scalar values at public boundaries.

## Decision

Add the four coherent SI dimensions and units to Aequitas:

- `ElectricCharge` with `Coulomb` (`C`), `I·s`;
- `ElectricPotential` with `Volt` (`V`), `kg·m²·s⁻³·A⁻¹`;
- `ElectricConductance` with `Siemens` (`S`), `kg⁻¹·m⁻²·s³·A²`; and
- `Capacitance` with `Farad` (`F`), `kg⁻¹·m⁻²·s⁴·A²`.

The provider owns these shared dimensions. Consumers compose them through the
existing quantity arithmetic and extract scalars only at numerical or
serialization boundaries.

## Alternatives rejected

- Consumer-local electrical wrappers: rejected because they duplicate provider
  dimensions and create competing unit vocabularies.
- Reusing `ElectricCurrent` or `Dimensionless`: rejected because the resulting
  contracts would be dimensionally unsound and would admit incompatible values.

## Verification

Dimension-law tests verify charge, capacitance, and conductance composition from
current, time, and potential. Layout tests verify that all four dimension and
unit markers are zero-sized. The standard Aequitas Nextest, Clippy, doctest, and
Rustdoc gates cover the public exports and README example.
