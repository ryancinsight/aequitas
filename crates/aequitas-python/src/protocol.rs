//! The cross-extension quantity protocol.
//!
//! Two independently built extension modules do not share Rust types: a
//! quantity constructed in `pyaequitas._pyaequitas` is an opaque object to any
//! other module unless both link the same `aequitas-python`. Requiring that
//! would chain every consumer's release to this wheel's version, and its
//! failure mode is an error naming the same class on both sides.
//!
//! So the contract is structural. An object is a quantity when it exposes a
//! magnitude in canonical SI base units and a dimension tag, both under the
//! names below. A consumer compares the tag and reads the magnitude; nothing
//! about this crate's layout is part of the agreement.

/// Attribute carrying the magnitude in canonical SI base units, as a float.
pub const BASE_ATTR: &str = "__aequitas_base__";

/// Attribute carrying the dimension tag as `((int x 7), str)`.
///
/// The exponents are SI base powers in length, mass, time, current,
/// temperature, amount, luminosity order. The string is the semantic
/// discriminant, which separates dimensions the exponents alone cannot:
/// stress from pressure, an absolute temperature from a difference.
pub const DIMENSION_ATTR: &str = "__aequitas_dimension__";

/// Version of the protocol these attribute shapes describe.
///
/// A consumer may gate on this; it changes only when the tag shape changes,
/// which is a breaking change for every consumer that reads it.
pub const PROTOCOL_VERSION: u32 = 1;
