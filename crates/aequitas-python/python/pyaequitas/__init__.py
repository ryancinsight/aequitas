"""SI physical quantities and dimensional arithmetic, backed by Rust.

Aequitas encodes SI dimensions in Rust types and discharges dimensional
correctness during monomorphization, which erases it before the binary exists.
Python has no compile step, so this package carries the dimension as data: a
magnitude in canonical SI base units alongside the seven SI exponents and a
semantic discriminant. Operations that ``rustc`` would reject raise here.

    >>> import pyaequitas as aq
    >>> radius = aq.length(2.5, "mm")
    >>> radius.base
    0.0025
    >>> area = radius * radius
    >>> area.quantity
    'area'
    >>> aq.length(1.0, "m") + aq.time(1.0, "s")
    Traceback (most recent call last):
        ...
    ValueError: cannot add `m` and `s`: dimensions differ

The semantic discriminant separates quantities the exponents alone cannot --
stress from pressure, an absolute temperature from a temperature difference --
so those do not silently unify either.
"""

from __future__ import annotations

from typing import Callable, Optional

from ._pyaequitas import (
    BASE_ATTR,
    DIMENSION_ATTR,
    PROTOCOL_VERSION,
    UNITS,
    Quantity,
    base_value_of,
    dimension_of,
    quantity_names,
    unit_symbols,
)

__all__ = [
    "BASE_ATTR",
    "DIMENSION_ATTR",
    "PROTOCOL_VERSION",
    "UNITS",
    "Quantity",
    "base_value_of",
    "dimension_of",
    "quantity_names",
    "unit_symbols",
]


def _constructor(name: str) -> Callable[[float, Optional[str]], Quantity]:
    """Build the constructor for one named quantity.

    Generated from the extension's own inventory rather than a second list, so
    a quantity added upstream appears here by rebuilding and cannot be spelled
    differently in the two places.
    """

    def construct(value: float, unit: Optional[str] = None) -> Quantity:
        if unit is None:
            return Quantity.from_base_of(value, name)
        return Quantity.from_unit(value, unit, quantity=name)

    construct.__name__ = name
    symbols = ", ".join(UNITS[name]) or "none (base units only)"
    construct.__doc__ = (
        f"Construct a {name.replace('_', ' ')} from a value in ``unit``.\n\n"
        f"Units: {symbols}.\n"
        "Omit ``unit`` to give the value in canonical SI base units."
    )
    return construct


for _name in quantity_names():
    globals()[_name] = _constructor(_name)
    __all__.append(_name)

del _name
