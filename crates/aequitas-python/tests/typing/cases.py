"""Fixture for the stub check in ``test_typing.py``.

Lines above the marker must type-check cleanly; every line below it must
produce the error code named in its trailing comment. Running mypy over this
file turns "the package ships usable stubs" from a claim into a test.
"""

from typing import List, Optional

import pyaequitas as aq

# --- must type-check clean ---

radius = aq.length(2.5, "mm")
base: float = radius.base
in_cm: float = radius.in_unit("cm")
area: aq.Quantity = radius * radius
scaled: aq.Quantity = 2.0 * radius
names: List[str] = area.quantities
label: Optional[str] = area.quantity
flat: bool = area.is_dimensionless
root: aq.Quantity = area.sqrt()
cubed: aq.Quantity = radius**3
ordered: bool = radius < aq.length(3.0)
no_unit: aq.Quantity = aq.length(1.0)
raw: float = aq.base_value_of(radius, "length")
symbols: List[str] = aq.unit_symbols("pressure")
typed_length: aq.Length = aq.length(1.0)
one_class: aq.ThermalDiffusivity = aq.kinematic_viscosity(1.0)
widened: aq.Quantity = aq.length(1.0)

# --- MUST FAIL BELOW ---

aq.length("two", "mm")  # arg-type
radius.nonexistent  # attr-defined
wrong: str = radius.base  # assignment
radius.in_unit(5)  # arg-type
aq.length(1.0) ** 0.5  # operator
mistyped: aq.Time = aq.length(1.0)  # assignment
