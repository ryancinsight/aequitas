# ADR 0019: Adopt the Eunomia 0.10 datatype contract

- Status: Accepted
- Date: 2026-10-15
- Item: AEQ-EUNOMIA-010-COHORT [major]

## Context and decision

Eunomia's current default implementation removes CastFrom/CastTo, but its
manifest still declares 0.9.1. The published compatible 0.9.2 maintenance source
retains those traits. The reviewed provider correction versions the current
contract as 0.10.0. Aequitas exposes Eunomia scalar bounds in its public quantity
APIs, so adopting the new nominal provider identities requires a pre-1.0 minor
boundary even though Aequitas's Rust algorithms do not change.

Require Eunomia 0.10.0 and advance the Rust law crate and its internal binding
edge to 0.3.0. Use the canonical Git URL with no manifest revision alias; Cargo
locks the reviewed candidate source. The final cohort must select the merged,
published provider source before release acceptance. No older-source fallback,
conversion shim or numerical implementation change is introduced.

ADR 0017 gives Python independent distribution versioning. The Python API has
no behavior change in this increment, so its 0.1.0 version stays unchanged.

## Validation and release constraints

Retain all existing native/Python tests, fixture domains and timeouts. Validate
no-std, all-target strict Clippy, docs/doctests, package/API gates and required
exact-head CI. Local candidate validation is distinct from final cohort
acceptance while the prerequisite and consumer defaults are not integrated.

The existing book workflow triggers on Cargo.toml/Cargo.lock merges and deploys
Pages on a default-branch push. Deployment remains outside the authorized scope;
keep the default merge held rather than changing that workflow or its gates.
Miri remains deferred and unverified, and no performance experiments occur.
