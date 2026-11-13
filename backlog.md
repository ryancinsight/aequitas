# Aequitas backlog

<a id="AEQ-PY-BINDING-001"></a>
## AEQ-PY-BINDING-001 — Publish the quantity surface as a Python wheel

- status: blocked; priority: feature; needs: none
- scope: `crates/aequitas-python/`, `.github/workflows/python-release.yml`
- outcome: `aequitas-python` (import package `pyaequitas`) is on PyPI with every SI quantity and unit,
  conversion and dimensional arithmetic, consumable from `kwavers-python`
  ([ADR 0016](docs/adr/0016-python-quantity-binding.md)). The member, typed classes, affine units, the
  free-threaded build and the ten-wheel-plus-sdist pipeline ([#72](https://github.com/ryancinsight/aequitas/pull/72), `d81c9fb`) are delivered.
- acceptance: every exported dimension round-trips against `Quantity::in_unit`; semantic normalization
  matches `MultiplyDimension`/`DivideDimension`; codegen regenerate-and-diff is clean; `pytest` is green
  against the built wheel; the `published` job installs the released wheel back.
- blocker: the PyPI trusted publisher is not registered (project `aequitas-python`, owner `ryancinsight`,
  repo `aequitas`, workflow `python-release.yml`, environment `pypi`), a pypi.org account action for the
  owner. `https://pypi.org/pypi/aequitas-python/json` returned 404 on 2026-11-13.
- re-open trigger: the publisher is registered.
- next step: tag `aequitas-python-v0.1.0` under release authority.

<a id="AEQ-LEAN-SPIKE-001"></a>
## AEQ-LEAN-SPIKE-001 — Decide a machine-checked proof tool for numeric laws

- status: blocked; priority: verification; needs: none
- scope: a throwaway pilot outside the repository; `docs/adr/` for the result
- outcome: a Proposed ADR names the tool that carries a machine-checked numeric law (for example the
  thermal-diffusivity identity in the README), Verus (SMT proofs of Rust) or Aeneas (Rust to Lean 4), with
  its coverage, CI cost and install footprint on dev hosts. The type system checks dimensional consistency
  (`src/dimension/`); value identities are only runtime-tested and the group laws are assumed from typenum.
- acceptance: the pilot states and proves one law in the chosen tool and records what it covers: the
  typenum projections, the external `FloatElement::powf`, and the `Quantity` wrappers.
- blocker: either toolchain adds a proof dependency to dev hosts and CI; the owner approves before any
  install.
- re-open trigger: the owner names a tool, or declines proof tooling and the gap is recorded as accepted.
- next step: draft the Proposed ADR with the recommended tool once the owner answers.
