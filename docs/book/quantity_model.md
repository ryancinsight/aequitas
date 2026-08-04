# 1. What Is a Physical Quantity?

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - The Quantity<T, D> struct: transparent wrapper, PhantomData dimension
  - from_base / as_base / into_base as the base-unit API
  - Why a raw f64 is insufficient: no compiler enforcement, no unit documentation
  - The "named quantity" pattern: type aliases for readability (Length<f64>)
  - Copy/Clone/PartialEq/PartialOrd derive without constraining D
-->
