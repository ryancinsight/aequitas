# ADR 0016: Expose Aequitas quantities to Python as a runtime dimension tag

## Status

Proposed — 2026-09-06. Mandated by
[AEQ-PY-BINDING-001](../../backlog.md#aeq-py-binding-001).

Revised 2026-09-06: the first revision asserted that `kwavers-python` was
absent from PyPI and left the distribution name unstated. A registry check
found `kwavers-python` 0.1.0 already published under this author, and the
`aequitas` name taken by an unrelated project. Both corrections are folded
into Consequences and change the naming decision, not the representation.

## Context

Aequitas encodes SI dimensions in types. `Quantity<T, D>` is
`#[repr(transparent)]` over `T`; `D` is `PhantomData<Dimension<L, M, T, I, Th,
N, J, S>>` over `typenum` exponents and a semantic marker. The crate-level
documentation states the consequence directly: no dynamic dispatch, no
allocation, and no runtime dimension metadata. Dimensional correctness is a
property of monomorphization, discharged by `rustc` and erased before the
binary exists.

Python has no compile step, so those types cannot be exported. A Python caller
holding a `Length` holds a `PyObject`, and any dimensional law it obeys must be
evaluated when the operation runs. The question this record answers is not
whether the Rust types can cross the boundary — they cannot — but what runtime
representation reproduces the same algebra, and how a consumer such as
`kwavers-python` accepts it without coupling the two wheels' Rust ABI.

The present surface is 81 quantity aliases over 81 dimension aliases, 87 unit
marker structs with 94 `LinearUnit` impls, and 11 semantic markers.
`LinearUnit` is sealed and linear (`base = value * SCALE`); affine units are
recorded as a future contract, so no `degC` or `degF` exists to expose.

Kwavers already depends on Aequitas in its binding crate: 12 of 187
`kwavers-python` source files reconstruct typed quantities from bare `f64`
arguments (`Length::from_unit::<Meter>(radius)`). The type safety therefore
begins one frame inside the FFI boundary. From Python, a radius is a naked
float whose meaning lives in a parameter name.

## Decision

Aequitas grows a second workspace member, `aequitas-python`: a `cdylib` PyO3
crate that owns every Python-facing type. The `aequitas` crate stays `no_std`,
`forbid(unsafe_code)`, and free of any `pyo3` dependency. This is the
architecture_scoping crate-to-workspace promotion trigger — a second deployment
artifact.

The exported representation is a value in canonical SI base units carried
alongside a runtime dimension tag: the seven integer exponents plus a semantic
discriminant. Both halves are derived from the Rust types through a const trait
implemented over the existing `Dimension` parameters, so `aequitas` remains the
single source of truth and the Python tag cannot drift from the Rust law.

Arithmetic on the Python side adds and subtracts exponent vectors, and
normalizes the semantic discriminant to `BaseSemantics` on multiplication and
division, matching `MultiplyDimension`/`DivideDimension`. A dimensional
violation raises a Python exception where Rust would have failed to compile.

The semantic markers are load-bearing and are not recoverable from the exponent
vector alone. `StressSemantics` and pressure share `M L^-1 T^-2`;
`AbsoluteTemperatureSemantics` and `TemperatureDifferenceSemantics` share
`Theta`; `SurfaceTensionSemantics` shares `M T^-2` with energy per area. A tag
carrying only exponents would collapse exactly the distinctions ADRs 0003,
0004, 0012 and 0014 exist to draw, so the discriminant is part of the tag, not
an annotation on it.

Static checking is recovered in the Python type checker rather than abandoned.
Codegen emits `.pyi` stubs in which each of the 81 aliases is a distinct class
and the closed set of dimensional pairings appears as `@overload` signatures,
so `mypy` rejects `length + time` before the program runs. The generated stubs
and the runtime table come from one pass over the Rust inventory.

### Cross-extension interop

Two independently built extension modules do not share Rust types. A `Length`
constructed in `pyaequitas._pyaequitas` is opaque to `pykwavers._pykwavers`
unless both link the same `aequitas-python` version, and a version skew there
fails at runtime with an error naming the same type twice.

The contract is therefore duck-typed, not ABI-shared: an object is a quantity
if it exposes its base-unit magnitude and its dimension tag through named
attributes. `pykwavers` validates the tag at the boundary and extracts the
magnitude, accepting any conforming object and staying decoupled from the
`aequitas-python` wheel version. A direct downcast may serve as a fast path where both
wheels agree, never as the requirement. The per-call cost is a tuple
comparison, which is not observable beside a simulation step.

Scalar parameters cross as quantities. Bulk arrays do not: per-element wrapping
would defeat the array substrate, so `PyReadonlyArray2<f64>` stays raw and its
dimension is carried by the surrounding call, checked once.

Backward compatibility is preserved by accepting both forms. A bare `float`
keeps its present meaning at every existing `kwavers-python` entry point; a
quantity is accepted where a float is, and is checked. No existing caller
breaks, and the migration is opt-in per call site.

## Alternatives rejected

- Export `Quantity<f64, D>` monomorphizations as 81 opaque `pyclass`es with a
  generated pairing table and no runtime tag. Rejected: products outside the
  named alias set have no class to land in, so the algebra is not closed, and
  the table is 81x81 entries to maintain against an inventory that grows per
  ADR.
- Carry only the seven exponents. Rejected: collapses stress into pressure and
  absolute temperature into temperature difference, discarding the semantic
  layer this crate spent five ADRs establishing.
- Depend on `pint` and map Aequitas units onto its registry. Rejected: the
  dimensional law would then have two owners with independent release
  cadences, and the semantic markers have no `pint` equivalent.
- Share the `aequitas-python` pyclass ABI between wheels as the required
  interop path. Rejected: it couples the release of every consumer wheel to
  the Aequitas wheel, and its failure mode is an error message that names the
  same type on both sides.
- Add the binding to the `aequitas` crate behind a feature. Rejected by the
  PyO3 boundary rule: a domain crate never depends on `pyo3`, and a
  feature-gated binding is a second build configuration of the law crate.

## Consequences

Affine units become a consumer-visible gap. A Python user of an ultrasound and
therapy package will expect `celsius`; the sealed `LinearUnit` contract cannot
express it. The deferred affine-unit item in the backlog is promoted to a
blocker for the thermal surface specifically, and is implemented upstream in
`aequitas` rather than approximated in the binding.

The consumer already ships. `kwavers-python` 0.1.0 is published on PyPI under
this author, so the interop described here lands against a live distribution
rather than a hypothetical one, and the bare-float compatibility clause above
is a released-API obligation, not a courtesy. Its Cargo `publish = false`
governs crates.io only.

Aequitas therefore needs its own publish pipeline: trusted publishing, an
`abi3` floor, a `manylinux` floor, and an install-and-import smoke test. The
binding surface is pure scalar, so `abi3` is clean for it.

The distribution name is constrained. `aequitas` on PyPI is taken by an
unrelated project at v1.1.0, and that project's top-level import is also
`aequitas`, so claiming that import name would collide inside any environment
holding both. The distribution is `aequitas-python` and the import package is
`pyaequitas`, mirroring the `kwavers-python`/`pykwavers` split already used in
this stack. The Rust crate name `aequitas` is unaffected.

The cross-repo half of this contract — the attribute protocol and its
validation — is a meta-repo concern and is recorded there, with contract tests
on both sides so a change to the tag shape fails in `kwavers` rather than in a
user's notebook.

## Verification

Codegen freshness is a regenerate-and-diff gate: the runtime table and the
`.pyi` stubs rebuild from the Rust inventory, and a drift fails CI rather than
awaiting the next touch. A generic conformance suite instantiates every
exported dimension and asserts round-trip equality against `Quantity::in_unit`
for each of its units, so a Python conversion cannot diverge from the Rust one.
Semantic normalization is asserted per marker across multiplication and
division. The Python suite runs under `pytest` against the built wheel, and the
`kwavers` side pins the attribute protocol in a contract test.
