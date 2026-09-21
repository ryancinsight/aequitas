# Aequitas backlog

<a id="AEQ-UNIT-COMPOSITION"></a>
## AEQ-UNIT-COMPOSITION — Compose linear units [minor] [arch]

- status: review; integrator: codex-unit-composition
- Outcome: product, quotient and integer-power units convert through normalized dimensions.
- Scope: Rust unit contracts, SI marker dimensions, conversion, display, binding conversion parity, tests and documentation.
- Non-goals: runtime parsing, Python expression API, automatic catalog-name formatting.
- Acceptance: named/composed equality, scaled/nested/unnamed expressions, scalar-generic behavior and compile-time dimension rejection.
- Decision: [ADR 0018](docs/adr/0018-unit-composition.md).
- Verification: 252 workspace tests, doctests, no-std, clippy, rustdoc, generated-surface freshness and SemVer checks pass; independent review passes.
- Baseline: 147 core tests pass at 812f35e with pre-existing local Cargo.lock changes.
- Dependency: [native inverse scaling](../eunomia/backlog.md#EUNOMIA-UNIT-DIVISION).

<a id="aeq-dimensioned-accepts-non-finite-2026-09-09"></a>

<a id="aeq-recurseml-permanently-red-2026-09-09"></a>

## AEQ-LAW-TREE-2026-09-21 — The law crate's test aggregate and additive kernel were horizontal [patch] — done 2026-09-21 <a id="aeq-law-tree-2026-09-21"></a>

- **Outcome:** `tests/dimension_laws.rs` (439 lines) held 28 dimensional
  identities in one scope spanning seven unit domains, so a thermal-coefficient
  law sat beside a hydraulic one and the file was the member's largest. The
  three `#[path]` leaves beside it (`dosimetry` 81, `angle` 57, `complex` 44)
  already showed the intended grain: one leaf per subject, six tests or fewer.
  `src/quantity/arithmetic/additive.rs` (169) had the same defect on the
  production side -- it held the dimension-generic `Add`/`Sub` kernel and its
  private witness trait together with the nine affine temperature impls, the
  one case where addition changes the dimension it started from.
- **Acceptance:** one leaf per SI unit domain, mirroring
  `systems::si::units::{geometry,kinematics,mechanics,thermal,transport,electrical,radiation}`;
  every test function preserved by name and body; the generic kernel and the
  affine laws separated; the gate green.
- **Non-goals:** `src/systems/si/{quantities,dimensions}.rs` and
  `src/systems/si/units/scaled.rs` -- see the limits below.
- Implemented:
  `tests/dimension_laws/{kinematics,mechanics,thermal,transport,hydraulics,electrical,radiation}.rs`,
  with the three complex-valued identities moved into `complex.rs` whose
  charter already owned them and the aggregate reduced to `#[path]` wiring;
  `src/quantity/arithmetic/affine.rs`, and `additive.rs` reduced to the kernel.
- Evidence: 40 test functions identical by name before and after (28 in the
  aggregate + 6/3/3 in the leaves), none added and none removed; nextest
  239/239 unchanged; `python scripts/generate-surface.py check` reports all
  four generated artifacts `current`; fmt, all-targets all-feature Clippy with
  `-D warnings`, doctests, `RUSTDOCFLAGS=-D warnings` and
  `--no-default-features` are green.
- Limits: the two files the generator reads in place cannot be split.
  `collect_quantities` reads *only* `systems/si/quantities.rs` **and in source
  order**, so a leaf split would both empty the generator's input and silently
  reorder the generated inventory; `collect_dimension_types` reads *only*
  `systems/si/dimensions.rs`. Unit files are free by contrast: `collect_units`
  and `collect_affine_units` glob `units/**/*.rs` recursively and sort what
  they find, so unit leaves are order-independent. `scaled.rs` was left whole
  deliberately, on the evidence in `gap_audit.md`'s non-gap list rather than on
  its size alone.

## AEQ-PY-TESTS-SPLIT-2026-09-21 — The quantity test file crossed the structural target [patch] — done 2026-09-21 <a id="aeq-py-tests-split-2026-09-21"></a>

- **Outcome:** `crates/aequitas-python/src/quantity/tests.rs` (576 lines) trips
  the stack's `oversized_files` class, which holds the member's pin behind the
  atlas sweep.
- **Acceptance:** no Rust file in the member exceeds the 500-line target; all 35
  tests remain, each body unchanged; the gate is green.
- **Non-goals:** the generated `.pyi` stub (5,196 lines, not scanned) and any
  behavioural change to the binding.
- Closed in `83e7293` (`test(python): Split quantity tests by contract`),
  merged as [#75](https://github.com/ryancinsight/aequitas/pull/75) (`e129d80`
  on `main`). Verified at that head, independently of the commit message: the
  576-line file is gone and the member's largest Rust file is
  `tests/dimension_laws.rs` at 439, so the 500-line class is at zero; fmt,
  `clippy -D warnings` over `--workspace --all-targets --all-features`, nextest
  (239/239), doctests and rustdoc are all green. Limits: the merge landed the
  *test-tree* half only -- the two commits after it are unlanded, and the
  recorded atlas pin trails member `main` by more than that; advancing a
  gitlink is the sweep's step, not this item's.

## AEQ-PY-TEST-TREE-2026-09-21 — The remaining binding test modules were horizontal, not contract-shaped [patch] — done 2026-09-21 <a id="aeq-py-test-tree-2026-09-21"></a>

- **Outcome:** `tag/tests.rs` (220 lines, 18 tests), `units/tests.rs` (348, 15)
  and `consumer/tests.rs` (386, 18) each hold several contracts in one scope, so
  a claim and the fixture that certifies it sit apart: `units/tests.rs` keeps
  the conversion sweep where the sweep-input fixture is defined, and
  `consumer/tests.rs` keeps the finiteness policy, the structural read and the
  `Dimensioned` parameter together. None of the three trips `oversized_files`,
  so no instrument reported it -- the defect is the shape, not the count.
- **Acceptance:** one leaf per contract, each mirroring the seam it covers;
  shared fixtures defined once; every test function preserved by name and body;
  the gate green.
- **Non-goals:** the production modules (no behaviour or public-surface change)
  and the generated `.pyi` stub.
- Implemented: `tag/tests/{derivation,semantics,algebra,rendering}.rs`,
  `units/tests/{fixtures,inventory,lookup,conversion,affine}.rs` and
  `consumer/tests/{fixtures,dimensioned,read,finiteness}.rs`. Evidence: the
  51 test functions are identical by name before and after (18/15/18); nextest
  239/239; `clippy -D warnings` over `--workspace --all-targets --all-features`;
  fmt, doctests and rustdoc green. Limits: this is test-module restructuring, so
  it adds no coverage and changes no behaviour; the pytest suite was not run
  here (this checkout has no built wheel).

## AEQ-PY-SEAM-2026-09-21 — The binding's production modules were horizontal, and no seam had a measured cost [patch] [perf] — done 2026-09-21 <a id="aeq-py-seam-2026-09-21"></a>

- **Outcome:** `tag/model.rs` (223 lines) held the tag's identity, its algebra
  and its two failure types in one scope, so the seven-element exponent loop
  sat beside the rendering with nothing saying which the file was about;
  `quantity/model.rs` (219) held the class declaration, the unit-resolution
  constructors, every getter and the cross-extension wire form. Neither is one
  contract: both are one value or one class whose `#[pymethods]` surface is
  several.
- **Acceptance:** one leaf per contract; private fields reachable only through
  their accessors; the declaration surface preserved exactly; the gate green.
- **Non-goals:** the generated `units/inventory.rs` and
  `quantity/classes/inventory.rs`, the `.pyi` stubs, and any behavioural
  change.
- Implemented: `tag/{model,algebra,error}.rs` and
  `quantity/{model,construct,inspect,wire}.rs`. The crate already opts into
  `pyo3`'s `multiple-pymethods` "so neither file approaches the structural size
  target", so this is the documented intent rather than a new pattern.
- Evidence: 71 function definitions before and after the split -- none added,
  none removed, names and bodies unchanged (splitting a value across sibling
  leaves does make direct private-field access illegal, so the leaves reach the
  magnitude and the tag through the existing accessors); 239/239 tests pass,
  and fmt, `clippy -D warnings` over `--workspace --all-targets --all-features`,
  doctests, `RUSTDOCFLAGS=-D warnings` and `--no-default-features` are green.
  `cargo doc -D warnings` failed once mid-work because a public module doc
  linked its private leaves; the links were dropped, not the lint.
- The one codegen fix here is `DimensionTag::combine`, which took
  `op: fn(i8, i8) -> Option<i8>` and so paid an indirect call per axis through
  a loop the optimizer could not see into. It is generic and monomorphizes
  now. **No speedup is claimed:** measured on a temporary release-mode probe
  (counting global allocator, deleted after use), `tag_multiply` is 0.000
  allocs/call at 1.70 ns before and 1.79 ns after -- a seven-element loop of
  checked additions leaves no room for an indirect call to show, so the change
  stands on the abstraction being zero-cost, not on a number.

## AEQ-PY-READ-COST-2026-09-21 — A Python-passed quantity costs 462 ns and two heap allocations per argument [perf] [minor] <a id="aeq-py-read-cost-2026-09-21"></a>

- **Outcome:** `consumer::value::read` is the path a Python-passed quantity
  takes through a consumer's `Dimensioned<D>` parameter: `consumer` cannot
  downcast to `PyQuantity` -- its own doc requires that a consumer which never
  registers the class must not instantiate its type object -- so it reads the
  object structurally instead. Measured in release mode: `read` **315 ns and
  2.000 allocations per call**, `Dimensioned::extract` **462 ns and 2.000
  allocations per argument**. The law crate allocates nothing at all: `src/`
  contains no `Box`, `Vec`, `String` or `format!`, so the whole cost is at the
  boundary.
- **Acceptance:** the read path is allocation-free, or the alternative's cost
  is measured before and after in release mode with the pytest suite run
  against a built wheel.
- **Non-goals:** the input set the wire form accepts, the abi3 floor, and the
  two-wheel interoperability guarantee.
- Every reduction was attempted and refused, and each reason is recorded
  in-source on `read` rather than left implicit:
  - *Borrow the marker instead of copying it* -- `PyStringMethods::to_str` is
    gated on `any(Py_3_10, not(Py_LIMITED_API))` and this crate builds
    `abi3-py38`, so the borrowed form does not exist; `to_cow` and
    `to_string_lossy` both fall back to an owned copy under the limited API.
    Borrowing needs the abi3 floor dropped: a distribution decision.
  - *Replace the exponent `Vec<i64>` with a stack array* -- `Vec` accepts any
    Python sequence of integers, a fixed-size array only its own shape, so
    this narrows what the protocol admits. Behavioural, and the suite that
    would justify it needs a wheel this checkout does not build.
  - *Cache the `((int x 7), str)` tuple* -- would cut the getter's per-read
    work, but adds shared mutable state to a crate that ships for
    free-threaded CPython with `gil_used = false`. Needs an ADR.
  - *Invert the dependency to give `consumer` a fast path* -- a hook
    installed at module init keeps `PyQuantity` out of the consumer's
    signatures, but it is the coupling that module forbids by construction.
- Not a gap, and recorded so it is not re-measured: the linear scans around
  this path are free. `units::by_tag` is **1.24 ns/call** and the scan
  `Quantity::in_unit` performs is **7.56 ns/call**, both under 3% of one
  `read`, so no lookup index is warranted.

## AEQ-PY-FREE-THREADED-2026-09-11 — The binding re-enables the GIL on a free-threaded interpreter [minor] — done 2026-09-11 <a id="aeq-py-free-threaded-2026-09-11"></a>

- [#71](https://github.com/ryancinsight/aequitas/pull/71), merge `ae321ff`: `gil_used = false` on an audited module;
  the bindings matrix adds 3.14t (PyO3 refuses 3.13t), where a fresh interpreter reports the GIL off
  after import. `Python bindings (free-threaded)` is now a required check.

## AEQ-PY-SCALAR-TAG-2026-09-11 — Scaling a quantity by a number dropped its semantic marker [patch] — done 2026-09-11 <a id="aeq-py-scalar-tag-2026-09-11"></a>

- Fixed in [#69](https://github.com/ryancinsight/aequitas/pull/69), merge `98446f9`:
  scalar `*` and `/` keep the tag; Rust tests pin the law crate's `Stress<f64>`
  result types and fail against the old code.

## AEQ-CI-BINDING-GATES-2026-09-11 — CI never compiled, linted or tested the binding crate in Rust [patch] [ci] — done 2026-09-11 <a id="aeq-ci-binding-gates-2026-09-11"></a>

- Fixed in [#67](https://github.com/ryancinsight/aequitas/pull/67), merge `16fefc2`:
  `verify` runs clippy, nextest, doctests and rustdoc with `--workspace`; the
  PR's `verify` log (run 34613878654) lists 85 `aequitas-python` test passes.

## AEQ-PY-OPERAND-FLOAT-2026-09-11 — Arithmetic read a protocol quantity with `__float__` as a scalar [patch] — done 2026-09-11 <a id="aeq-py-operand-float-2026-09-11"></a>

- Fixed in #67: `Operand::parse` reads the protocol before `__float__` via
  `carries_protocol`; Rust and pytest regressions fail against the old order.
  Quantity products went from 205 ns to 41 ns, the discarded TypeError gone.

## AEQ-SURFACE-STALE-AFTER-64-2026-09-11 — #64 left the binding surface stale and main red [patch] [ci] — done

- Merged in #65 (2026-09-11 14:11 UTC), held by the new gate until `verify`,
  `Python bindings`, `supply-chain` and `Lockfile integrity` passed. Cause: #64
  added `LinearUnit` impls without regenerating, ran only the law crate's gate,
  and merged before protection existed. The pre-push hook still lacks
  `generate-surface.py check`; carried to the stack-level hook item rather than
  patched here, since that campaign is replacing member hooks.

## AEQ-GITDIR-GUTTED-2026-09-10 — The aequitas gitdir lost its metadata and recent objects overnight [patch] — done

- **Found 2026-09-11 09:50:** `repos/aequitas` reported "not a git repository".
  The pointer was correct; `.git/modules/repos/aequitas` held only `objects/`
  and `refs/` -- `HEAD`, `config`, `index`, `logs/`, `packed-refs` gone, and
  every loose object written after 17:07 the day before deleted with them
  (35 missing, including `HEAD`'s tree). Directory mtime 22:50; no other member
  was touched. Cause not determined from the tree.
- **Nothing unique was lost.** Surviving `refs/heads/main` equalled
  `origin/main` (`9cdf1d7`), and after repair the working tree is identical to
  `HEAD`. The damaged store's only non-origin content was dangling editor
  checkpoints and a stale copy of `crate-aequitas-v0.2.0` from before origin
  moved it from the #16 merge to the #17 lock-fix merge on 2026-08-09.
- **Repair:** a no-negotiation refetch into the corrupt store recovered only
  part of it, so the gitdir was rebuilt from a fresh clone of origin (fsck
  clean), re-pointed at the working tree, and the index rebuilt from `HEAD`.

## AEQ-MERGE-GATE-2026-09-10 — main took merges with no verification [patch] [ci] — done

- **Found by merging into it.** #64 was enqueued with `--auto` and merged
  **immediately**, before a single check reported: `main` had no protection, so
  every workflow here was advisory. This is the defect kwavers closed as
  KW-CI-115 after the same discovery.
- **Applied 2026-09-10.** Four required contexts, all from `ci.yml`:
  `verify` (50s), `Python bindings` (51s), `supply-chain` (42s),
  `Lockfile integrity / Lockfile integrity` (15s). The whole gate is about a
  minute, so it is faster than the work it gates and will not be routed around.
- **Excluded:** `SemVer (informational)` is informational by name;
  `SemVer (release gate)` skips on pull requests; `CodeRabbit` and
  `recurseml/analysis` are third-party and the latter is permanently red
  (AEQ-RECURSEML-PERMANENTLY-RED-2026-09-09).
- **No deadlock risk here**, unlike kwavers: `ci.yml`'s `pull_request:` trigger
  carries no path filter, so every pull request runs it and a required check
  always reports. `enforce_admins` stays false so board commits can still reach
  `main` directly, which is how this file is maintained.
- **`strict: false`** deliberately: requiring branches be up to date forces a
  full re-run whenever `main` moves, and the queue is the stack's scarce
  resource.
- Repository mechanics were already correct: auto-merge on, squash off, merge
  and rebase allowed.

## AEQ-RECURSEML-PERMANENTLY-RED-2026-09-09 — The recurseml check errors on every pull request [patch] [ci] — blocked

- **Measured 2026-09-09:** `recurseml/analysis` reports ERROR on #58, #59, #60
  and #61 — every recent pull request. It is a third-party GitHub App, not a
  repository workflow, so no committed job produces or can fix it.
- **Why it matters:** a check that is red on every pull request whatever the
  change trains readers to skim the check list, which is the same
  desensitisation the SemVer informational gate was fixed for. It also makes
  "one failing check" a useless signal for the PR-watching automation.
- **Blocker:** removing or reconfiguring a GitHub App installation is an
  account/permission change, outside the merge-mechanics grant. The user
  uninstalls the app or disables its checks.
- **Re-open trigger:** the app is removed, or it starts reporting a real
  verdict.
- Same class as kwavers `KW-CI-094`; if a third member shows it, this belongs
  on the meta board as one fleet item rather than per-member copies.

## AEQ-DIMENSIONED-ACCEPTS-NON-FINITE-2026-09-09 — The extractor forwarded NaN and infinity [patch] — done 2026-09-09

- **Found by review on kwavers#726:** `Dimensioned::extract` checked neither
  arm for finiteness, so a consumer's `pmut_self_heating` multiplied an
  extracted drive voltage and returned a non-finite power.
- **Delivered:** both arms check. `Dimensioned<D>` is finite-only;
  `Dimensioned<D, MayBeInfinite>` admits `±inf` for a parameter that publishes
  it as a sentinel (kwavers' `set_focus_distance` documents `INF` for "no
  focusing", so a blanket rejection would break a published contract). `NaN` is
  rejected under both. Recorded as a revision on
  [ADR 0016](docs/adr/0016-python-quantity-binding.md).
- **Verified:** six tests cover NaN and infinity on the float arm, NaN on the
  protocol arm, the sentinel under `MayBeInfinite`, NaN still rejected there,
  and an ordinary magnitude unaffected; workspace 217/217, fmt and
  warning-denied clippy clean.

## AEQ-EUNOMIA-IDENTITY-2026-09-03 — Unify Eunomia source identity [patch] — done 2026-09-04 <a id="aeq-eunomia-identity-2026-09-03"></a>

- [x] PR #51 merged at `e61a3cf`; Eunomia resolves from the canonical default
  source at `2723759`; lockfile, strict Clippy, Nextest 127/127, doctests,
  rustdoc, no-default, and lockfile checks pass.

Strategic roadmap; tags `[patch]`/`[minor]`/`[major]`/`[arch]` per SemVer class.
Aequitas is the Atlas physical-quantity and dimensional-law SSOT: type-level SI
dimensions, transparent quantities over Eunomia scalars, and linear SI unit
conversion, consumed by proteus, hyperion, kwavers, CFDrs, helios, and the
domain layer.

## AEQ-PY-BINDING-001 — Publish the quantity surface as a Python wheel [arch][minor] — blocked <a id="aeq-py-binding-001"></a>

- **Decision:** [ADR 0016](docs/adr/0016-python-quantity-binding.md).
- **Outcome:** a PyPI wheel exposing every SI quantity and unit with
  conversion and dimensional arithmetic, consumable from `kwavers-python`.
- **Names:** distribution `aequitas-python`, import package `pyaequitas`;
  plain `aequitas` is taken on PyPI by an unrelated project whose top-level
  import would collide.
- **Landed:** the `aequitas-python` member; the runtime tag derived from the
  `Dimension` parameters; generated inventory and stubs; `Dimensioned<D>` as
  the consumer parameter type; 74 Rust tests. The consumer side landed too --
  kwavers converted 93 dimensioned parameters over 8 files (kwavers#726).
- **Correction:** `Dimensioned<D>` tried `f64` first, and that honours
  `__float__`, which quantity types define (pint's does), so a declared time
  was accepted as a length with its dimension never checked. Fixed in #58;
  the protocol is read before the float arm.
- **Integrator:** claude-opus-5, taken over 2026-09-11 (the item had none), lane
  `worktrees/aequitas-python-typing`. PyPI name re-checked: `aequitas-python`
  is unregistered (404), `aequitas` taken (200).
- **Pipeline landed** in [#72](https://github.com/ryancinsight/aequitas/pull/72) (`d81c9fb`): `python-release.yml`
  builds 8 wheels (abi3 and 3.14t on four native runners) plus an sdist, each smoke-tested;
  its PR run 34618118204 passed every build job. Affine units and typed classes landed too.
- **Blocker:** the PyPI trusted publisher is not registered (project `aequitas-python`,
  owner `ryancinsight`, repo `aequitas`, workflow `python-release.yml`, environment `pypi`);
  that is a pypi.org account action for the owner.
- **Re-open trigger:** the publisher registered; then a release tagged
  `aequitas-python-v0.1.0` publishes and the `published` job installs it back.
- **Acceptance:** every exported dimension round-trips against
  `Quantity::in_unit`; semantic normalization matches
  `MultiplyDimension`/`DivideDimension`; codegen regenerate-and-diff clean;
  `pytest` green against the built wheel.

## AEQ-PY-WHEEL-MACOS-X86-2026-09-11 — The wheel matrix has no Intel macOS wheel [patch] [ci] — todo <a id="aeq-py-wheel-macos-x86-2026-09-11"></a>

- **Gap:** `python-release.yml` builds and smoke-tests on four native
  runners; Intel macOS is absent, so pip there falls back to the sdist and
  needs a Rust toolchain.
- **Open question:** which GitHub-hosted label provides x86_64 macOS now, or
  whether the wheel cross-builds on arm64 and smoke-tests under Rosetta.
- **Acceptance:** an x86_64 macOS wheel per ABI, smoke-tested by running the
  Python suite on x86_64, in the same matrix.
- **Found by:** [AEQ-PY-BINDING-001](#aeq-py-binding-001), PR #72.

## AEQ-RELEASE-TAG-FORM-2026-09-11 — The crate release workflow keys on a `crate-` tag prefix [patch] [ci] — done 2026-09-11 <a id="aeq-release-tag-form-2026-09-11"></a>

- [#73](https://github.com/ryancinsight/aequitas/pull/73) (`c322c77`): ADR 0017 records independent versioning;
  `rust-release.yml` gates on `aequitas-v` and parses `<package>-v<version>`.

## AEQ-RELEASE-EUNOMIA-CBRT-001 — Aequitas cannot be packaged for crates.io [patch] — blocked <a id="aeq-release-eunomia-cbrt-001"></a>

- **Symptom:** `cargo package --locked -p aequitas` fails to verify the
  tarball with `E0599: no method named cbrt found for type parameter T` at
  `src/quantity/root.rs:110`.
- **Cause:** packaging strips the git source from the `eunomia` dependency, so
  the verification build resolves `eunomia 0.8.0` from crates.io.
  `FloatElement::cbrt` exists on eunomia's default branch but not in that
  release: the published `src/impls/field.rs` carries `sqrt` and no `cbrt`.
- **Not caused by the binding work:** the commit that surfaced this touched
  neither `src/` nor the `eunomia` requirement.
- **Blocker:** eunomia must publish a release carrying `cbrt`, which is a
  release action outside this repository's authority.
- **Re-verified 2026-09-11:** crates.io's newest eunomia is 0.8.0, whose
  `src/impls/field.rs` still has no `cbrt`; the blocker stands.
- **Re-open trigger:** a crates.io eunomia release containing
  `FloatElement::cbrt`; then advance the requirement and re-run
  `cargo package --locked -p aequitas`.

## AEQ-PY-TYPING-001 — Per-quantity classes so a type checker sees dimensions [minor] — done 2026-09-11 <a id="aeq-py-typing-001"></a>

- [#68](https://github.com/ryancinsight/aequitas/pull/68) (`9a7034d`): 73 classes, one per distinct dimension;
  [#70](https://github.com/ryancinsight/aequitas/pull/70) (`0a20498`): overloads carry the tag algebra, so mypy
  infers `Area` and rejects `length + time`; the stub is checked against the runtime both ways.

## AEQ-PY-AFFINE-001 — Affine units for the thermal surface [minor] — done 2026-09-11 <a id="aeq-py-affine-001"></a>

- Contract in #64; binding in [#66](https://github.com/ryancinsight/aequitas/pull/66),
  merge `ee91281`: `degC` and `degF` resolve per quantity from `pyaequitas`,
  affine for a temperature and linear for a difference.
- #66 also made the binding's inverse bitwise the law crate's (it divided by
  the scale: 24 of 89 units off by an ulp, now swept over 4,107 values).

## AEQ-DOC-BOOK-001 — Execute book samples [patch] — done 2026-09-11 <a id="aeq-doc-book-001"></a>

- Hosted `mdbook test` passed at `b77fe83` (run 34611215330, `deploy / Build
  book`); all 11 chapters also pass locally against artifacts staged from
  cargo's JSON output. Escaped defect recorded in [gap_audit.md](gap_audit.md).

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
  slice (`uom`-style `Kind` system). The temperature slice landed in
  [AEQ-PY-AFFINE-001](#aeq-py-affine-001); the
  general `Kind` system stays deferred.
- [ ] [minor] Integer and rational quantity storage. The simulation boundary
  is floating-point over Eunomia scalars; revisit on a consumer need.
- [ ] [patch] Broader formatting surface beyond `UnitDisplay` (unit-algebra
  pretty-printing, `Display` for dimension/unit markers). Currently outside
  the boundary.
