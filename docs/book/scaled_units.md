# 5. Scaled Units

Scaled units are ordinary Aequitas linear units whose `SCALE` expresses a
prefix or an exact relationship to an SI base unit. The scale always points
from the named unit to the canonical stored value.

```rust
extern crate aequitas;

use aequitas::systems::si::{quantities::{Length, Frequency}, units::{Millimeter, Megahertz}};

assert_eq!(Length::from_unit::<Millimeter>(2.5).into_base(), 2.5e-3);
assert_eq!(Frequency::from_unit::<Megahertz>(3.0).into_base(), 3.0e6);
```

## Prefix and conversion laws

`Prefixed<P, U>` applies any of the 24 decimal prefixes to an eligible
unit token. The ten binary prefixes apply to bit. Mass prefixes attach to
gram. A prefix attaches to one token, so a cube of a prefixed metre cubes
the scale too.

A linear unit with scale `sᵤ` stores `x × sᵤ`. The reading in unit `V` is
`(x × sᵤ) / sᵥ`. This follows by equating the two readings in base units.
Products add dimension exponents and multiply scales. Quotients subtract
dimension exponents and divide scales. An integer power multiplies each
dimension exponent by that integer and raises the scale to the same power.
The laws are exact over the reals. Aequitas stores the scale as binary64
metadata and applies it using the quantity's scalar precision.

```rust
extern crate aequitas;
extern crate typenum;

use aequitas::systems::si::{
    quantities::{Mass, ThermodynamicTemperature, Volume},
    units::{Celsius, Gram, Kelvin, Kilogram, Meter},
};
use aequitas::unit::{Power, Prefixed, prefixes::{Deci, Kilo, Milli}};

let mass = Mass::from_unit::<Prefixed<Kilo, Gram>>(2.5);
assert_eq!(mass.in_unit::<Kilogram>(), 2.5);

type CubicDecimeter = Power<Prefixed<Deci, Meter>, typenum::P3>;
let volume = Volume::from_unit::<CubicDecimeter>(300.0_f64);
assert!((volume.into_base() - 0.3).abs() <= f64::EPSILON);

let temperature = ThermodynamicTemperature::from_affine_unit::<Prefixed<Milli, Celsius>>(1000.0);
assert_eq!(temperature.in_unit::<Kelvin>(), 274.15);
```

An absolute prefixed Celsius reading converts as `x × s + 273.15` kelvin,
with inverse `(T − 273.15) / s`. Subtracting two absolute temperatures cancels
the origin. A Celsius interval therefore converts as `x × s` kelvin.

The BIPM uses the same dimensions for several different quantities. For
periodic phenomena, `AngularFrequency::from_frequency` multiplies hertz by
`2π` because each cycle advances the phase by `2π` radians. For radiation
dose, `DoseEquivalent::from_absorbed_dose` takes the quality factor explicitly
and evaluates `H = QD`. The `constants` module provides all seven SI defining
constants.

## Table 8 units

`systems::si::units` includes minute, hour, day, degree, arcminute, arcsecond,
are, hectare, barn, litre, tonne, angstrom, gal, bar, dalton, astronomical
unit, nautical mile, knot, electronvolt and var. The dalton conversion uses
the CODATA 2022 central value. `DALTON_STANDARD_UNCERTAINTY` records the
uncertainty in kilograms. A reading of `x` daltons contributes
`|x| × DALTON_STANDARD_UNCERTAINTY` to the standard uncertainty in kilograms
from this coefficient alone. The other Table 8 linear definitions are exact
over the reals.

Nepers, bels and decibels use `systems::si::levels`. For amplitude
ratio `A/A₀`, one neper corresponds to a natural-log coordinate of one.
For power ratio `P/P₀`, one bel corresponds to `ln(10)` and one decibel
to `ln(10)/10`. When both powers are proportional to squared amplitudes
with the same coefficient, taking natural logarithms gives
`ln(P/P₀) = 2 ln(A/A₀)`. The square-law conversion encodes that assumption.

```rust
extern crate aequitas;

use aequitas::systems::si::{
    levels::{AmplitudeLevel, Decibel},
    quantities::ElectricPotential,
    units::Volt,
};

let signal = ElectricPotential::from_unit::<Volt>(2.5);
let reference = ElectricPotential::from_unit::<Volt>(0.25);
let level = AmplitudeLevel::from_ratio(signal, reference).expect("positive voltages");
let db = level.square_law_power().in_unit::<Decibel>();
assert!((db - 20.0_f64).abs() <= 20.0 * f64::EPSILON);
```

Python exposes `AmplitudeLevel` and `PowerLevel` with the same explicit
reference and square-law relationships. The `UNITS` registry supplies the
named and prefixed unit strings for ordinary quantities.

Definitions and notation follow [BIPM SI Brochure, 9th edition, version 4.01, §§2–5](https://doi.org/10.59161/AUEZ1291).
