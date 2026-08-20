# 1. What Is a Physical Quantity?

A physical quantity is a scalar paired with a dimension known to the Rust
compiler. Aequitas represents that pair as `Quantity<T, D>`: a transparent
wrapper around `T` plus a zero-sized `PhantomData<D>` marker.

```rust
extern crate aequitas;

use aequitas::quantity::Quantity;
use aequitas::systems::si::{dimensions, quantities::Length};

let distance: Length<f64> = Quantity::from_base(1.5);
assert_eq!(distance.as_base(), &1.5);

// The named alias is easier to read while retaining the same generic model.
let _: Quantity<f64, dimensions::Length> = distance;
let distance = Length::from_base(1.5);
assert_eq!(distance.into_base(), 1.5);
```

`from_base` accepts a value already expressed in the canonical SI base unit.
`as_base` borrows that value and `into_base` moves it out. Unit-aware
construction and extraction are covered in the next chapter; these methods are
useful at formula and storage boundaries where the base representation is
already explicit.

A raw `f64` cannot tell a reviewer whether `1.5` means metres, seconds, or
kilograms, and the compiler cannot prevent adding incompatible values. A named
alias such as `Length<f64>` supplies documentation, while its dimension marker
makes invalid arithmetic a type error.

The dimension marker occupies no storage. `Quantity<T, D>` is `#[repr(transparent)]`
and has the size and alignment of `T`. Its structural `Copy`, `Clone`,
`Default`, equality, ordering, and debug implementations constrain `T` only;
the zero-sized dimension type does not need to implement those traits.
