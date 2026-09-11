"""Every quantity is an instance of its dimension's class, as the stubs say.

The stubs declare a class per named dimension and type each constructor as
returning one. That claim is worth something only if the runtime produces
those classes, so this reads the shipped stub and checks it against the
objects the extension returns, rather than trusting the generator that wrote
both.
"""

from __future__ import annotations

from pathlib import Path

import pytest

import pyaequitas as aq

STUB = Path(aq.__file__).with_name("__init__.pyi")


def _declared_constructors() -> list[tuple[str, str]]:
    """`(function, returned class)` for every constructor the package stub declares."""
    declared = []
    for line in STUB.read_text(encoding="utf-8").splitlines():
        if line.startswith("def ") and "(value: float" in line:
            name = line[len("def ") : line.index("(")]
            returned = line.rsplit("-> ", 1)[1].split(":")[0]
            declared.append((name, returned))
    return declared


def test_the_stub_declares_a_constructor_per_quantity() -> None:
    declared = sorted(name for name, _ in _declared_constructors())
    assert declared == sorted(aq.quantity_names())


@pytest.mark.parametrize(("name", "returned"), _declared_constructors())
def test_each_constructor_returns_the_class_its_stub_declares(name: str, returned: str) -> None:
    assert type(getattr(aq, name)(1.0)) is getattr(aq, returned)


def test_arithmetic_returns_the_class_of_its_result() -> None:
    length = aq.length(2.0)
    assert type(length * length) is aq.Area
    assert type(aq.area(6.0) / length) is aq.Length
    assert type(length + length) is aq.Length
    assert type(length - length) is aq.Length
    assert type(-length) is aq.Length
    assert type(+length) is aq.Length
    assert type(abs(length)) is aq.Length
    assert type(aq.area(4.0).sqrt()) is aq.Length
    assert type(length / aq.time(1.0)) is aq.Velocity
    assert type(2.0 * length) is aq.Length
    assert type(1.0 / aq.time(2.0)) is aq.Frequency


def test_a_dimension_no_class_names_stays_a_plain_quantity() -> None:
    fifth_power = aq.length(2.0) ** 5
    assert type(fifth_power) is aq.Quantity
    assert fifth_power.quantity is None


def test_aliases_of_one_dimension_are_one_class() -> None:
    assert aq.ThermalDiffusivity is aq.AreaPerTime
    assert aq.KinematicViscosity is aq.AreaPerTime
    assert aq.EnergyPerVolume is aq.Pressure
    assert type(aq.energy_per_volume(1.0)) is aq.Pressure
    assert aq.pressure(1.0) == aq.energy_per_volume(1.0)


def test_a_semantic_marker_keeps_classes_apart() -> None:
    assert aq.Stress is not aq.Pressure
    assert type(aq.stress(1.0)) is aq.Stress
    assert aq.ThermodynamicTemperature is not aq.TemperatureDifference


def test_every_class_is_a_quantity_bound_under_its_name() -> None:
    assert len(aq.CLASSES) == len(aq.quantity_names())
    for name, cls in aq.CLASSES.items():
        assert issubclass(cls, aq.Quantity), name
        assert getattr(aq, name) is cls
    assert isinstance(aq.length(1.0), aq.Quantity)
    assert aq.base_value_of(aq.length(3.0), "length") == 3.0
