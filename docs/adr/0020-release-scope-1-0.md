# ADR 0020: Aequitas release scope for 1.0

- Status: Accepted
- Date: 2026-11-10
- Revised: 2026-11-19
- Class: `[arch]` `[minor]`

## Context

Aequitas provides the Atlas physical-quantity and dimensional law over the
Eunomia scalar surface. Aequitas 0.3.2 permits Eunomia 0.10 through 0.12 and
locks an upstream Git commit for repository builds. Separately built crates
need the same registry package identity to exchange Eunomia scalar values.

Eunomia's `main` declares 0.12.0, its latest crates.io version. The earlier
Aequitas 0.4.0 milestone assumed a future Eunomia 1.0 release. The 0.12.0
archive provides the scalar operations the current Aequitas catalog uses.

## Decision

- **M-published-eunomia** (Aequitas 0.4.0) uses the published Eunomia 0.12 line
  from crates.io, with default features disabled in the dependency declaration.
  The lock selects Eunomia and Eunomia Derive 0.12.0. Aequitas's `std` feature
  opts into Eunomia's `std` feature.
- **M-1.0** (Aequitas 1.0.0) commits to stable quantity contracts after its
  documented gates pass and the owner authorizes the release. Evaluate later
  provider migrations against the releases available at that time.
- **M-ecosystem** tracks the Python wheel, proof tooling and third-party
  recurseml work under their existing ownership and acceptance criteria.

## Alternatives

A future Eunomia 1.0 requirement would make current consumers depend on an
unpublished package. Continuing to resolve a Git checkout in the workspace
would preserve the source difference between repository and registry builds.

## Verification

The lockfile guard requires one checksummed crates.io identity for Eunomia
and its derive crate, followed by locked Cargo resolution. The 0.4.0 gate
includes the full workspace suite, a release SemVer check, a packaging dry
run and a fresh consumer of the published archive with default features
enabled and disabled.

## Consequences

Open items carry milestone assignments. Close a milestone after its exit
checks pass on the fetched default branch. The deferred boundaries keep
their existing scope, and the Aequitas 1.0 release remains owner-authorized.
