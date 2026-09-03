# Aequitas backlog

## AEQ-EUNOMIA-IDENTITY-2026-09-03 — Unify Eunomia source identity [patch] — done <a id="aeq-eunomia-identity-2026-09-03"></a>

- **Integrator:** Codex; **branch:** `build/aequitas-eunomia-identity`; **lease:** none.
- **Outcome:** align Aequitas’ Eunomia dependency with provider PR #87 (`fdbf122`) so its quantity scalars remain nominally identical to Leto, Hermes, and Hephaestus without conversions.
- **Acceptance:** standalone lockfile resolves one Eunomia source; check, Clippy, nextest (127/127), doctests (17 runtime plus 9 compile-fail), and rustdoc pass. **Commit:** `0110460`. **Last-update:** 2026-09-03.

Strategic roadmap; tags `[patch]`/`[minor]`/`[major]`/`[arch]` per SemVer class.
Aequitas is the Atlas physical-quantity and dimensional-law SSOT: type-level SI
dimensions, transparent quantities over Eunomia scalars, and linear SI unit
conversion, consumed by proteus, hyperion, kwavers, CFDrs, helios, and the
domain layer.

## AEQ-DOC-BOOK-001 — Execute book samples [patch] — implementation complete; hosted verification pending

- **Owner:** Atlas coordinator; PR [#37](https://github.com/ryancinsight/aequitas/pull/37)
  at exact head `2f63705`.
- **Scope:** the shared Pages caller, the nine existing Rust book fences, and
  the two included book example sources; no public API, lockfile, or dependency
  change.
- **Acceptance:** every existing Rust fence executes through `mdbook test` with
  the packaged `aequitas` library and pinned Rust 1.97.0; `mdbook build` and the
  provider hosted Pages gate pass.
- **Baseline:** the caller used Atlas workflow revision `4c31dd7` without
  `mdbook-test`, and all Rust fences were `rust,ignore`, so the previous local
  test was existence-only.
- **Correction:** the first PR revision used a non-existent Atlas commit and
  failed before job creation in run `32335910858`; commit `2f63705` pins the
  existing root commit `53eb15ae2fa7ee9192e5d006989a430269fdc881`.
- **Local evidence:** format, `mdbook build`, and strict link scan pass. The
  local executable `mdbook test` reaches the real snippets but cannot resolve
  the staged MSVC package and proc-macro artifacts from the shared Windows
  cache; hosted Linux is the acceptance environment.

## AEQ-STRUCTURE-001 — Split oversized unit and law-test leaves [patch] — done 2026-08-18

- **Owner:** Atlas coordinator; scope is the private derived-unit module and
  `tests/dimension_laws.rs` only.
- **Acceptance:** every touched Rust file is below the 500-line structural
  target; all 38 dimension-law tests remain present and value-semantic; the
  public unit paths and standalone module exports remain unchanged.
- **Non-goals:** quantity laws, public names, dependency changes, and deferred
  affine/integer-storage/formatting capabilities.
- **Evidence:** private `derived.rs` is now `derived/mod.rs` plus the
  `derived/transport.rs` leaf (432 and 92 lines); `dimension_laws.rs` is 493
  lines and its six dosimetry/temperature cases live in
  `tests/dimension_laws/dosimetry.rs`. The dimension-law count remains 38.
  Offline local diagnostics pass: all-features Clippy with `-D warnings`,
  Nextest 125/125, doctests 26/26 including compile-fail cases, and rustdoc.
  The locked gate remains blocked before compilation by the Atlas overlay's
  generated lock mismatch; the committed `Cargo.lock` is unchanged.

## AEQ-STRUCTURE-002 — Split derived-unit domain leaves [patch] — done 2026-08-19

- **Owner:** Atlas coordinator; scope is the private
  `src/systems/si/units/derived/` module tree and its manifest only.
- **Acceptance:** `derived/mod.rs` contains declarations and re-exports only;
  every derived-unit implementation has one domain-named leaf; public unit
  paths and all value semantics remain unchanged.
- **Evidence:** the implementation now lives in `geometry`, `kinematics`,
  `mechanics`, `electrical`, `radiation`, `thermal`, and `transport` leaves.
  The conformance scan reduces `manifest_implementation` from `1` to `0`
  with every other tracked class unchanged at zero. Pinned-MSVC all-feature
  Clippy passes, Nextest passes `127/127`, and doctests pass `17` plus `9`
  compile-fail cases with one ignored example. The Atlas overlay rewrote the
  working lock during local gates; it was restored unchanged and is not part
  of the provider increment.

## Active

- [x] [patch] Make CI verify the committed lock graph with
  `cargo metadata --locked` instead of re-resolving provider heads before the
  locked gates. Reconcile the comparison table's delivered `0.2.0` scope.
  Refresh the standalone lock to record Eunomia default `b6f001a` and remove
  Atlas overlay-only patch entries. Evidence: exact default-head run
  `31785906110` passes verify and supply-chain at merged head `7a63b76`, and
  `git diff --check` passes.

- [x] [patch] Refresh the generated ADR index from the existing fifteen
  canonical decision headers. The index now records `Accepted` for every ADR;
  no decision content changes. Evidence: the Atlas generator's scoped output
  matches `docs/adr/README.md` exactly.

## Delivered — foundation (0.1.0)

- [x] [arch] Scaffold the provider: generic `Quantity<T, D>` over Eunomia
  scalar types, type-level SI dimension algebra, and the sealed linear-unit
  conversion SSOT. Decision: [ADR 0001](docs/adr/0001-aequitas-quantity-law.md).
  Evidence: differential verification against `uom` 0.38.0 (dev-only oracle),
  layout tests proving ZST dimension/unit markers and `repr(transparent)`
  quantity storage, and codegen equivalence fixtures.

- [x] [minor] Fluid and acoustic transport quantity laws: dynamic and
  kinematic viscosity, volumetric flow rate, acoustic impedance, intensity,
  volumetric power density, and the named area-per-time contract. Decision:
  [ADR 0002](docs/adr/0002-fluid-acoustic-quantity-laws.md).

- [x] [minor] Energy-per-volume and temperature-difference semantics with
  `J/m³`, `J/ml`, and affine temperature arithmetic (absolute minus absolute
  yields difference; difference plus absolute yields absolute). Decision:
  [ADR 0003](docs/adr/0003-energy-density-temperature-difference.md).

- [x] [minor] Surface-tension semantic distinct from energy-per-area despite
  equal SI base exponents; `N/m` unit with dimensional pressure recovery.
  Decision: [ADR 0004](docs/adr/0004-surface-tension-semantic.md).
- [x] [minor] Angle semantic stored in radians as a distinct dimensionless
  quantity, not a raw scalar. Decision:
  [ADR 0005](docs/adr/0005-angle-semantic.md).
- [x] [minor] Force quantity with the coherent Newton unit. Decision:
  [ADR 0006](docs/adr/0006-force-semantic.md).
- [x] [minor] Number-density quantity with coherent `m⁻³` for population and
  concentration contracts; `ReciprocalVolume` folded onto the same type
  identity as a SSOT alias. Decision:
  [ADR 0007](docs/adr/0007-number-density-contract.md).
- [x] [minor] Electrical quantity set: charge, potential, conductance,
  capacitance (`C`/`V`/`S`/`F`), electrical impedance (`Ω`), and electric
  polarizability (`F·m²`). Decision:
  [ADR 0008](docs/adr/0008-electrical-quantities.md).
- [x] [minor] Complex physical phasors through Eunomia's provider-owned
  `UnitScalar` seam — real and quadrature components share one observable
  unit; no imaginary SI dimension is introduced. Decision:
  [ADR 0009](docs/adr/0009-complex-physical-quantities.md). Cross-link:
  Eunomia E-035.
- [x] [minor] Molar concentration (`mol/m³`, `mol/L`, `µmol/L`) and the
  nanometer optical-wavelength unit. Decision:
  [ADR 0010](docs/adr/0010-molar-concentration-optical-wavelength.md).
- [x] [minor] Electrical conductivity (`S/m`) composing electric-field
  magnitude and mass density into `SpecificAbsorptionRate`. Decision:
  [ADR 0011](docs/adr/0011-electrical-conductivity-quantity.md).
- [x] [minor] Specific-energy semantic (`J/kg`) sharing the coherent absorbed-
  dose axis without a radiation-specific contract. Decision:
  [ADR 0012](docs/adr/0012-specific-energy-semantic.md).
- [x] [minor] Acceleration (`m/s²`) and pressure-rate (`Pa/s`) quantities for
  dynamic-state contracts. Decision:
  [ADR 0013](docs/adr/0013-acceleration-quantity.md).
- [x] [minor] Mechanical impedance (`kg/s`) for force-per-velocity radiation
  and impedance phasors. Decision:
  [ADR 0014](docs/adr/0014-mechanical-impedance-semantic.md).
- [x] [minor] MEMS quantity family: volume charge density (`C/m³`), flexural
  rigidity (`J`), spring stiffness, damping coefficient, pressure-per-
  potential, potential-per-pressure, length-per-potential, and surface charge
  density. Decision:
  [ADR 0015](docs/adr/0015-mems-physical-quantities.md).
- [x] [minor] Absorbed-dose-rate axis with `Gy/s`/`W/kg` and the
  `SpecificAbsorptionRate` alias — radiofrequency and radiation dosimetry name
  one coherent dimension and convert without a scale factor.
- [x] [minor] Thermal-response coefficient dimensions (`m/(s·K)`,
  `kg/(m³·K)`, `1/(m·K)`) and reciprocal-temperature / reciprocal-squared-
  temperature (`K⁻¹`/`K⁻²`) for constitutive-law coefficients.
- [x] [minor] Photon and optical interaction dimensions: reciprocal length,
  area-per-mass, energy-per-area (`m⁻¹`, `cm⁻¹`, `m²/kg`, `cm²/g`, `J/m²`).
- [x] [minor] Biological-response vocabulary: absorbed dose, molar energy,
  molar heat capacity, reciprocal time (`Gy`, `J/mol`, `J/(mol·K)`, `s⁻¹`).
- [x] [minor] Mass-density rate (`kg/(m³·s)`), reciprocal-time-squared,
  volumetric-power-density gradient (`W/m⁴`), pressure-per-current,
  quadratic hydraulic resistance, and hydraulic conductance.
- [x] [patch] Optional `serde` support serializing quantities as canonical
  scalar values while retaining compile-time dimensions in Rust APIs.

## Delivered — release and documentation

- [x] [minor] Publish Aequitas 0.2.0 with repository-owned trusted-publishing
  automation and one resolved Eunomia source identity (revision qualifier
  removed). Evidence: crates.io indexing and the clean-provider gate set.
- [x] [patch] Author and close the eight-chapter provider book
  (ATLAS-AEQUITAS-PROVIDER-DOCS-001): quantity model, canonical SI storage,
  dimension system, linear and scaled units, additive and derived arithmetic,
  stack position, plus two runnable examples. Evidence: link detector 0/0/0
  and mdBook build.
- [x] [patch] Add unit-aware `UnitDisplay` formatting (`"2.5 m/s"`) with
  `Debug`/`Display` parity; the printed value is materialized through
  `Quantity::in_unit`. Evidence: 5 value-semantic tests (velocity, scaled
  length, derived energy, Debug/Display parity, immutability).
- [x] [patch] Scalar-arithmetic ergonomics: commutative scalar-left
  multiplication (`scalar * quantity`), `MulAssign`/`DivAssign` in-place
  scaling, and the complex-phasor (`Quantity<Complex<T>, D>`) equivalents.
  Landed upstream as PR #21 (commit `dd0b8e1`, merge `0052b80`); evidence:
  9 value-semantic tests (commutativity, in-place scaling, dimension/unit
  preservation, composition with quantity arithmetic, value-vs-compound
  parity, complex paths) plus the f32 / complex-division paths completed in
  the `rational-powers` increment.
- [x] [patch] Rational-power operations beyond `uom`: type-level
  `SqrtDimension`/`CbrtDimension` over the concrete exponent tuples in the
  SI inventory (8 sqrt shapes, 3 cbrt shapes; exact division only, so
  odd-exponent dimensions get no impl by construction) and `Quantity::sqrt`/
  `cbrt` rooting the scalar through the `FloatElement` power surface —
  `sqrt(area)` is a `Length`, `cbrt(volume)` is a `Length`, with no runtime
  dimension checks. Landed on `codex/aequitas-root-ops-closure`
  (`72ef8b4`); evidence: 12 value-semantic tests and the full canonical gate
  set (fmt, `-D warnings` all-targets, clippy, nextest, doctests,
  no-default).
- [x] [patch] Eunomia-owned sign-preserving `cbrt` (`libm::cbrtf` default,
  native `libm::cbrt` for `f64`/`F64`) as the scalar-math SSOT;
  `Quantity::cbrt` now roots through `FloatElement::cbrt` (`cbrt(-8 m³) ==
  -2 m`), dropping the `powf(1/3)` path and its NaN-for-negative-operands
  caveat. Cross-link: root ATLAS-AEQUITAS-ROOT-OPS-012 follow-up.
- [x] [patch] Semantics-marked sqrt/cbrt variants: `Angle::sqrt()` (→
  dimensionless) and `ReciprocalVolume::cbrt()` (→ reciprocal length) now
  compile; the root output normalizes the semantic marker to
  `BaseSemantics`, matching `MultiplyDimension`/`DivideDimension`. Evidence:
  2 value-semantic tests (angle and reciprocal-volume paths).

## Deferred (documented boundary)

- [ ] [minor] Affine unit kinds and quantity kinds beyond the linear-unit
  slice (`uom`-style `Kind` system). Not required by current Atlas consumers;
  revisit when a consumer needs offset units beyond temperature.
- [ ] [minor] Integer and rational quantity storage. The simulation boundary
  is floating-point over Eunomia scalars; revisit on a consumer need.
- [ ] [patch] Broader formatting surface beyond `UnitDisplay` (unit-algebra
  pretty-printing, `Display` for dimension/unit markers). Currently outside
  the boundary.
