# Aequitas checklist

Target version: 0.2.0

Sprint phase: Closed — delivered 2026-08-12 (all committed scope verified green)

## AEQ-STRUCTURE-001 — Oversized unit and law-test leaves

- [x] Move the private derived-unit implementation into a vertical module and
      place the transport leaves in a dedicated child module.
- [x] Move the final dosimetry/temperature law group behind a test module
      without changing test bodies or public unit exports.
- [x] Run the provider format, locked package, Clippy, Nextest, doctest, and
      rustdoc gates; formatting, Clippy, Nextest 125/125, doctests 26/26, and
      rustdoc pass on the offline local graph. The locked package gate remains
      blocked before compilation by the Atlas overlay's lock mismatch; the
      committed lock was restored unchanged.

## AEQ-STRUCTURE-002 — Derived-unit domain leaves

- [x] Split the derived-unit implementations into domain-named leaves for
      geometry, kinematics, mechanics, electrical, radiation, and thermal
      units while retaining the transport leaf and the public re-export list.
- [x] Verify the conformance ratchet reaches
      `manifest_implementation=0` without changing any other class.
- [x] Run formatting, pinned-MSVC all-feature Clippy, all-feature Nextest
      `127/127`, and workspace doctests (`17` pass, `9` compile-fail pass,
      `1` ignored). Restore the overlay-generated lock and leave it unchanged.

## AEQ-STRUCTURE-003 — Contract-shaped binding test leaves

- [x] Split `tag/tests.rs` by contract: derivation from the law crate's type
      parameters, the semantic marker, the exponent algebra with its refusals,
      and the wire form.
- [x] Split `units/tests.rs` by contract: the shape of the generated inventory,
      lookup and aliasing, the differential conversion oracle, and the affine
      offset contract. `assert_exact`, `assert_exact_in_context` and `sweep()`
      are defined once, in the `fixtures` leaf.
- [x] Split `consumer/tests.rs` by seam: the shared object builder and foreign
      quantities, `Dimensioned`, the structural read, and the finiteness policy.
- [x] Verify the function set is unchanged -- 51 functions, identical names
      before and after -- and run the gate: formatting, all-feature
      all-targets Clippy with `-D warnings`, Nextest `239/239`, doctests, and
      Rustdoc. The overlay-generated lock is restored byte-for-byte.

## AEQ-STRUCTURE-004 — Production leaves and the tag algebra's indirection

- [x] Split `tag/model.rs` (223 lines) into the tag's identity (`model`), its
      exponent algebra (`algebra`) and its two refusals (`error`).
- [x] Split `quantity/model.rs` (219 lines) by seam: the value (`model`), the
      unit-resolution constructors (`construct`), the inspection getters
      (`inspect`) and the cross-extension wire form with its read-back
      (`wire`). Private fields stay private; the leaves use the accessors.
- [x] Make `DimensionTag::combine` generic over its checked operation instead
      of taking a `fn` pointer, so both call sites monomorphize and the
      seven-axis loop inlines. No speedup is claimed: the probe measured
      1.70 ns before and 1.79 ns after.
- [x] Verify 71 function definitions before and after with none added or
      removed, then run the gate: formatting, all-feature all-targets Clippy
      with `-D warnings`, Nextest `239/239`, doctests, Rustdoc under
      `-D warnings`, and `--no-default-features`.
