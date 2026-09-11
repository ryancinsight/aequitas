# ADR 0017: Version and tag each distribution independently

## Status

Proposed — 2026-09-11. Mandated by
[AEQ-RELEASE-TAG-FORM-2026-09-11](../../backlog.md#aeq-release-tag-form-2026-09-11).

## Context

The workspace ships two distributions to two registries:

| distribution | registry | version | consumer |
|---|---|---|---|
| `aequitas` | crates.io | 0.2.0 | Rust crates across the Atlas stack |
| `aequitas-python` | PyPI | 0.1.0 | Python code, including `kwavers-python` |

`aequitas-python` sets `publish = false` for crates.io; its deliverable is the
wheel ([ADR 0016](0016-python-quantity-binding.md)). The two already carry
different versions, so the workspace does not version as one unit, and no
record said so.

Releases so far were tagged `v0.1.0` and `crate-aequitas-v0.2.0`, and
`rust-release.yml` publishes only for tags starting `crate-`. That prefix is a
type marker: every tag here names a package, and release tooling
(cargo-release, release-plz) reads neither it nor a bare `v<version>` when a
workspace holds more than one distribution.

## Decision

Each distribution versions independently and is released under a tag of the
form `<package>-v<version>`: `aequitas-v0.3.0`, `aequitas-python-v0.1.0`. This
is the form the ecosystem's release tooling already understands.

- A fix confined to the binding releases `aequitas-python` without bumping
  the law crate, and a law-crate release does not force a wheel.
- Each release workflow acts on its own prefix and nothing else:
  `rust-release.yml` on `aequitas-v`, `python-release.yml` on
  `aequitas-python-v`. `aequitas-python-v...` does not start with
  `aequitas-v`, so a wheel release never starts the crate workflow.
- Both workflows fail a release whose tag version differs from the manifest.
- The published tags `v0.1.0` and `crate-aequitas-v0.2.0` stay as history;
  the next law-crate release is `aequitas-v<version>`.

## Alternatives rejected

- **One workspace version, one `v<version>` tag.** Every release of either
  distribution would bump both, publishing unchanged artifacts to one registry
  whenever the other changes, and a wheel fix would imply a law-crate release.
- **Keep `crate-<package>-v<version>`.** It carries a type marker, and the
  Python distribution would need a second invented prefix (`wheel-`...) for the
  same concept.

## Consequences

- `rust-release.yml` resolves the package from the tag as before, from the new
  form; the rest of its validation is unchanged.
- The atlas SemVer gate is still called for `aequitas` only.
- Consumers pin by version, not by tag, so the change in tag form breaks no
  dependency.

## Verification

- The tag parse and the prefix gate are checked against `aequitas-v0.3.0`,
  `aequitas-python-v0.1.0` and `crate-aequitas-v0.2.0` in the change that
  introduces them: the first releases `aequitas`, the other two do not start
  the crate workflow.
- `python-release.yml` checks its tag against the manifest in its `version`
  job, exercised by every pull request that edits it.
