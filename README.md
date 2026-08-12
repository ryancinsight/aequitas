# Aequitas

Aequitas is the Atlas physical-quantity and dimensional-law foundation. It
encodes SI dimensions in types, stores quantities in canonical base units, and
monomorphizes arithmetic over `T: eunomia::FloatElement`.

The name refers to Aequitas, the Roman personification of equity and fair
measure.

## Boundary

Aequitas owns:

- type-level SI dimensions;
- transparent physical quantities;
- linear SI units and conversion factors;
- dimensional arithmetic over Eunomia scalar types.

Aequitas does not own scalar representations, arrays, solvers, material laws,
domain validation, formatting, persistence, or accelerator execution. Those
remain with Eunomia, Leto, domain packages, Consus, and Hephaestus.

The SI surface includes thermophysical dimensions needed by material
providers. Thermal diffusivity is derived without a raw-scalar escape:

```rust
use aequitas::systems::si::quantities::{
    MassDensity, SpecificHeatCapacity, ThermalConductivity, ThermalDiffusivity,
};

let density = MassDensity::from_base(1_000.0_f64);
let heat_capacity = SpecificHeatCapacity::from_base(4_000.0_f64);
let conductivity = ThermalConductivity::from_base(0.6_f64);
let diffusivity: ThermalDiffusivity = conductivity / (density * heat_capacity);

assert_eq!(diffusivity.into_base(), 1.5e-7);
```

Mechanical specific energy has its own semantic `SpecificEnergy` alias and
`JoulePerKilogram` unit. It shares the coherent `J/kg` dimension with absorbed
dose while keeping turbulence and mechanical contracts distinct from radiation
dosimetry.

Fluid and acoustic transport laws retain their dimensions through the same
arithmetic. Dynamic viscosity divided by density yields kinematic viscosity,
and planar flow per unit width uses the named area-per-time contract,
volumetric flow divided by area yields velocity, and absorption times
intensity yields volumetric power density; pressure times area yields force.
Pressure-per-current transducer
gain, quadratic hydraulic resistance, and hydraulic conductance are named
dimensions as well, so nonlinear flow and device-response coefficients cannot
be substituted across contracts:

```rust
use aequitas::systems::si::{
    quantities::{
        Area, DynamicViscosity, Intensity, KinematicViscosity, MassDensity,
        ReciprocalLength, Velocity, VolumetricFlowRate, VolumetricPowerDensity,
    },
    units::{
        CubicMeterPerSecond, KilogramPerCubicMeter, PascalSecond, SquareMeter,
        WattPerSquareMeter,
    },
};

let dynamic = DynamicViscosity::from_unit::<PascalSecond>(0.004_f64);
let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
let kinematic: KinematicViscosity = dynamic / density;
let flow = VolumetricFlowRate::from_unit::<CubicMeterPerSecond>(0.002_f64);
let area = Area::from_unit::<SquareMeter>(0.01_f64);
let velocity: Velocity = flow / area;
let intensity = Intensity::from_unit::<WattPerSquareMeter>(12.0_f64);
let absorption = ReciprocalLength::from_base(2.0_f64);
let power_density: VolumetricPowerDensity = absorption * intensity;

assert_eq!(kinematic.into_base(), 4.0e-6);
assert_eq!(velocity.into_base(), 0.2);
assert_eq!(power_density.into_base(), 24.0);
```

Entity concentration has its own number-density contract, so population
models do not pass a raw scalar where a mass density is expected:

```rust
use aequitas::systems::si::{
    quantities::NumberDensity,
    units::PerCubicMeter,
};

let concentration = NumberDensity::from_unit::<PerCubicMeter>(1.0e12_f64);
assert_eq!(concentration.into_base(), 1.0e12);
```

Volumetric power density divided by mass density yields specific absorption
rate, which is the same coherent dimension as absorbed dose rate. `W/kg` and
`Gy/s` therefore name one axis, so radiofrequency and radiation dosimetry share
a deposition vocabulary without a conversion:

```rust
use aequitas::systems::si::{
    quantities::{AbsorbedDose, MassDensity, SpecificAbsorptionRate, Time, VolumetricPowerDensity},
    units::{GrayPerSecond, KilogramPerCubicMeter, Second, WattPerCubicMeter, WattPerKilogram},
};

let deposition = VolumetricPowerDensity::from_unit::<WattPerCubicMeter>(2_000.0_f64);
let density = MassDensity::from_unit::<KilogramPerCubicMeter>(1_000.0_f64);
let rate: SpecificAbsorptionRate = deposition / density;
let dose: AbsorbedDose = rate * Time::from_unit::<Second>(60.0_f64);

assert_eq!(rate.in_unit::<WattPerKilogram>(), 2.0);
assert_eq!(rate.in_unit::<GrayPerSecond>(), 2.0);
assert_eq!(dose.into_base(), 120.0);
```

