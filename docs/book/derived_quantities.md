# 7. Derived Quantities

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - Mul<Quantity<T, RhsDimension>> for Quantity<T, LhsDimension> using
    MultiplyDimension associated type
  - Div similarly via DivideDimension
  - Concrete example: MassDensity × Velocity → AcousticImpedance (Rayl)
    kg/m³ × m/s = kg/(m²·s) = Pa·s/m
  - Why the dimension algebra is closed: every product/quotient of SI
    dimensions is itself an SI dimension
  - Practical rule: compute in base units, convert only at the boundary
    (from_unit at input, in_unit at output)
  - Scalar multiplication: Mul<T> scales the value, keeping the dimension
-->
