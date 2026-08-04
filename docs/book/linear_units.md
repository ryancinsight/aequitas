# 4. Linear Units and Conversion

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - LinearUnit<D> trait: SYMBOL and SCALE constants; to_base / from_base helpers
  - Sealed trait: only Aequitas-defined unit structs implement LinearUnit so
    callers cannot introduce mis-scaled conversions
  - Quantity::from_unit::<U>(value) and Quantity::in_unit::<U>()
  - Compile-time constant folding: SCALE is a compile-time f64; LLVM folds
    value × SCALE into a single multiply instruction
  - Symbol constants for display without runtime allocation
-->
