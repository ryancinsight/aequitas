# ADR 0007: Name the SI Number-Density Quantity

## Status

Accepted

## Context

Contrast-agent and population models report entity concentration as entities
per volume. A raw scalar is interchangeable at the call site with mass
density, volumetric flow, or any other inverse-volume value, even though those
quantities have different domains and downstream meanings.

## Decision

Aequitas defines `NumberDensity` with the coherent `m⁻³` unit
`PerCubicMeter`. Consumers use the typed quantity at public population and
concentration contracts and extract the base scalar only at dense-array,
formula, or serialization boundaries.

The quantity is dimensionally the reciprocal of volume and intentionally has
no runtime overhead beyond its scalar representation. Domain validation remains
in the consumer, consistent with Aequitas' quantity boundary.

## Alternatives

- Keep concentration as `f64`: rejected because the public contract permits
  accidental substitution with mass density and loses the entity-count
  vocabulary needed by population models.
- Add a consumer-local wrapper: rejected because the SI dimension and coherent
  unit are provider-owned shared vocabulary.

## Verification

- The dimensional-law test constructs `NumberDensity` through `PerCubicMeter`.
- The layout test verifies the marker remains zero-sized.
- Aequitas' standard nextest, clippy, doctest, and rustdoc gates cover the
  public surface.
