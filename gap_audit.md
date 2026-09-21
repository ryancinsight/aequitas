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

## Escaped defects

- **A path filter ignored an input of the job it gates.** The book workflow
  ran on `docs/book/**` while its samples compile against `src/`; 82ec9b5 gave
  `Kilopascal` a second `LinearUnit`, the chapter failed E0283, and nothing ran
  for eight days. Check: 08f687c triggers on `src/**` and the manifests.
  Pattern: a `paths` filter covers the dependency closure of its job.
- **A bitwise claim tested on round numbers.** The binding's conversion test
  used seven hand-picked values and passed while 24 of 89 units differed from
  the law crate by an ulp. Check: the seeded 4,107-value sweep, introduced in
  `crates/aequitas-python/src/units/tests.rs` (3f93d67) and now in
  `units/tests/{fixtures,conversion}.rs`, shown to fail against the old
  division. Pattern: an equality claim over a domain needs a sweep.
  That sweep ran only locally until #67 (next entry).
- **A gate scoped by a comment, not by its commands.** `Cargo.toml` said gates
  covering the binding pass `--workspace`; no CI step did, and
  `default-members` kept them on the law crate. Check: #67 adds `--workspace`
  and the PR's `verify` log lists the binding's tests. Pattern: confirm a
  crate's tests appear in the CI run before naming them as a guard.

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

### Horizontal files split into leaves

The binding's test surface was horizontal: one `tests.rs` per module holding
every contract for that module. `units/tests.rs` kept the conversion sweep, the
sweep-input fixture and the affine offset contract in one scope, so the fixture
that certifies the oracle sat beside the claim it certifies rather than beneath
it; `consumer/tests.rs` kept the finiteness policy, the structural read and the
`Dimensioned` parameter together, which is how a `NaN` arm and a sentinel arm
that must never unify end up ordered by nothing. A 576-line
`quantity/tests.rs` was the same shape and crossed the stack's 500-line
structural target, which is what made the shape visible at all -- the other
three are 220, 348 and 386 lines, so no instrument reported them.