Temperature-response coefficients retain their inverse-temperature dimensions:

```rust
use aequitas::systems::si::{
    quantities::{Dimensionless, ReciprocalTemperature, TemperatureDifference},
    units::{Kelvin, PerKelvin},
};

let slope = ReciprocalTemperature::from_unit::<PerKelvin>(0.01_f64);
let delta = TemperatureDifference::from_unit::<Kelvin>(5.0_f64);
let response: Dimensionless = slope * delta;

assert_eq!(response.into_base(), 0.05);
```

Interfacial tension is distinct from energy per area even though the SI base
exponents coincide. This prevents a surface-tension input from being confused
with an areal energy while retaining dimensional pressure recovery:

```rust
use aequitas::systems::si::{
    quantities::{Length, Pressure, SurfaceTension},
    units::{Meter, NewtonPerMeter, Pascal},
};

let tension = SurfaceTension::from_unit::<NewtonPerMeter>(0.072_f64);
let radius = Length::from_unit::<Meter>(2.0e-3_f64);
let pressure: Pressure = tension / radius;

assert_eq!(pressure.in_unit::<Pascal>(), 36.0);
```

Angles are represented as a distinct dimensionless semantic quantity and are
stored in radians:

```rust
use aequitas::systems::si::{quantities::Angle, units::Radian};

let quarter_turn = Angle::from_unit::<Radian>(core::f64::consts::FRAC_PI_2);
assert_eq!(quarter_turn.in_unit::<Radian>(), core::f64::consts::FRAC_PI_2);
```

Electrical contracts compose current, time, and potential without raw unit
conventions:

```rust
use aequitas::systems::si::{
    quantities::{Capacitance, ElectricCharge, ElectricCurrent, ElectricPotential, Time},
    units::{Ampere, Coulomb, Farad, Second, Volt},
};

let current = ElectricCurrent::from_unit::<Ampere>(2.0_f64);
let charge: ElectricCharge = current * Time::from_unit::<Second>(3.0_f64);
let potential = ElectricPotential::from_unit::<Volt>(5.0_f64);
let capacitance: Capacitance = charge / potential;

assert_eq!(charge.in_unit::<Coulomb>(), 6.0);
assert_eq!(capacitance.in_unit::<Farad>(), 1.2);
```

Absolute temperature and temperature difference use distinct semantic markers.
Subtracting two absolute temperatures produces a difference; adding a
difference to an absolute temperature produces an absolute temperature:

```rust
use aequitas::systems::si::{
    quantities::{TemperatureDifference, ThermodynamicTemperature},
    units::Kelvin,
};

let lower = ThermodynamicTemperature::from_unit::<Kelvin>(290.0_f64);
let upper = ThermodynamicTemperature::from_unit::<Kelvin>(300.0_f64);
let delta: TemperatureDifference = upper - lower;
let restored = lower + delta;

assert_eq!(delta.in_unit::<Kelvin>(), 10.0);
assert_eq!(restored.in_unit::<Kelvin>(), 300.0);
```

Energy density is available as a first-class quantity for acoustic, thermal,
and cavitation metrics:

```rust
use aequitas::systems::si::{
    quantities::{Energy, EnergyPerVolume, Volume},
    units::{CubicMeter, Joule, JoulePerCubicMeter},
};

let energy = Energy::from_unit::<Joule>(12.0_f64);
let volume = Volume::from_unit::<CubicMeter>(3.0_f64);
let density: EnergyPerVolume = energy / volume;

assert_eq!(density.in_unit::<JoulePerCubicMeter>(), 4.0);
```

Biological-response models can state dose and kinetic parameters without raw
unit conventions:

```rust
use aequitas::systems::si::{
    quantities::{AbsorbedDose, MolarEnergy, MolarHeatCapacity, ReciprocalTime},
    units::{Gray, JoulePerMole, JoulePerMoleKelvin, PerSecond},
};

let dose = AbsorbedDose::from_unit::<Gray>(2.0_f64);
let activation = MolarEnergy::from_unit::<JoulePerMole>(284_000.0_f64);
let gas_constant =
    MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(8.314_462_618_153_24_f64);
let frequency_factor = ReciprocalTime::from_unit::<PerSecond>(1.0e44_f64);

assert_eq!(dose.into_base(), 2.0);
assert_eq!(activation.into_base(), 284_000.0);
assert_eq!(gas_constant.into_base(), 8.314_462_618_153_24);
assert_eq!(frequency_factor.into_base(), 1.0e44);
```

