# 2. Canonical SI Storage

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - All values stored in the coherent SI base unit for their dimension
  - Conversion happens at construction (from_unit) and extraction (in_unit)
  - Formulas always operate in base units — no scale factor inside a kernel
  - The UnitScalar::scale_by_f64 seam: constant-foldable at compile time
  - Why this is safe: the only way to create a quantity is through from_base
    or from_unit; both leave the stored value in base units
-->
