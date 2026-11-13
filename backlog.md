# Aequitas backlog

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
  builds 10 wheels (abi3 and 3.14t on five native runners) plus an sdist, each smoke-tested;
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


## AEQ-LEAN-SPIKE-001 — Decide a machine-checked proof tool for numeric laws [arch] — blocked <a id="aeq-lean-spike-001"></a>

- **Question:** can a numeric law of aequitas, such as the thermal-diffusivity
  identity in the README, be stated and proved by a machine checker? Which tool
  should carry it: Verus, which proves Rust programs with SMT, or Aeneas, which
  translates safe Rust into Lean 4 for proof there?
- **Why:** the type system checks dimensional consistency (`src/dimension/`).
  Value identities are only runtime-tested. The dimension group laws are assumed
  from typenum and nothing in the stack checks them.
- **Method:** a throwaway pilot outside the repository states and proves one law
  in the chosen tool. The pilot records what the tool covers: the typenum
  projections, the external `FloatElement::powf`, and the `Quantity` wrappers.
- **Deliverable:** a Proposed ADR that records the tool choice, its coverage,
  its CI cost, and its install footprint on dev hosts, with the pilot result.
- **Blocker:** either toolchain adds a proof dependency to dev hosts and CI.
  That needs the owner's approval before anything is installed.
- **Re-open trigger:** the owner names a tool, or declines proof tooling and the
  gap is recorded as accepted.


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
