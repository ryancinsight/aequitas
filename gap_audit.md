# Aequitas gap audit

## Closed gaps

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

The `uom` 0.38.0 comparison table (README) shows the intentional boundary:
Aequitas limits SI breadth to current Atlas consumers, owns the scalar
vocabulary through Eunomia, and exposes one `Quantity<T, D>` API instead of
`uom`'s closed storage-specific modules. `uom` remains a dev-only
differential oracle; no production dependency.

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
`cbrt` through the `FloatElement` power surface (commit `72ef8b4`; 12
value-semantic tests). Deferred: eunomia-owned sign-preserving `cbrt` and
semantics-marked dimension tuples (see backlog.md).

## Deferred (documented boundary — see backlog.md)

- Affine unit kinds and quantity kinds beyond the linear-unit slice.
- Integer and rational quantity storage.
- Formatting breadth beyond `UnitDisplay`.

Each is gated on a driving Atlas consumer; no consumer currently needs them.

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
