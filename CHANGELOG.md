# Changelog

All externally observable changes are recorded here.

## [Unreleased]

### Changed

- The Python floor moves from 3.8 to 3.10, and the structural read allocates
  nothing. `abi3-py310` is the lowest stable-ABI level that exposes
  `PyUnicode_AsUTF8AndSize`, so it is the lowest at which the tag's marker name
  can be borrowed rather than copied into a `String` on every call; that is the
  read's last Rust heap allocation, now gone (measured 1.0000 -> 0.0000 per
  call, with the floor and the code change separated so neither alone accounts
  for it). Observable to a Python user twice over: `requires-python` is now
  `>=3.10` and the abi3 wheel is tagged `cp310-abi3`. Python 3.8 and 3.9 are
  past end of life, and `kwavers-python` still publishes the 3.8 floor, so a
  user on either version can install that distribution but not this one.

- The semantic wire name resolves to its marker through
  `SemanticTag::from_name`, beside `name()` in the file that owns the
  vocabulary, instead of a scan of `SemanticTag::ALL` that both the consumer and
  the round-trip test wrote out separately. Not observable — the same names map
  to the same markers and an unknown name raises the same `ValueError` — but the
  mapping now has one owner, and the read path resolves it by `match` rather
  than by comparing every earlier name.

