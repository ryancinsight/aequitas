# 3. The Dimension Type System

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - Dimension<L, M, T, I, Θ, N, J, Semantics> — seven SI base exponents as
    typenum integers, plus an optional zero-sized semantics marker
  - Why typenum: const-generic arithmetic without nightly features; the
    exponents are checked at the type level, not at runtime
  - DimensionProduct / DimensionQuotient: type-level addition/subtraction of
    exponent vectors — what makes Mul<Quantity<V>> → Quantity<Velocity×MassDensity>
    resolve to Quantity<AcousticImpedance>
  - Semantics markers: why Angle, AbsoluteTemperatureSemantics,
    SpringStiffnessSemantics etc. exist — two distinct physical meanings that
    share the same exponent vector (e.g. SpringStiffness and SurfaceTension are
    both m⁰·kg·s⁻²) need separate types to prevent silent substitution
  - How to read a dimension alias: Length = Dimension<P1,Z0,Z0,Z0,Z0,Z0,Z0>
    means L¹M⁰T⁰I⁰Θ⁰N⁰J⁰ — one metre per SI base combination
-->
