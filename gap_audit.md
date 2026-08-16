# Aequitas gap audit

## ATLAS-AEQUITAS-AUDIT-075 — Isolated provider re-verification — closed 2026-08-16

The current provider head is `4ab0eb4` (`chore(aequitas): Promote the lint
floor to a denied workspace table`). The isolated checkout passed the
provider-owned gates from outside the Atlas umbrella overlay:

- `cargo fmt --check --manifest-path .../Cargo.toml`
- `cargo check --locked --all-features --all-targets --manifest-path .../Cargo.toml`
- `cargo clippy --locked --all-targets --all-features --manifest-path .../Cargo.toml -- -D warnings`
- `cargo nextest run --locked --all-features --manifest-path .../Cargo.toml`: 104/104 passed, 0 skipped
- `cargo test --locked --doc --all-features --manifest-path .../Cargo.toml`: 13 runtime doctests and 8 compile-fail doctests passed; 1 doctest is intentionally ignored
- `cargo doc --locked --no-deps --all-features --manifest-path .../Cargo.toml`
- `cargo deny check`: advisories, bans, licenses, and sources passed

The cargo-deny run emitted one expected `unmatched-source` warning for the
Eunomia Git source because the Atlas development overlay resolves that
dependency locally. The overlay also rewrites the local lockfile; that derived
churn was discarded and is not a provider defect. The standalone locked gates
above were run outside the overlay and did not require a lockfile rewrite.

The deferred affine-unit, integer/rational-storage, and broader-formatting
capabilities remain documented consumer-gated boundaries. No current Atlas
consumer requires them, so this increment adds no speculative API or storage
variant. The suite supplies dimension-law, conversion-property, generic-scalar,
layout, and `uom` differential evidence; it does not establish runtime
performance, memory usage, hardware behavior, or hosted release readiness.

## Closed gaps

### ADR index freshness

The generated ADR index previously retained missing and date-suffixed status
tokens despite all fifteen decision records carrying canonical accepted
statuses. It now matches the generator output and records `Accepted` for every
ADR. No decision content changed; this is documentation-derived state.

### Complex-unit conversion seam (resolved via Eunomia E-035)

The stack audit found Aequitas could convert `FloatElement` values but had no
provider-owned conversion seam for Eunomia's native complex phasors. Eunomia
now owns `UnitScalar` for every shipped real storage type and
`Complex32`/`Complex64`; Aequitas consumes that single seam, so no
overlapping real/complex conversion implementations or consumer-local
wrappers remain. Evidence: the Eunomia provider test and the Aequitas
complex-unit contract test (`test(si): Verify complex unit conversion`).
Cross-link: Eunomia ADR 0004 / gap_audit §Physical-unit scalar conversion.

### `ReciprocalVolume` vs `NumberDensity` duplicate axis

`ReciprocalVolume` duplicated the `NumberDensity` dimension axis. Resolved by
folding `ReciprocalVolume` onto `NumberDensity` as a SSOT alias — one type
identity, no redundant implementation (`d9e464a`).

### Unit-aware formatting gap

Raw scalar formatting left unit context implicit. Resolved by
`quantity::UnitDisplay` (value materialized through `Quantity::in_unit` plus
`LinearUnit::SYMBOL`), with `Debug`/`Display` parity and 5 value-semantic
tests.

### `uom` feature parity

