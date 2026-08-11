# Example: First Quantities

**Crate**: `aequitas`
**Source**: `examples/book_first_quantities.rs`

The fastest way to see Aequitas in action is to build a few quantities from
clinical measurements, convert between units, and derive a secondary value.
All conversion factors are compile-time constants; the only runtime cost is
a single multiply per `from_unit` or `in_unit` call, which the optimizer
typically folds away when the factor is 1.0.

## Source

```rust,ignore
{{#include ../../../examples/book_first_quantities.rs}}
```

## Output

```text
aperture: 38.0 mm = 0.0380 m
centre frequency: 3.50 MHz = 3.500e6 Hz
peak pressure: 250 kPa = 250000 Pa
wavelength at 3.5 MHz in tissue: 0.440 mm
period: 2.8571e-1 µs
```

## What to notice

- `Length::from_unit::<Millimeter>(38.0)` constructs a `Quantity<f64,
  dimensions::Length>` whose stored value is `0.038` (metres).  The type
  never changes; only the stored value is scaled at construction time.

- `in_unit::<Megahertz>()` is the inverse: divide the stored-in-Hz value by
  the MHz scale factor to recover the display value.  Neither call touches
  the heap.

- The wavelength calculation extracts `in_unit::<Hertz>()` as a raw `f64`,
  performs the scalar division, and wraps the result back with
  `Length::from_unit::<Meter>`.  This is the boundary-extraction pattern:
  formula kernels work on plain scalars; physical typing guards the public
  contract.

- The period's `in_unit::<Microsecond>()` call returns `2.857e-1 µs`
  (≈ 0.286 µs), confirming that `1 / 3.5e6 Hz = 285.7 ns`.
