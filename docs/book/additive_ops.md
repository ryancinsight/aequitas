# 6. Additive Operations

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - Add and Sub are only defined for same-dimension quantities (enforced by
    the trait bound on the dimension parameter)
  - AddAssign / SubAssign follow the same rule
  - Neg for quantities with a signed scalar
  - Why you cannot add a Length to a Time: the compiler rejects it because
    Length and Time are distinct types, even though both are Quantity<f64, _>
  - The pattern for "sum N quantities of the same type": use Iterator::sum
    after implementing Sum (not yet in the crate — note as future work)
-->
