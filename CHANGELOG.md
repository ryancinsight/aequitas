# Changelog

All externally observable changes are recorded here.

## Unreleased

### Changed

- Removed the revision qualifier from the Eunomia manifest dependency.
  Consumer workspaces now resolve one Eunomia source identity while their lock
  files retain the exact reproducibility pin.

### Added

- Absorbed-dose, molar-energy, molar-heat-capacity, and reciprocal-time
  dimensions with coherent `Gy`, `J/mol`, `J/(mol·K)`, and `s⁻¹` units for
  biological-response and kinetic-law contracts.
- Reciprocal-temperature and reciprocal-squared-temperature dimensions,
  quantities, and coherent `K⁻¹`/`K⁻²` units for typed constitutive response
  coefficients.
- Generic `Quantity<T, D>` representation over Eunomia scalar types.
- Type-level SI dimensional algebra and linear unit conversion.
- Initial SI quantities and units required by Atlas multiphysics consumers.
- Specific heat capacity with a dimensionally checked J/(kg·K) contract.
- Differential verification against `uom` 0.38.0.