Resolved by splitting each file into one leaf per contract, with shared
fixtures held once: `quantity/tests/{arithmetic,comparison,construction,protocol,fixtures}`
(`83e7293`), `tag/tests/{derivation,semantics,algebra,rendering}`,
`units/tests/{inventory,lookup,conversion,affine,fixtures}` and
`consumer/tests/{dimensioned,read,finiteness,fixtures}`. Evidence: 51 test
functions identical by name before and after the second split (18/15/18);
nextest 239/239; `clippy -D warnings` over `--workspace --all-targets
--all-features`; fmt, doctests and rustdoc green; no Rust file in the member
exceeds 500 lines. Limits, stated rather than implied: this is test-module
restructuring, so it adds no coverage, changes no behaviour and does not touch
the generated surface; the pytest suite did not run in this checkout because no
wheel is built here, so the Python-side tests are unverified by this increment.
The test-tree half landed as
[#75](https://github.com/ryancinsight/aequitas/pull/75) (`e129d80`).

The production modules had the same shape and were split the same way:
`tag/model.rs` (223 lines) and `quantity/model.rs` (219) became
`tag/{model,algebra,error}` and `quantity/{model,construct,inspect,wire}`, with
`quantity/mod.rs` naming the four seams. Two mechanics are worth reusing. A
public module's doc may not link its private leaves -- `cargo doc -D warnings`
fails on it -- so the module doc names them in plain text and the links stay in
the leaves. And because field privacy is module-scoped, moving an `impl` into a
sibling leaf makes direct private-field access illegal; the leaves reach the
value through the accessors that already existed, rather than widening the
fields to `pub(super)` and giving up the invariant. Evidence: 71 function
definitions identical before and after, no generated surface touched, the same
gate set green.

The law crate had the same shape on both sides. `tests/dimension_laws.rs`
(439 lines) held 28 identities spanning seven unit domains in one scope, so a
thermal-coefficient law was ordered by nothing but the sequence in which it was
written; the three `#[path]` leaves already beside it (`dosimetry` 81, `angle`
57, `complex` 44) showed the intended grain. It is now one leaf per domain --
`kinematics`, `mechanics`, `thermal`, `transport`, `hydraulics`, `electrical`,
`radiation` -- with the three complex-valued identities moved into `complex.rs`,
whose charter already owned the value-kind axis, and with the aggregate left as
module wiring. On the production side,
`quantity/arithmetic/additive.rs` (169 lines) held the dimension-generic
`Add`/`Sub` kernel together with the nine affine temperature impls; it is now
`additive.rs` (kernel plus its private `BaseAdditiveDimension` witness) and
`affine.rs` (the one place where addition changes the dimension it started
from, documented as such in the leaf). Evidence: 40 test functions identical by
name, none added and none removed; nextest 239/239; the generator's `check` mode
reports all four artifacts current; the same gate set green. Limits: this is
restructuring, so it adds no coverage and changes no behaviour, and the Python
side remains untested here as before.

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

## Measured cost baseline (2026-09-21)

Release-mode probe over the binding's dimension seam, temporary and not
committed: a counting `GlobalAlloc` for the allocation columns, and a
deterministic loop with `black_box` for the time columns. Method limits: one
machine, one run per figure, and no statistical treatment -- read the numbers
as magnitudes, not as benchmarks. The run-to-run spread observed across the
three passes was 1.70-1.79 ns (`tag_multiply`), 296-315 ns (`read`) and
448-462 ns (`dimensioned`), so differences under ~5% are noise.

| Path | Rust heap allocations | Time |
| --- | --- | --- |
| `DimensionTag::multiply` | 0.000 /call | 1.7-1.8 ns/call |
| `consumer::read` (structural, 2 attribute lookups) | 1.000 /call (was 2.000) | 296-315 ns/call |
| `Dimensioned::<Length>::extract`, native quantity | 1.000 /call (was 2.000) | 448-462 ns/call |
| `units::by_tag` | 0.000 /call | 1.24 ns/call |
| the scan `Quantity::in_unit` performs | 0.000 /call | 7.56 ns/call |
| law crate `src/` | none -- no `Box`, `Vec`, `String` or `format!` | -- |

The allocation column was re-measured; the time column was not. A second probe
run took both read paths from 2.000 to 1.000 allocations per call -- an
allocation count does not depend on the build profile, unlike a timing -- and
reported 1080 -> 909 ns for `read` and 1274 -> 1097 ns for `Dimensioned` **in
the debug profile**, where the release figures in the table do not apply.

Conclusion: the boundary is where the money is, and it is the *structural read*
that spends it, not the scans around it. One of its two allocations is now gone,
and the one that remains is the semantic marker's name, which is the price of
the `abi3-py38` floor rather than an oversight. `AEQ-PY-READ-COST-2026-09-21`
carries the rest; the scans are closed as not worth an index, at under 3% of one
`read` between them.

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
- **Linear tag lookups are not a cost** — measured 1.24 ns for `units::by_tag`
  and 7.56 ns for the scan `Quantity::in_unit` performs, against 296-315 ns for
  one structural read. Seven of the 81 quantities share the first axis, so the
  filter rejects almost every entry on one comparison; a sorted index or a hash
  would buy nothing and add a second ordering to keep in step with the
  generated inventory.
- **The two `fn`-pointer tables in `quantity::classes::model` are not
  removable indirection** — a `const` table cannot hold closures, and each
  entry is already a distinct monomorphized function per generated class, so
  the pointer is the table's representation, not an abstraction cost. The
  removable one was `DimensionTag::combine`'s operation parameter, now generic.
- **The binding has no `dyn`** — `crates/aequitas-python/src` contains no
  trait object; the generated class table is `&'static [Class]` of statically
  known functions.
- **The tag's exponent vector was removable, and pyo3's array extractor was
  not the way to do it** — unpacking the tag as `(Vec<i64>, String)` built a
  seven-element vector per read only to copy it into `[i8; AXES]` and drop it,
  and taking the two members as objects removes that allocation without changing
  what the protocol admits. `([i64; AXES], String)` is the shorter edit and was
  rejected: pyo3's array extractor goes through `PySequence_Check`, which would
  narrow the admitted input from *any iterable* to a sequence — silently, for a
  tag shape this crate does not produce but does accept.
  `read_rejects_a_malformed_tag` now pins both directions of the arity check
  (`exactly 7 exponents, got 9` as well as the short case) plus the
  out-of-range and unknown-semantics messages, so the diagnostics the rewrite
  had to preserve are asserted rather than assumed.
- **Two law-crate source files are pinned by the generator, not by taste** —
  `scripts/generate-surface.py`'s `collect_quantities` reads *only*
  `src/systems/si/quantities.rs`, and returns `(alias, dimension)` pairs **in
  source order**; splitting it into leaves would empty the generator's input
  (`SystemExit`, "no quantity aliases matched") and reordering it would
  silently reorder the generated inventory. `collect_dimension_types` reads
  *only* `src/systems/si/dimensions.rs` in the same way. Unit files are the
  opposite and may be split freely: `collect_units` and `collect_affine_units`
  glob `src/systems/si/units/**/*.rs` recursively and sort the markers they
  find, so unit-leaf boundaries and file order do not reach the artifact.
  Evidence: `python scripts/generate-surface.py check` reports all four
  artifacts `current` after a production-side split.
- **`src/systems/si/units/scaled.rs` (226 lines) is not a split candidate** —
  it is 23 markers of one shape (doc comment, zero-sized struct, `Sealed`,
  `LinearUnit` impl), under the 500-line target, and the sibling
  `units/derived/` leaves do not encode a domain taxonomy a split could follow
  consistently: `MolePerCubicMeter` sits in `kinematics.rs`, while
  `KilogramPerCubicMeter`, `Watt` and `SquareMeterPerSecond` sit in
  `thermal.rs` and `PerMeter`, `SquareMeter` and `SquareMeterPerKilogram` in
  `geometry.rs`. A "by domain" split would either propagate those placements or
  introduce a second taxonomy inside one directory; a homogeneous list is
  cheaper to read than either, and `AEQ-STRUCTURE-002` already made the
  domain-leaf decision for the implementations that carry behaviour.

## Current verified state (2026-08-12)

- Strict all-targets check: pass (warning-denied).
- Nextest: 59/59 (default features) at the provider head; all-feature gate
  re-verified in the Atlas foundation gate sweep.
- Doctests and rustdoc: pass; `cargo deny check`: clean.
- No `TODO`/`FIXME`/`unimplemented!` markers remain in `src/`.

The 59/59 above is a default-feature count from an older head and understates
the workspace; superseded on that axis by the snapshot below.

## Current verified state (2026-09-21)

- Nextest: 239/239 (workspace, all features), 0 skipped, for the content this
  branch commits -- unchanged across the three leaf splits, each of which
  preserves its declaration set. The structural-read change is verified on the
  binding crate alone (92/92, up from 89 by the three malformed-tag cases that
  came with it), because this checkout concurrently held unrelated uncommitted
  work whose tests are no part of this record.
- Formatting, all-targets all-feature Clippy with `-D warnings`, doctests (28
  passed, 2 ignored) and `cargo doc` with `RUSTDOCFLAGS=-D warnings`: pass.
- `cargo check --no-default-features`: pass. Largest file in the law crate:
  231 lines (`tests/integer_powers.rs`), down from 439
  (`tests/dimension_laws.rs`); no file exceeds 500, and the member's largest
  file remains the generated binding `units/inventory.rs` at 278.
- Production leaves added in the binding increment:
  `tag/{model,algebra,error}` and `quantity/{model,construct,inspect,wire}`;
  `quantity/mod.rs` names the four seams so a reader knows which leaf answers
  which question.
- Leaves added in the law-crate increment: seven domain leaves under
  `tests/dimension_laws/`, the three complex-valued identities consolidated
  into the existing `complex.rs`, and `quantity/arithmetic/{additive,affine}.rs`
  separating the dimension-generic kernel from the affine temperature laws.
  The aggregate file is now module wiring, so the 40 identities still compile
  as one binary and no test count moves.
- Not run here: the pytest suite and the wheel build (no interpreter-side
  environment in this checkout), and `cargo deny` -- so supply-chain and
  Python-side claims are not evidenced by this snapshot.
