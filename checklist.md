# Aequitas checklist

Target version: 0.2.0

Sprint phase: Closed — delivered 2026-08-12 (all committed scope verified green)

## Lock-graph verification (Unreleased)

- [x] [patch] Verify the committed dependency graph with locked metadata before
  the feature, Clippy, test, doctest, and Rustdoc gates; remove the unlocked
  lock-rewrite step. Refresh the standalone lock to Eunomia `b6f001a` and
  remove Atlas overlay-only patch entries. Reconcile the gap table to Aequitas
  0.2.0. Hosted run `31785906110` passes verify and supply-chain at merged
  head `7a63b76`.

## AEQ-REL-001 [minor] — crates.io publication

- [x] Enable crates publication with repository-owned trusted-publishing
      automation and no stored registry credential.
- [x] Bump to 0.2.0, index on crates.io, and remove the revision qualifier
      from the Eunomia manifest dependency so consumer workspaces resolve one
      Eunomia source identity.
- [x] Verify the clean-provider gate set at the delivery revision: locked
      metadata, formatting, no-default-feature check, warning-denied
      all-feature Clippy, nextest, doctests, rustdoc, and `cargo deny check`.

## AEQ-QUANT-001 [arch] — quantity-law foundation

- [x] Generic `Quantity<T, D>` over Eunomia scalar types; `repr(transparent)`
      storage equal in size/alignment to `T`.
- [x] Type-level SI dimension algebra over the seven-axis exponent vector.
- [x] Sealed linear-unit contract and conversion SSOT; ZST unit markers.
- [x] Layout, codegen-equivalence, dimension-law, generic-scalar, and
      conversion-property test suites; `uom` 0.38.0 differential oracle
      (dev-only).

## AEQ-QUANT-002 [minor] — SI quantity vocabulary (consumer-driven)

- [x] Fluid/acoustic laws: dynamic/kinematic viscosity, volumetric flow rate,
      acoustic impedance, intensity, volumetric power density, area-per-time.
- [x] Energy-per-volume and affine temperature-difference semantics.
- [x] Surface tension, angle, force, number density, mass-density rate.
- [x] Electrical set: charge, potential, conductance, capacitance, impedance,
      polarizability, electrical conductivity.
- [x] Complex phasors over `eunomia::Complex32`/`Complex64` through the
      provider-owned `UnitScalar` seam (real and quadrature share one unit).
- [x] Dosimetry axis: absorbed dose, absorbed-dose rate with `Gy/s`/`W/kg`,
      `SpecificAbsorptionRate` alias, specific energy.
- [x] Photon/optical: reciprocal length, area-per-mass, energy-per-area,
      molar concentration, nanometer wavelength.
- [x] Biological/kinetic: molar energy, molar heat capacity, reciprocal time,
      reciprocal temperature, reciprocal squared temperature.
- [x] Dynamic/MEMS: acceleration, pressure rate, mechanical impedance,
      volumetric-power-density gradient, pressure-per-current, hydraulic
      resistance/conductance, volume charge density, flexural rigidity.
- [x] Every addition lands an ADR decision (0001–0015) and value-semantic
      tests; no raw-scalar escape in derived laws.

## AEQ-DOC-001 [patch] — provider book closure

- [x] Replace all eight `Chapter prose deferred` placeholders with
      API-accurate prose (quantity model, canonical storage, dimension
      system, linear/scaled units, additive/derived arithmetic, stack
      position).
- [x] Include two runnable example pages and verify link detector 0/0/0 and
      the mdBook build.
- [x] Land the closure commit and the 0.2.0 release; advance the Atlas
      gitlink at the delivery boundary.

## AEQ-FMT-001 [patch] — unit-aware formatting

- [x] Add `quantity::UnitDisplay` formatting a quantity's value in a chosen
      linear unit together with its `LinearUnit::SYMBOL` (e.g. `"2.5 m/s"`).
- [x] Keep `Debug` mirrored to `Display` and add 5 value-semantic tests.
