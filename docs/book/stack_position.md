# 8. Position in the Stack

Aequitas sits between scalar representation and domain algorithms:

```text
eunomia  →  aequitas  →  domain providers and integrators
scalars     quantities    Kwavers, CFDrs, Helios, RITK, …
```

Eunomia owns what a scalar is: native real, complex, and reduced-precision
representations. Aequitas owns what a scalar means physically: dimensions,
semantic quantity names, and linear unit conversion. Domain crates then use
those types in public contracts while retaining responsibility for their
formulas and validation.

`Quantity<T, D>` and `LinearUnit<D>` both rely on Eunomia's `UnitScalar` seam.
That lets a supported Eunomia scalar carry a physical quantity without
Aequitas defining another numeric vocabulary. In the audited Atlas integrations,
public physical contracts use Aequitas quantities while raw scalars remain at
formula, storage, and serialization boundaries. Complex values remain valid for
real-plus-quadrature observables with one physical unit; Aequitas does not
invent an imaginary SI dimension.

Dimension and unit markers are zero-sized type information. They guide
compilation and disappear from the runtime representation, so a quantity has
the size and alignment of its scalar. Conversion multiplies at explicit
boundaries; for constant units, normal optimization can fold the operation.

Aequitas deliberately does not own material-property tables (Proteus), scalar
allocation or arrays (Mnemosyne/Leto), scheduling and execution (Moirai),
coordinate systems and geometry (Gaia/Leto), or domain physical constants.
Those ownership boundaries keep the quantity layer reusable and prevent a
second provider from silently redefining a physical law.
