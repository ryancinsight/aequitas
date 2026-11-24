use crate::units::registry::{Quantities, Quantity};

use super::{head, tail};

const HEAD: &[Quantity] = head::Inventory::ALL;
const TAIL: &[Quantity] = tail::Inventory::ALL;
const SIZE: usize = HEAD.len() + TAIL.len();

/// The generated unit catalog, with conversion metadata supplied by Aequitas.
pub struct Inventory;

impl Quantities for Inventory {
    const ALL: &'static [Quantity] = &join();
}

const fn join() -> [Quantity; SIZE] {
    // The generator requires both leaves to be non-empty and emits them
    // in declaration order. Joining at compile time retains the single
    // immutable slice (and its original resolution order) at runtime.
    let mut entries = [HEAD[0]; SIZE];
    let mut index = 0;
    while index < SIZE {
        entries[index] = if index < HEAD.len() {
            HEAD[index]
        } else {
            TAIL[index - HEAD.len()]
        };
        index += 1;
    }
    entries
}
