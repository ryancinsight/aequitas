//! Acceptance laws from the BIPM SI Brochure, 9th edition, version 4.01.

mod support;

#[path = "si_brochure/coherent.rs"]
mod coherent;
#[path = "si_brochure/non_si.rs"]
mod non_si;
#[path = "si_brochure/prefixes.rs"]
mod prefixes;
