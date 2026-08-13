# Changelog

All externally observable changes are recorded here.

## [Unreleased]

### Added

- Added type-level `SqrtDimension` and `CbrtDimension` traits on the SI
  dimension vector: exponents divisible by two (three) map to their exact
  type-level quotient, implemented for the concrete exponent tuples present
  in the shipped dimension inventory (area, mass-squared,
  acceleration-squared, reciprocal-time-squared,
  reciprocal-temperature-squared, volume, number-density, and
  dimensionless). `Quantity::sqrt` and `Quantity::cbrt` root the scalar
  through the `FloatElement` power surface while carrying the halved /
  thirded dimension — `sqrt(area)` is a `Length` and `cbrt(volume)` is a
  `Length`, with no runtime dimension checks. This is the capability the
  `uom` crate cannot express generically (its dimensions are
  integer-exponent only). 12 value-semantic tests cover dimension halving /
  thirding, unit-aware display, round-trips through quantity multiplication,
  the kinetic-energy composition (`sqrt(2E/m)`), the
  acceleration-squared path, the reciprocal-time / reciprocal-temperature
  paths, and the `NaN` domain of the power surface for negative operands.

- Completed the scalar-arithmetic ergonomics on `Quantity`: scalar-left
  multiplication (`scalar * quantity`) is now commutative with the existing
  quantity-right form, and `MulAssign`/`DivAssign` scale quantities in place.
  The complex-phasor (`Quantity<Complex<T>, D>`) surface gains the same
  commutative scalar product and compound-assignment operators. 9
  value-semantic tests cover commutativity, in-place scaling, dimension and
  unit preservation, composition with quantity arithmetic, value-vs-compound
  parity, and the complex-phasor paths.

- Added `quantity::UnitDisplay`, a unit-aware display wrapper that formats a
  quantity's value in a chosen linear unit together with its
  `LinearUnit::SYMBOL` abbreviation (e.g. `"2.5 m/s"`). The value is
  materialized through `Quantity::in_unit`, so the printed number is expressed
  in the requested unit rather than the canonical SI base unit. `Debug`
  mirrors `Display`. 5 value-semantic tests cover velocity, scaled length
  (kilometres), derived energy, Debug/Display parity, and immutability.

## [0.2.0] - 2026-08-09

### Changed

- Enabled crates.io publication with repository-owned trusted-publishing
  automation.
- Removed the revision qualifier from the Eunomia manifest dependency.
  Consumer workspaces now resolve one Eunomia source identity while their lock
  files retain the exact reproducibility pin.

### Added

- Added provider-owned `VolumeChargeDensity` (`C/m³`) and
  `FlexuralRigidity` (`J`) quantities for MEMS charge-gradient and plate
  rigidity contracts. Eunomia complex values retain one observable unit for
  real and quadrature components; no imaginary SI dimension is introduced.

- Added the semantically distinct `MechanicalImpedance` quantity with the
  coherent `kg/s` unit for force-per-velocity radiation and mechanical
  impedance phasors. Eunomia `Complex` real and quadrature components retain
  this single observable unit; no imaginary SI dimension is introduced.

- Added the `VolumetricPowerDensityGradient` quantity and coherent `W/m⁴`
  unit for formulas whose spatial power-deposition derivative is explicit.
  This remains a real SI dimension; complex Eunomia values are still confined
  to numerical phasor boundaries and do not create an imaginary unit.

- Added provider-owned temperature derivatives for velocity, mass density,
  and reciprocal length, with coherent `m/(s·K)`, `kg/(m³·K)`, and `1/(m·K)`
  units. These are real SI dimensions; Eunomia complex values remain valid at
  numerical phasor boundaries without introducing an imaginary unit.

- Added the `Acceleration` quantity and coherent `m/s²` unit for dynamic
  state contracts such as therapeutic microbubble wall acceleration. Eunomia
  complex values remain valid for genuine phasor data; acceleration is a real
  physical dimension with no imaginary-unit counterpart.

- Added the `PressureRate` quantity and coherent `Pa/s` unit for acoustic
  pressure-time-derivative contracts. It is a real pressure rate; Eunomia
  complex values remain reserved for genuine complex phasors at numerical
  boundaries.

- Added the semantic `SpecificEnergy` alias and coherent `JoulePerKilogram`
  unit for mechanical specific-energy metrics such as turbulent kinetic
  energy. It shares the `J/kg` dimension with absorbed dose without introducing
  a radiation-specific or imaginary unit.

- Added the `ElectricalConductivity` quantity and coherent `S/m` unit for
  electromagnetic power-deposition consumers. Its dimensional law composes
  with electric-field magnitude and mass density to produce
  `SpecificAbsorptionRate`; complex Eunomia values remain valid at the
  numerical phasor boundary, while SAR itself is a real magnitude metric.
- Added semantic `MolarConcentration` with `mol/m³`, `mol/L`, and `µmol/L`
  units, plus the `Nanometer` length unit for optical contracts. These
  real-valued units
  preserve Eunomia's complex-valued formula and storage boundaries without
  introducing an imaginary physical unit.
- Named `AreaPerTime` quantity and dimension for planar flow rate per unit
  width, sharing one provider-owned SI axis with thermal diffusivity and
  kinematic viscosity.
- Added `ReciprocalTimeSquared` for vorticity-squared and enstrophy metrics.

- Electric polarizability with coherent `F·m²` units, including Eunomia
  `Complex64` phasors, plus a semantically distinct reciprocal-volume quantity
  for geometric coupling coefficients. Reciprocal volume and entity number
  density share SI exponents but remain separate type-level contracts.

- Mechanical and voltage-transducer dimensions for MEMS consumers: spring
  stiffness, damping coefficient, pressure-per-potential, potential-per-pressure,
  length-per-potential, and surface charge density, with coherent SI unit
  markers. These dimensions keep CMUT/PMUT and sensitivity contracts on the
  provider-owned quantity surface.

- Complex phasor unit conversion through Eunomia's provider-owned
  `UnitScalar` seam, plus the electrical-impedance `Ω` dimension and unit.
  Real and imaginary components share one physical unit; no imaginary unit is
  introduced.
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
