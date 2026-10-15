# ADR 0019: Adopt the Eunomia 0.10 datatype contract

- Status: Accepted
- Date: 2026-10-15
- Item: AEQ-EUNOMIA-010-COHORT [major]

## Context and decision

At the start of this increment, Eunomia's default implementation removed
CastFrom/CastTo while its manifest still declared 0.9.1. The compatible 0.9.2 maintenance source
retains those traits. The reviewed provider correction versions the current
contract as 0.10.0. Aequitas exposes Eunomia scalar bounds in its public quantity
APIs, so adopting the new nominal provider identities requires a pre-1.0 minor
boundary even though Aequitas's Rust algorithms do not change.

Require Eunomia 0.10.0 and advance the Rust law crate and its internal binding
edge to 0.3.0. Use the canonical Git URL with no manifest revision alias; Cargo
locks the reviewed merged provider source. Eunomia and derive 0.10.0 are
registry-verified from [the published merge](https://github.com/ryancinsight/eunomia/commit/a231e54dd3311bc916e78c91d994b00d651c1646).
All own package/API and hosted gates pass before the verified publication below. No older-source fallback,
conversion shim or numerical implementation change is introduced.

ADR 0017 gives Python independent distribution versioning. The Python API has
no behavior change in this increment, so its 0.1.0 version stays unchanged.

## Validation and release constraints

Retain all existing native/Python tests, fixture domains and timeouts. Validate
no-std, all-target strict Clippy, docs/doctests, package/API gates and required
exact-head CI. Local candidate validation is distinct from final cohort
acceptance while the prerequisite and consumer defaults are not integrated.

The user authorizes normal publication and automatic documentation deployment
after required exact-head checks and review. [PR #136](https://github.com/ryancinsight/aequitas/pull/136)
merges normally from reviewed source with all Rust and deadline files unchanged.

## Verified publication

Registry Aequitas 0.3.0 is not yanked and its downloaded archive checksum is
`18b05003076db4767bd4c721421fea137a8b06b305d6ec45b45b6487f931fd24`.
Archive VCS names [merge f9c84f8c](https://github.com/ryancinsight/aequitas/commit/f9c84f8c3965fa58a71d95a2f9de975abae3a0b5),
whose tree equals reviewed source. [Publisher 38744751314](https://github.com/ryancinsight/aequitas/actions/runs/38744751314)
and [Pages 38744751088](https://github.com/ryancinsight/aequitas/actions/runs/38744751088)
both succeed. All 164 unchanged archived Rust cases pass on real registry
Eunomia 0.10; the source workspace additionally retains 92 binding cases and
standard/free-threaded Python CI. The existing 60-second native deadline stays.
Required Mnemosyne/Hermes Miri is authorized separately; their final cohort
acceptance and held owner work are not claimed by this independent release.
