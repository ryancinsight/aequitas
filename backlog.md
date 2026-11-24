# Aequitas backlog

<a id="aequitas-stable-qualification"></a>
## AEQUITAS-STABLE-QUALIFICATION — aequitas stable-release qualification [patch] — todo
- priority: verification; milestone: M-1.0; basis: `e28f4b022781`.
- Outcome: Freeze the quantity and unit boundary on the published Eunomia line. Follow the [release scope](docs/adr/0020-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `aequitas` are 0.5.1, then 0.6.0 if the fractional-dimension proposal lands, then 1.0.0. Verification alone can advance the current version.
- Representative manifest `aequitas 0.5.0`. Target 1.0.0.
- [ ] [patch] Run dimension laws, affine-temperature, scalar arithmetic and conversion properties on the candidate. Compare composed dimensions and converted values to the independent unit oracle, including values adjacent to a rounding boundary.
- [ ] [patch] Build fresh consumers with default features on and off and verify a single Eunomia type identity. Reconcile the fractional-dimension and Python spelling proposals ([#172](https://github.com/ryancinsight/aequitas/pull/172) and [#171](https://github.com/ryancinsight/aequitas/pull/171)) with the freeze before choosing the candidate.
- [ ] [major] Apply ADR 0020 to the stable Rust surface. Keep the Python wheel and proof-tool decision in M-ecosystem with their existing owners.
- [ ] [patch] **0.5.1.** Qualify the 0.5 API with an external Rust caller. For affine temperatures, subtract two absolute readings and check the interval in both units. Reject addition of two absolute readings at compile time. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.6.0 if the fractional-dimension proposal lands.** Resolve #172 before freeze. If admitted, compose fractional exponents and recover the original dimension after inversion. Otherwise carry the existing integer contract directly to beta.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. A default-off quantity consumer resolves one Eunomia identity and reproduces the unit and dimension laws. Repeat the applicable exits on the exact RC before stable promotion. Attach exact-revision evidence for the local exits and this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

- [minor] Runtime 0.1.0 is published. The [release record](gap_audit.md)
  identifies the checksummed archive and source tag.

<a id="AEQ-PY-BINDING-001"></a>
## AEQ-PY-BINDING-001 — Publish the quantity surface as a Python wheel

- status: blocked; priority: feature; needs: none
- basis: 64898d22720ad88437f464c82d6ef0a52e15a801
- scope: `crates/aequitas-python/`, `.github/workflows/python-release.yml`
- outcome: `aequitas-python` (import package `pyaequitas`) is on PyPI with every SI quantity and unit, usable from
  `kwavers-python` ([ADR 0016](docs/adr/0016-python-quantity-binding.md)); the wheel pipeline is delivered (#72, `d81c9fb`).
- acceptance: every exported dimension round-trips against `Quantity::in_unit`; semantic normalization
  matches `MultiplyDimension`/`DivideDimension`; codegen regenerate-and-diff is clean; `pytest` is green
  against the built wheel; the `published` job installs the released wheel back.
- blocker: the PyPI trusted publisher is not registered (project `aequitas-python`, owner `ryancinsight`,
  repo `aequitas`, workflow `python-release.yml`, environment `pypi`); `pypi.org/pypi/aequitas-python/json` was 404 on 2026-11-13.
- re-open trigger: the publisher is registered.
- next step: tag `aequitas-python-v0.1.0` under release authority.
- milestone: M-ecosystem (owner-gated; does not gate 1.0 per ADR 0020).

<a id="AEQ-LEAN-SPIKE-001"></a>
## AEQ-LEAN-SPIKE-001 — Decide a machine-checked proof tool for numeric laws

- status: blocked; priority: verification; needs: none
- basis: 64898d22720ad88437f464c82d6ef0a52e15a801
- scope: a throwaway pilot outside the repository; `docs/adr/` for the result
- outcome: a Proposed ADR names the tool that carries a machine-checked numeric law (Verus, SMT proofs of
  Rust, or Aeneas, Rust to Lean 4) with its coverage, CI cost and install footprint on dev hosts. The type
  system checks dimensional consistency (`src/dimension/`); value identities are only runtime-tested.
- acceptance: the pilot states and proves one law in the chosen tool and records what it covers: the
  typenum projections, the external `FloatElement::powf`, and the `Quantity` wrappers.
- blocker: either toolchain adds a proof dependency to dev hosts and CI; the owner approves before any
  install.
- re-open trigger: the owner names a tool, or declines proof tooling and the gap is recorded as accepted.
- next step: draft the Proposed ADR with the recommended tool once the owner answers.
- milestone: M-ecosystem (owner-gated; does not gate 1.0 per ADR 0020).

<a id="AEQ-CONFORMANCE-GUARD-001"></a>
## AEQ-CONFORMANCE-GUARD-001 — Fail a member PR that raises a conformance debt class

- status: todo; priority: verification; needs: none
- basis: a55971204a5c6e1fdd22de5636927272d6415c8e
- scope: `.github/workflows/ci.yml`, the `main` branch required status checks
- outcome: a pull request that raises a debt class above the Atlas baseline row for `aequitas` fails a required check.
  `.github/workflows/ci.yml:33-45` calls Atlas `lockfile-guard.yml` and `semver-gate.yml`, not `conformance-guard.yml`
  (inputs `atlas-ref`, `member`); `missing_deny_docs` rose 0 to 1 on `main` until `0d0efbe91de676f991451de50e200984aaa46f65`.
- acceptance: `ci.yml` calls `ryancinsight/atlas/.github/workflows/conformance-guard.yml@<full SHA>` with `atlas-ref`
  equal to it and `member: aequitas`; its context is in `gh api repos/ryancinsight/aequitas/branches/main/protection
  --jq .required_status_checks.contexts` (today `verify`, `Python bindings`, `supply-chain`,
  `Lockfile integrity / Lockfile integrity`, `Python bindings (free-threaded)`); a probe branch removing
  `#![deny(missing_docs)]` from one crate fails it.
- next step: add the job beside `lockfile` at the current Atlas `main` commit, then require its context.
- milestone: M-1.0

<a id="AEQ-RUNTIME-FORBID-UNSAFE-001"></a>
## AEQ-RUNTIME-FORBID-UNSAFE-001 — Forbid unsafe code at the runtime crate root

- status: todo; priority: tightening; needs: none
- basis: a55971204a5c6e1fdd22de5636927272d6415c8e
- scope: `crates/aequitas-runtime/src/lib.rs`
- outcome: the crate root carries `#![forbid(unsafe_code)]` as the root crate does (`src/lib.rs:12`). The crate has no
  `unsafe` (`git grep -n unsafe -- crates/aequitas-runtime` prints nothing) and inherits `unsafe_code = "forbid"` through
  `[lints] workspace = true`, so the attribute adds no new restriction; it keeps the guarantee visible in the source
  and independent of the manifest, per the stack's crate-level convention.
- acceptance: `git grep -n 'forbid(unsafe_code)' -- crates/aequitas-runtime/src/lib.rs` prints the attribute (empty today) and
  the workspace format, clippy and test gate passes.
- next step: add the attribute after `#![no_std]`.
- milestone: M-1.0