- The structural quantity read allocates once per call instead of twice. The
  protocol tag's exponents are read straight into the fixed-width axis array,
  where unpacking the tag as `(Vec<i64>, String)` built a seven-element vector
  only to copy it and drop it. Nothing observable changes: the admitted input is
  the same (any iterable of integers, not merely a sequence, which is why
  pyo3's array extractor was rejected), and every error message this function
  raises is unchanged, with the malformed-tag cases that assert them extended to
  cover an over-long tag and an out-of-range exponent. Measured 2.000 -> 1.000
  allocations per call on both the read and the `Dimensioned` path. The
  remaining allocation is the semantic marker's name, which cannot be borrowed
  under this crate's `abi3-py38` floor.

- Split the law crate's dimensional-identity tests into one leaf per SI unit
  domain (kinematics, mechanics, thermal, transport, hydraulics, electrical,
  radiation), moved the complex-valued identities into the leaf that already
  owns the value-kind axis, and reduced the aggregate file to `#[path]` wiring.
  The affine temperature arithmetic is likewise separated from the
  dimension-generic additive kernel it was mixed with. Nothing observable
  changes: the 40 test functions are identical by name, the public quantity and
  unit paths are untouched, and `scripts/generate-surface.py check` still
  reports all four generated artifacts current.

- Enable executable mdBook samples through the shared Atlas Pages workflow.
  The nine existing Rust fences now compile against the packaged `aequitas`
  library under the pinned Rust 1.97.0 gate instead of being silently ignored.
  The caller also pins the existing Atlas workflow commit so the hosted reusable
  workflow resolves before job creation.

- Split the derived SI unit implementations into domain-named modules while
  retaining the existing public unit paths and transport boundary. The
  `derived` manifest now contains only module declarations and re-exports.

- Split the binding's test files and production modules into one leaf per
  contract, with shared fixtures defined once instead of per leaf. Nothing
  observable changes: the declaration surface is identical (71 function
  definitions, none added and none removed), the generated classes, stubs and
  method behaviour are untouched, and the suite still reports 239 tests.

- `DimensionTag::combine` takes its checked operation as a generic parameter
  instead of a `fn` pointer, so both call sites monomorphize and the seven-axis
  loop inlines. Not observable, and no speedup is claimed: the measured change
  (1.70 ns to 1.79 ns per call) is noise for a loop of seven checked
  additions, so the justification is that the abstraction is now zero-cost.

### Added

- `aequitas-python` publishes an x86_64 macOS wheel per ABI. The release matrix
  ran four native runners and Intel macOS was not among them, so `pip` there
  fell back to the sdist and needed a Rust toolchain. The cell uses
  `macos-15-intel`, the only standard-runner label for that architecture
  (`macos-13` and its larger siblings were retired on 2025-12-04), and that
  label is itself the last x86_64 image Actions publishes, retiring with the
  macOS 15 image in August 2027 -- so the architecture's end date is recorded
  in the matrix rather than discovered at that point. Both ABIs are built and
  smoke-tested there: `setup-python` serves the free-threaded interpreter on
  Intel macOS as well as the GIL build.

- Compose SI units without adding a named marker per derived unit: `Unit<D>`
  owns conversion scale and allocation-free symbol formatting, a blanket impl
  derives it from `LinearUnit<D>`, and explicit impls cover `Product`,
  `Quotient` and integer `Power`. Coherence stays closed because `LinearUnit`
  now also requires a private named-marker trait, which additionally requires
  `UnitDimension`, so a named unit cannot ship without its composition
  dimension. Decision:
  [ADR 0018](docs/adr/0018-unit-composition.md). Compound scales are
  const-evaluated, inverse conversions divide natively instead of multiplying
  by a reciprocal that could overflow first, and Celsius/Fahrenheit contribute
  temperature *intervals* only -- a compound expression cannot construct an
  affine absolute temperature. Nothing observable changes for named units:
  `scripts/generate-surface.py check` still reports all four generated
  artifacts current. Evidence: `tests/unit_composition.rs` compares named and
  composed units, signed and zero powers, scaled denominators, nested
  expressions, unnamed dimensions and formatting, and compile-fail doctests
  reject dimension and semantic mismatches. No performance improvement is
  claimed.

- `pyaequitas` declares itself safe on free-threaded Python: importing it on
  3.14t leaves the GIL off, and CI runs the Python suite on a free-threaded
  interpreter.

- `pyaequitas` stubs type arithmetic: to a type checker `aq.length(1.0) *
  aq.length(2.0)` is an `aq.Area`, and `aq.length(1.0) + aq.time(1.0)` is an
  error, as it is a `ValueError` at runtime.

- `pyaequitas` has a class per named dimension. `aq.length(2.0)` is an
  `aq.Length`, the product of two lengths an `aq.Area`, and a dimension no
  alias names a plain `aq.Quantity`. Aliases of one dimension are one class
  (`aq.ThermalDiffusivity is aq.AreaPerTime`); the stubs declare every class
  and type each constructor as returning its own, and `aq.CLASSES` maps every
  name to its class.

- Added the `Degree` linear unit for the `Angle` dimension, with
  `SCALE = pi / 180` — the exact defining ratio. `Angle` already carried its own
  semantics but `Radian` was its only unit, so degree-facing call sites wrote
  `Angle::from_unit::<Radian>(x.to_radians())` and the unit lived in a method
  name rather than the type. `Angle::from_unit::<Degree>(90.0)` and
  `angle.in_unit::<Degree>()` now express that directly. Angles remain
  non-additive: `Add` is bound on `BaseAdditiveDimension`, which
  `AngleSemantics` does not implement, and that is unchanged.

- Added the `PowDimension<P>` trait: raising a dimension to the type-level
  integer power `P` multiplies every SI base exponent through typenum
  `Prod`. `Quantity::powi::<P>()` applies the scalar power via
  `FloatElement::powi` while raising the dimension itself at the type
  level — `Length::powi::<P2>()` is an `Area`, `Time::powi::<N1>()` is a
  `ReciprocalTime`, and `powi::<Z0>()` is dimensionless. `uom`'s `powi`
  only scales the value at runtime with a hardcoded per-quantity dimension,
  so this is a strict capability extension. Semantics-marked dimensions
  normalize to `BaseSemantics` (matching `MultiplyDimension` /
  `DivideDimension`). 17 value-semantic tests cover squaring / cubing,
  inversion, negative powers, zero-power, round-trips through `sqrt`,
  dimension shapes, display in `m²`, and semantics normalization.

- Added type-level `SqrtDimension` and `CbrtDimension` traits on the SI
  dimension vector: exponents divisible by two (three) map to their exact
  type-level quotient, implemented for the concrete exponent tuples present
  in the shipped dimension inventory (area, mass-squared,
  acceleration-squared, reciprocal-time-squared,
  reciprocal-temperature-squared, volume, number-density, angle,
  reciprocal-volume, and dimensionless; semantics-marked inputs normalize
  to `BaseSemantics`). `Quantity::sqrt` and `Quantity::cbrt` root the scalar
  through `FloatElement` (`cbrt`; `powf(0.5)` for `sqrt`) while carrying the
  halved / thirded dimension — `sqrt(area)` is a `Length` and
  `cbrt(volume)` is a `Length`, with no runtime dimension checks. This is the capability the
  `uom` crate cannot express generically (its dimensions are
  integer-exponent only). 14 value-semantic tests cover dimension halving /
  thirding, unit-aware display, round-trips through quantity multiplication,
  the kinetic-energy composition (`sqrt(2E/m)`), the
  acceleration-squared path, the reciprocal-time / reciprocal-temperature
  paths, and the sign-preserving `cbrt` of negative operands.

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

### Fixed

- `pyaequitas` scaling by a bare number keeps the quantity's dimension, as
  `Quantity<T, D> * T` does in Aequitas. `aq.stress(1.0) * 2.0` was a
  pressure and `aq.angle(1.0) * 2.0` a bare dimensionless value, so a scaled
  stress no longer added to a stress.

- `pyaequitas` arithmetic reads the quantity protocol before `__float__`. A
  protocol-conforming quantity that also defines `__float__` was multiplied
  or divided as a bare scalar, so a length times such a time came back a
  length, with no error raised. Bare numbers remain dimensionless scalars.

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