Photon-interaction contracts compose attenuation, material density, and path
length without raw unit conventions:

```rust
use aequitas::systems::si::{
    quantities::{AreaPerMass, Dimensionless, Length, MassDensity, ReciprocalLength},
    units::{GramPerCubicCentimeter, Meter, SquareCentimeterPerGram},
};

let mass_attenuation =
    AreaPerMass::from_unit::<SquareCentimeterPerGram>(0.06_f64);
let density = MassDensity::from_unit::<GramPerCubicCentimeter>(1.0_f64);
let attenuation: ReciprocalLength = mass_attenuation * density;
let path = Length::from_unit::<Meter>(0.1_f64);
let optical_depth: Dimensionless = attenuation * path;

assert_eq!(attenuation.into_base(), 6.0);
// Three multiplicative conversions contribute at most gamma_3; 4ε is a
// conservative first-order bound because 3ε / (1 - 3ε) < 4ε.
assert!((optical_depth.into_base() - 0.6).abs() <= 4.0 * f64::EPSILON * 0.6);
```

## Example

```rust
use aequitas::quantity::Quantity;
use aequitas::systems::si::{
    dimensions,
    quantities::{Length, Time, Velocity},
    units::{Meter, MeterPerSecond, Second},
};

let distance = Length::from_unit::<Meter>(1500.0_f64);
let duration = Time::from_unit::<Second>(1.0_f64);
let velocity: Velocity = distance / duration;

assert_eq!(velocity.in_unit::<MeterPerSecond>(), 1500.0);

// The generic representation remains available when a named alias is absent.
let _: Quantity<f64, dimensions::Velocity> = velocity;
```

Adding a length to a time is rejected at compile time:

```compile_fail
use aequitas::systems::si::{
    quantities::{Length, Time},
    units::{Meter, Second},
};

let length = Length::from_unit::<Meter>(1.0_f64);
let time = Time::from_unit::<Second>(1.0_f64);
let _invalid = length + time;
```

## Architecture

```text
src/
├── dimension/
│   ├── algebra.rs       # type-level dimension composition
│   └── model.rs         # seven-axis SI exponent vector and semantics
├── quantity/
│   ├── arithmetic/      # additive, multiplicative, scalar, unary operations
│   ├── construction.rs  # base/unit boundary
│   └── model.rs         # transparent Quantity<T, D>
├── systems/
│   └── si/
│       ├── dimensions.rs
│       ├── quantities.rs
│       └── units/       # base, scaled, and derived ZST unit markers
└── unit/
    └── linear.rs        # sealed linear-unit contract and conversion SSOT
```

`lib.rs` and every `mod.rs` are manifests. Operation families live in leaf
modules. Unit markers and dimensions are zero-sized; `Quantity<T, D>` is
`#[repr(transparent)]` over `T`.

## `uom` gap analysis

[`uom` 0.38.0](https://docs.rs/uom/0.38.0/uom/) is the comparison baseline and
remains a development-only differential oracle.

| Capability | `uom` 0.38.0 | Aequitas 0.1 scope |
| --- | --- | --- |
| Compile-time dimensional analysis | Mature, broad implementation | Required; implemented through one generic dimension algebra |
| SI and non-SI breadth | Extensive | Deliberately limited to current Atlas consumers |
| Storage types | Closed macro-generated set of primitive, integer, rational, and complex types | Real quantities over Eunomia's `UnitScalar` implementations; complex phasors over `eunomia::Complex32`/`Complex64` |
| Atlas datatype SSOT | Uses `num-traits` storage contracts | Uses Eunomia directly; defines no scalar vocabulary |
| API variation | Generates storage-specific modules such as `si::f32` and `si::f64` | One `Quantity<T, D>` API with inferred or defaulted `T` |
| `no_std` | Supported | Supported |
| Affine units and quantity kinds | Supported | Not in the initial linear-unit slice |
| Formatting and serialization | Supported | `UnitDisplay` (unit-aware `Display`/`Debug`) and optional serde (canonical scalar) supported |
| Integer and rational storage | Supported | Outside the floating-point simulation boundary |

The architectural decision and source-level comparison are recorded in
[ADR 0001](docs/adr/0001-aequitas-quantity-law.md).

## Verification

The committed gates are:

```sh
cargo fmt --all -- --check
cargo check --locked --no-default-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo nextest run --locked --all-features
cargo test --locked --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
cargo deny check
```

The release-mode codegen fixture compares typed velocity arithmetic with raw
scalar division. Layout tests prove the dimension and unit markers are
zero-sized and `Quantity<T, D>` has the size and alignment of `T`.

## License

Licensed under either the MIT License or Apache License 2.0.
