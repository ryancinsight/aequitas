//! Compile-time dimensional identities with exact binary values.
//!
//! One leaf per domain of `systems::si::units`, so a law about a thermal
//! coefficient is read beside the other thermal laws rather than beside a
//! hydraulic one, and a new law has one obvious home. Leaves are wired by
//! `#[path]` so the identities still compile as one integration binary of
//! record, and `complex` and `dosimetry` keep the two axes that cut across
//! domains: value kind, and the radiological dose chain.

#[path = "dimension_laws/angle.rs"]
mod angle;

#[path = "dimension_laws/complex.rs"]
mod complex;

#[path = "dimension_laws/dosimetry.rs"]
mod dosimetry;

#[path = "dimension_laws/electrical.rs"]
mod electrical;

#[path = "dimension_laws/hydraulics.rs"]
mod hydraulics;

#[path = "dimension_laws/kinematics.rs"]
mod kinematics;

#[path = "dimension_laws/mechanics.rs"]
mod mechanics;

#[path = "dimension_laws/radiation.rs"]
mod radiation;

#[path = "dimension_laws/thermal.rs"]
mod thermal;

#[path = "dimension_laws/transport.rs"]
mod transport;
