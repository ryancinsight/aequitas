# ADR 0001: Own Atlas physical-quantity law in Aequitas

- Status: Accepted
- Date: 2026-07-19
- Class: [arch] [minor]

## Context

Atlas has a repeated physical-unit boundary:

- Kwavers directly depends on `uom` for bubble-energy quantities and also owns
  raw conversion helpers.
- Helios owns transparent unit-suffixed domain newtypes and raw millimetre,
  centimetre, joule, electronvolt, and attenuation conversions.
- CFDrs exposes material properties as raw scalars whose units exist only in
  documentation.

The evidence is pinned to the audited consumer revisions:

- [Kwavers `uom` dependency](https://github.com/ryancinsight/kwavers/blob/3fb92218c0c519bf7b7ec5670786bee4dcb9414c/crates/kwavers-physics/Cargo.toml#L30)
- [Helios validating unit newtypes](https://github.com/ryancinsight/helios/blob/79b09e98a1bb7fda4e80abd048e2b5ea768889aa/crates/helios-core/src/units.rs)
- [CFDrs material-property units](https://github.com/ryancinsight/CFDrs/blob/8e00b40a3c8052cf4638d4c1e2b8c862771afc00/crates/cfd-core/src/physics/material/traits.rs)

`uom` 0.38.0 is a mature `no_std` implementation with compile-time
dimensional analysis, custom systems, extensive SI and non-SI units, affine
conversion support, formatting, serialization, and many storage types. Its
quantity arithmetic is already designed as a zero-cost abstraction. These are
strengths, not gaps:

- [`uom` 0.38.0 design and features](https://docs.rs/uom/0.38.0/uom/)
- [`Quantity` implementation](https://github.com/iliekturtles/uom/blob/v0.38.0/src/system.rs#L252)
- [storage-type generation](https://github.com/iliekturtles/uom/blob/v0.38.0/src/storage_types.rs)

The Atlas-specific gap is storage ownership. `uom::storage_types!` dispatches
through a closed list of supported types and generates per-storage modules.
Its SI units and conversion implementations therefore cannot be extended
downstream to Eunomia wrapper types without modifying or forking `uom`.
Atlas requires a single first-party implementation over the sealed
`eunomia::FloatElement` contract so every supported precision inherits the
same quantity laws and tests.

## Decision

Create Aequitas as an independent public foundation repository.

The initial package is one `no_std` crate:

- `Quantity<T, D>` is `#[repr(transparent)]` over `T`.
- `D` is a zero-sized seven-axis SI exponent vector composed through Typenum.
- arithmetic is generic and monomorphized once per admitted `T` and dimension;
  no dynamic dispatch or runtime dimension vector exists;
- linear units are sealed zero-sized markers with one default conversion
  implementation;
- unit coefficient metadata is `f64`, but each coefficient is converted once
  through `T::from_f64` and all quantity arithmetic executes in `T`;
- Eunomia owns scalar representations and numeric laws;
- `uom` is a development-only differential oracle and is absent from the
  runtime dependency graph.
- First-party manifests use Eunomia's canonical repository source without a
  revision qualifier. Consumer lock files pin the exact commit so Aequitas and
  the consumer cannot instantiate distinct Eunomia source identities.

The first consumer slice replaces the direct `uom` dependency in Kwavers
bubble-energy code. This supplies real pressure, power, energy, heat capacity,
temperature, time, length, area, mass, velocity, and thermal-conductivity
operations rather than an empty package shell.

The Hyperion photon/optical provider supplies the next concrete extension
trigger. Reciprocal length, area per mass, and energy per area remain Aequitas
dimensions because they compose exactly from length, area, mass, and energy.
Hyperion wraps these quantities with attenuation, scattering, optical-depth,
and fluence validity contracts instead of re-owning SI exponent or conversion
laws.

## Rejected alternatives

### Adopt `uom` as the Atlas provider

Rejected because its closed storage-generation surface cannot admit Eunomia
wrappers downstream. Keeping `uom` would preserve two scalar law systems and
storage-specific API modules.

### Fork or patch `uom`

Rejected because Atlas would inherit a broad external compatibility surface
and a permanent merge burden. Aequitas needs a narrower floating-point
simulation contract, not a fork of all `uom` features.

### Keep consumer-local newtypes and conversion functions

Rejected because the same dimension and conversion laws already recur across
three integrators. Consumer ownership cannot provide one conformance suite or
compile-time cross-domain interoperability.

### Reimplement type-level integers

Rejected because Typenum already supplies stable, zero-sized type arithmetic.
Aequitas owns physical dimension semantics, not generic type-level arithmetic.

## Consequences

- Aequitas starts narrower than `uom`; it does not claim feature parity.
- Affine temperature units, quantity kinds, formatting, serialization,
  integer/rational storage, and comprehensive non-SI coverage remain excluded
  until a concrete Atlas consumer requires them.
- Domain validity remains outside Aequitas. A quantity may contain NaN or
  infinity when `T` supports them; domain newtypes validate finite/range
  constraints at their ownership boundary.
- Adding a unit or quantity extends the provider and its conformance suite
  rather than creating a consumer-local unit implementation.

## Verification

- generic law tests instantiate every shipped Eunomia float implementation:
  `f32`, `f64`, `F16`, `F32`, `F64`, `Bf16`, `Bf8`, `Bf4`, `F8`, and `F4`;
- differential tests compare the shared SI surface with `uom` 0.38.0;
- property tests cover conversion round trips over bounded finite inputs;
- compile-fail doctests reject dimensionally invalid addition;
- layout tests prove all unit/dimension markers are zero-sized and quantities
  are transparent over their scalar;
- a release codegen fixture compares typed velocity computation with raw scalar
  division;
- the migrated Kwavers package runs its complete focused native and doctest
  gates after removing `uom`.
