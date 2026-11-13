# Aequitas backlog

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
