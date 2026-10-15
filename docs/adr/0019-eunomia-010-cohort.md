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
Final consumer acceptance still requires all own package/API and hosted gates. No older-source fallback,
conversion shim or numerical implementation change is introduced.

ADR 0017 gives Python independent distribution versioning. The Python API has
no behavior change in this increment, so its 0.1.0 version stays unchanged.

## Validation and release constraints

Retain all existing native/Python tests, fixture domains and timeouts. Validate
no-std, all-target strict Clippy, docs/doctests, package/API gates and required
exact-head CI. Local candidate validation is distinct from final cohort
acceptance while the prerequisite and consumer defaults are not integrated.

The existing book workflow triggers on Cargo.toml/Cargo.lock merges and deploys
Pages on a default-branch push; the user now authorizes this documentation
deployment. The same push selects the immutable pending-package publisher, and
registry Aequitas 0.3.0 is absent. The user now authorizes package publication
after all exact-head checks/reviews pass; merge normally and verify its source,
publication and documentation receipts without workflow or gate changes.
Required provider Miri CI is now authorized with actual
results pending; no held safety work or performance experiments are resumed.
