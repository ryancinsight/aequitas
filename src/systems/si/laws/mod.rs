//! Quantity identities that require more information than a common dimension.
//!
//! One leaf per physical domain; each leaf holds inherent methods on the
//! quantity types it extends, so no re-export is needed. The pre-existing
//! rotation bridge and dose-equivalent constructor live in `rotation` and
//! `radiation` respectively.

pub mod acoustics;
pub mod chemistry;
pub mod electromagnetism;
pub mod equilibrium;
pub mod fluid;
pub mod mechanics;
pub mod radiation;
pub mod radiation_thermal;
pub mod rotation;
pub mod thermal;
pub mod waves;
