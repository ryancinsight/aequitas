# 8. Position in the Stack

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - Atlas layer: eunomia (scalars) → aequitas (quantities) → domain crates
  - All public physical-value fields in kwavers, CFDrs, helios, ritk are
    Aequitas quantities; raw scalars appear only at formula boundaries and
    in storage (Leto arrays hold the base-unit value without the type wrapper)
  - The UnitScalar seam: eunomia::UnitScalar is the scalar bound that both
    Quantity<T, D> and LinearUnit<D> use, so any eunomia scalar can carry
    a physical quantity
  - No runtime dimension metadata: dimension information is erased at
    codegen; the only overhead vs a raw scalar is the from_unit/in_unit
    multiplications, which fold away for compile-time constants
  - What aequitas does NOT own: material property tables (proteus), physical
    constants (each integrator), coordinate systems (gaia)
-->