The `uom` 0.38.0 comparison table (see [§uom gap analysis](#uom-gap-analysis))
shows the intentional boundary: Aequitas limits SI breadth to current Atlas
consumers, owns the scalar vocabulary through Eunomia, and exposes one
`Quantity<T, D>` API instead of `uom`'s closed storage-specific modules. `uom`
remains a dev-only differential oracle; no production dependency.

### Scalar-operator ergonomics gap

Consumers could only write `quantity * scalar`; the scalar-left form, in-place
`MulAssign`/`DivAssign`, and the complex-phasor equivalents were missing.
Resolved upstream as PR #21 (commit `dd0b8e1`, merge `0052b80`): commutative
scalar-left multiplication for `f32`/`f64`, compound assignment on real and
complex quantities, with 9 value-semantic tests (f32 and complex-division
paths completed in the rational-power increment).

### Rational-power dimension gap

`uom` cannot express fractional dimensions generically, so `sqrt`/`cbrt` could
not carry the halved/thirded dimension. Resolved by `SqrtDimension`/
`CbrtDimension` with concrete exponent-tuple impls plus `Quantity::sqrt`/
`cbrt` through the `FloatElement` scalar surface (commit `72ef8b4`; 12
value-semantic tests). The eunomia-owned sign-preserving `cbrt`
(`FloatElement::cbrt`, `libm::cbrtf`/`libm::cbrt`) has since landed as the
scalar-math SSOT and `Quantity::cbrt` now uses it (dropping the `powf(1/3)`
path). Semantics-marked variants (`Angle::sqrt` → dimensionless,
`ReciprocalVolume::cbrt` → reciprocal length) now compile with
`BaseSemantics`-normalized output; no open rational-power gap remains.

## Deferred (documented boundary — see backlog.md)

- Affine unit kinds and quantity kinds beyond the linear-unit slice.
- Integer and rational quantity storage.
- Formatting breadth beyond `UnitDisplay`.

Each is gated on a driving Atlas consumer; no consumer currently needs them.

## uom gap analysis

[`uom` 0.38.0](https://docs.rs/uom/0.38.0/uom/) is the comparison baseline and
remains a development-only differential oracle.

| Capability | `uom` 0.38.0 | Aequitas 0.2 scope |
| --- | --- | --- |
| Compile-time dimensional analysis | Mature, broad implementation | Required; implemented through one generic dimension algebra |
| Type-level rational/integer powers | Integer exponents only; `sqrt`/`powi` hardcoded per quantity | `SqrtDimension`, `CbrtDimension`, and generic `PowDimension<P>` raise the dimension itself at the type level (`powi::<P2>` of `Length` is `Area`) |
| SI and non-SI breadth | Extensive | Deliberately limited to current Atlas consumers |
| Storage types | Closed macro-generated set of primitive, integer, rational, and complex types | Real quantities over Eunomia's `UnitScalar` implementations; complex phasors over `eunomia::Complex32`/`Complex64` |
| Atlas datatype SSOT | Uses `num-traits` storage contracts | Uses Eunomia directly; defines no scalar vocabulary |
| API variation | Generates storage-specific modules such as `si::f32` and `si::f64` | One `Quantity<T, D>` API with inferred or defaulted `T` |
| `no_std` | Supported | Supported |
| Affine units and quantity kinds | Supported | Not in the initial linear-unit slice |
| Formatting and serialization | Supported | `UnitDisplay` (unit-aware `Display`/`Debug`) and optional serde (canonical scalar) supported |
| Integer and rational storage | Supported | Outside the floating-point simulation boundary |

The architectural decision and source-level comparison are recorded in
[ADR 0001](docs/adr/0001-aequitas-quantity-law.md).

## Verified non-gaps (do not chase)

- **No imaginary SI dimension** — complex phasors carry one observable unit
  for real and quadrature components; `eunomia::Complex::i()` is a numerical
  imaginary unit, not a physical unit.
- **Surface tension ≠ energy-per-area** — semantically distinct quantities
  despite equal base exponents (ADR 0004); same for specific energy vs
  absorbed dose (ADR 0012) and reciprocal volume vs number density (alias).
- **Temperature difference is affine** — absolute minus absolute yields a
  difference; difference plus absolute yields absolute (ADR 0003).
- **ZST/transparent law** — dimension and unit markers are zero-sized;
  `Quantity<T, D>` has the size and alignment of `T`; proven by layout tests.
- **no_std** — the crate builds and checks with `--no-default-features`; the
  optional `serde` and `std` features are additive.

## Current verified state (2026-08-12)

- Strict all-targets check: pass (warning-denied).
- Nextest: 59/59 (default features) at the provider head; all-feature gate
  re-verified in the Atlas foundation gate sweep.
- Doctests and rustdoc: pass; `cargo deny check`: clean.
- No `TODO`/`FIXME`/`unimplemented!` markers remain in `src/`.
