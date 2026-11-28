//! Value identities for the named quantity laws in `systems::si::laws`.
//!
//! Where `dimension_laws` pins dimensional closure with exact binary values,
//! this tree pins the closed forms of the named laws themselves: Reynolds,
//! Pennes, Beer–Lambert, Arrhenius, Ohm, Newton II and their companions.
//! One leaf per law domain, wired by `#[path]` into one integration binary.

#[path = "quantity_laws/acoustics.rs"]
mod acoustics;

#[path = "quantity_laws/chemistry.rs"]
mod chemistry;

#[path = "quantity_laws/electromagnetism.rs"]
mod electromagnetism;

#[path = "quantity_laws/equilibrium.rs"]
mod equilibrium;

#[path = "quantity_laws/fluid.rs"]
mod fluid;

#[path = "quantity_laws/mechanics.rs"]
mod mechanics;

#[path = "quantity_laws/radiation.rs"]
mod radiation;

#[path = "quantity_laws/radiation_thermal.rs"]
mod radiation_thermal;

#[path = "quantity_laws/thermal.rs"]
mod thermal;

#[path = "quantity_laws/waves.rs"]
mod waves;