- [x] Record the measured cost of the structural read (`~315 ns`, 2
      allocations per call) and of the scans around it (`by_tag` 1.24 ns,
      `in_unit`'s scan 7.56 ns) in `gap_audit.md`; file the unreduced cost as
      `AEQ-PY-READ-COST-2026-09-21` rather than leaving it unstated.

## AEQ-STRUCTURE-005 — Law-crate domain leaves and the affine arithmetic seam

- [x] Split `tests/dimension_laws.rs` (439 lines, 28 tests) into one leaf per
      unit domain -- kinematics, mechanics, thermal, transport, hydraulics,
      electrical, radiation -- leaving the aggregate as `#[path]` wiring so the
      identities still compile as one integration binary.
- [x] Move the three complex-valued identities into `complex.rs`, whose
      existing charter already owns the value-kind axis, instead of leaving
      them under a domain leaf that does not describe them.
- [x] Split `src/quantity/arithmetic/additive.rs` (169 lines) into the
      dimension-generic kernel with its private `BaseAdditiveDimension`
      witness, and `affine.rs` holding the nine temperature impls that relate
      two dimensions instead of preserving one.
- [x] Verify the function set is unchanged -- 40 test functions, identical
      names and bodies before and after -- and that the generator's own `check`
      mode still reports all four generated artifacts current.
- [x] Run the gate: formatting, all-feature all-targets Clippy with
      `-D warnings`, Nextest `239/239`, doctests, Rustdoc under `-D warnings`,
      and `--no-default-features`. The overlay-generated lock is restored
      byte-for-byte.
- [x] Record why `systems/si/{quantities,dimensions}.rs` and `units/scaled.rs`
      stay whole, citing the generator's read targets, so the same split is not
      re-attempted on a pinned file.

## AEQ-UNIT-COMPOSITION — Compose linear units through dimensional algebra

- [x] Give compound units a contract of their own: `Unit<D>` owns conversion
      scale and symbol formatting, a blanket impl derives it from
      `LinearUnit<D>`, and explicit impls cover `Product`, `Quotient` and
      integer `Power`. `LinearUnit` additionally requires the private
      named-marker trait, which itself requires `UnitDimension`, so a named
      unit cannot ship without its composition dimension.
- [x] Keep const evaluation total: compound scales are const-evaluated and
      bounded, inverse conversions use native division rather than a
      reciprocal that could overflow, and Celsius/Fahrenheit contribute
      temperature intervals so no compound expression yields an affine
      absolute temperature. Decision:
      [ADR 0018](docs/adr/0018-unit-composition.md).
- [x] Value tests cover named and composed equality, signed and zero powers,
      scaled denominators, nested expressions, unnamed dimensions and
      formatting, with compile-fail doctests for the dimension and semantic
      refusals.
- [x] Run the gate on the combined tree: formatting, all-feature all-targets
      Clippy with `-D warnings`, Nextest `252/252`, doctests, Rustdoc under
      `-D warnings`, `--no-default-features`, generated-surface freshness and
      the SemVer check. The overlay-generated lock is restored byte-for-byte.
- [x] Land it: composition committed on `codex/unit-composition` and landed as
      [#78](https://github.com/ryancinsight/aequitas/pull/78) (`b159a32`),
      with the binding leaves it composes into landed as
      [#77](https://github.com/ryancinsight/aequitas/pull/77) (`3746b36`).
      Limits: the design and its claims are the authoring session's -- what is
      verified here is that the combined tree passes the gate set, and the
      tests are the ones that came with it, so this adds no independent
      coverage.

## ATLAS-AEQUITAS-AUDIT-075 — Isolated provider re-verification — closed 2026-08-16

- [x] Re-run the locked provider gate set from an isolated checkout at the
      current provider head; record exact results and evidence limits in
      `gap_audit.md`.
- [x] Confirm deferred affine, integer/rational-storage, and broader-formatting
      capabilities remain consumer-gated and do not require implementation in
      this increment.
- [x] Complete the provider-local documentation closure and hosted checks.
      Isolated gates are green at `4ab0eb4`; hosted validation remains the
      authoritative release boundary for a future provider change.

## Lock-graph verification (Unreleased)

- [x] [patch] Verify the committed dependency graph with locked metadata before
  the feature, Clippy, test, doctest, and Rustdoc gates; remove the unlocked
  lock-rewrite step. Refresh the standalone lock to Eunomia `b6f001a` and
  remove Atlas overlay-only patch entries. Reconcile the gap table to Aequitas
  0.2.0. Hosted run `31785906110` passes verify and supply-chain at merged
  head `7a63b76`.

## ADR governance

- [x] Regenerate `docs/adr/README.md` from the fifteen canonical ADR headers;
      normalize the generated status tokens to `Accepted` without changing
      any decision record.

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
