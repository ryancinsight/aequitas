# Changelog

All externally observable changes are recorded here.

## Unreleased

### Changed

- Removed the revision qualifier from the Eunomia manifest dependency.
  Consumer workspaces now resolve one Eunomia source identity while their lock
  files retain the exact reproducibility pin.

### Added

- Electrical charge, potential, conductance, and capacitance quantities with
  coherent Coulomb, Volt, Siemens, and Farad units for typed biophysical and
  sonogenetics contracts.
- Number-density quantity with the coherent `m⁻³` unit for typed population and
  concentration contracts.
- Force quantity with the coherent Newton unit for typed force-balance and
  mechanical-result contracts.
- Absorbed-dose-rate dimension with the coherent `Gy/s` and `W/kg` units, plus
  the `SpecificAbsorptionRate` alias. Radiofrequency dosimetry and radiation
  dosimetry name the same coherent SI dimension, so both spellings resolve to
  one axis and convert without a scale factor. Completes the interface
  vocabulary for the Atlas deposition spine, whose remaining quantities
  (`Intensity`, `VolumetricPowerDensity`, `EnergyPerArea`, `AbsorbedDose`, and
  the bioheat coefficient set) were already present.
- Angle quantity with a distinct radian semantic contract for typed rotational
  and plane-wave geometry.
- Pressure-per-electric-current, quadratic hydraulic-resistance, and hydraulic
  conductance dimensions for typed transducer and nonlinear vascular metrics.
- Optional `serde` support serializes quantities as their canonical scalar
  values while retaining compile-time dimensions in Rust APIs.
- Surface-tension quantity with a distinct semantic dimension and coherent
  `N/m` unit for interfacial and cavitation contracts.
- Mass-density-rate quantity with the coherent `kg/(m³·s)` unit for typed
  perfusion and mass-transfer contracts.
- Energy-per-volume and temperature-difference quantities with `J/m³`,
  `J/ml`, and affine temperature arithmetic for consumer metric contracts.
- Dynamic and kinematic viscosity, volumetric flow rate, acoustic impedance,
  intensity, and volumetric power-density dimensions with coherent SI units.
- Reciprocal-length, area-per-mass, and energy-per-area dimensions with
  `m⁻¹`, `cm⁻¹`, `m²/kg`, `cm²/g`, and `J/m²` units for typed photon and
  optical interaction contracts.
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
