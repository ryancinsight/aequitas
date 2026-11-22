# Aequitas backlog

<a id="aequitas-stable-qualification"></a>
## AEQUITAS-STABLE-QUALIFICATION — aequitas stable-release qualification [patch] — todo
- priority: verification; milestone: M-1.0; basis: `17c7c91e727e`.
- Outcome: Freeze the quantity and unit boundary on the published Eunomia line. Follow the [release scope](docs/adr/0020-release-scope-1-0.md) and [milestone exits](backlog/milestones.md).
- Representative manifest `aequitas 0.5.0`. Target 1.0.0.
- [ ] [patch] Run dimension laws, affine-temperature, scalar arithmetic and conversion properties on the candidate. Compare composed dimensions and converted values to the independent unit oracle, including values adjacent to a rounding boundary.
- [ ] [patch] Build fresh consumers with default features on and off and verify a single Eunomia type identity. Reconcile the fractional-dimension and Python spelling proposals ([#172](https://github.com/ryancinsight/aequitas/pull/172) and [#171](https://github.com/ryancinsight/aequitas/pull/171)) with the freeze before choosing the candidate.
- [ ] [major] Apply ADR 0020 to the stable Rust surface. Keep the Python wheel and proof-tool decision in M-ecosystem with their existing owners.
- [ ] [patch] Attach exact-revision evidence for the local exits and this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

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
