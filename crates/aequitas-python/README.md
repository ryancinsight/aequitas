# pyaequitas

SI physical quantities and dimensional arithmetic for Python, backed by the
Rust [Aequitas](https://github.com/ryancinsight/aequitas) crate.

```bash
pip install aequitas-python
```

```python
import pyaequitas as aq

radius = aq.length(2.5, "mm")
radius.base            # 0.0025 -- always canonical SI base units
radius.in_unit("cm")   # 0.25

area = radius * radius
area.quantity          # 'area'

speed = aq.length(10.0, "m") / aq.time(2.0, "s")
speed.quantity         # 'velocity'
speed.in_unit("m/s")   # 5.0
```

Operations that do not make physical sense raise instead of returning a
plausible number:

```python
aq.length(1.0, "m") + aq.time(1.0, "s")
# ValueError: cannot add `m` and `s`: dimensions differ
```

## What crosses the boundary

Aequitas encodes SI dimensions in Rust types. `Quantity<T, D>` is
`#[repr(transparent)]` over `T` with `D` in `PhantomData`, so dimensional
correctness is discharged by the compiler and erased before the binary exists.
There is nothing left at runtime for Python to hold.

This package therefore carries the dimension as data: a magnitude in canonical
SI base units, the seven SI base exponents, and a semantic discriminant.
Checks that `rustc` performs at compile time happen here when the operation
runs. Every unit symbol and scale factor is read from the Rust `LinearUnit`
associated constants, so a conversion here and a conversion in Rust cannot
disagree.

## Why the semantic discriminant

The exponent vector alone would merge quantities Aequitas deliberately keeps
apart. Stress and pressure are both `m^-1 kg s^-2`; an absolute temperature
and a temperature difference are both `K`; surface tension and energy per area
are both `kg s^-2`. Each pair differs only in its marker, so the marker is part
of the dimension tag:

```python
aq.stress(1.0, "Pa") + aq.pressure(1.0, "Pa")
# ValueError: cannot add `m^-1 kg s^-2 [stress]` and `m^-1 kg s^-2`
```

The same reason makes unit lookup quantity-scoped: `Pa` belongs to both
pressure and stress, `K` to both temperature quantities, and `J` to both energy
and flexural rigidity, so a bare symbol does not identify a unit.

## Interoperating with other extensions

Two independently built extension modules do not share Rust types, so this
package publishes a structural contract rather than a class to downcast to. An
object is a quantity when it exposes:

- `__aequitas_base__` -- the magnitude in canonical SI base units, as a float;
- `__aequitas_dimension__` -- `((int x 7), str)`: the SI exponents in length,
  mass, time, current, temperature, amount, luminosity order, then the semantic
  discriminant.

Anything satisfying that participates in arithmetic here, whatever built it:

```python
class Reading:
    __aequitas_base__ = 1500.0
    __aequitas_dimension__ = ((1, 0, -1, 0, 0, 0, 0), "base")

aq.length(3.0, "mm") / aq.time(2.0, "us")   # a velocity
aq.base_value_of(Reading(), "velocity")     # 1500.0, dimension validated
```

A consumer wanting the magnitude without depending on this package at all can
read the two attributes directly. `PROTOCOL_VERSION` changes only when the tag
shape does.

## Typing

The package ships `py.typed` and generated stubs, so a type checker sees the
`Quantity` surface. Dimensional compatibility itself is checked at runtime:
the exponents live in the value, so a checker sees `Quantity` on both sides of
every operator.

## Units

`aq.quantity_names()` lists every quantity; `aq.unit_symbols(name)` lists the
units one accepts. Both come from the Rust inventory, not a copy of it.

Temperature is kelvin only. `LinearUnit` in the law crate is linear
(`base = value * scale`), so degrees Celsius and Fahrenheit -- which need an
offset -- are not yet expressible. That is an upstream gap, tracked as
`AEQ-PY-AFFINE-001`, not something this layer fakes.

## Licence

MIT OR Apache-2.0.
