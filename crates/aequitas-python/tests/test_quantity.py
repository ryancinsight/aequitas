"""Python-side behaviour of the built wheel.

These exercise the surface as a consumer meets it: the importable package, the
generated constructors, and the operator protocol. The Rust suite covers the
same laws from the other side; what only this suite can reach is whether the
wheel imports, whether the constructors were generated, and whether Python's
operator dispatch lands where it should.
"""

from __future__ import annotations

import math

import pytest

import pyaequitas as aq


def test_every_named_quantity_has_a_constructor() -> None:
    names = aq.quantity_names()
    assert len(names) == 81
    for name in names:
        assert callable(getattr(aq, name)), name


def test_construction_scales_into_base_units() -> None:
    assert aq.length(2.5, "mm").base == 0.0025
    assert aq.length(1.0, "km").base == 1000.0
    assert aq.mass(250.0, "g").base == 0.25


def test_construction_without_a_unit_takes_base_units() -> None:
    assert aq.length(3.0).base == 3.0


def test_in_unit_inverts_construction() -> None:
    value = 12.5
    assert aq.length(value, "mm").in_unit("mm") == value
    assert aq.pressure(value, "kPa").in_unit("kPa") == value


@pytest.mark.parametrize(
    ("left", "right", "expected"),
    [
        ("length", "length", "area"),
        ("velocity", "time", "length"),
        ("pressure", "area", "force"),
    ],
)
def test_multiplication_lands_on_the_derived_quantity(
    left: str, right: str, expected: str
) -> None:
    product = getattr(aq, left)(2.0) * getattr(aq, right)(3.0)
    assert product.quantity == expected
    assert product.base == 6.0


def test_division_lands_on_the_derived_quantity() -> None:
    speed = aq.length(10.0, "m") / aq.time(2.0, "s")
    assert speed.quantity == "velocity"
    assert speed.in_unit("m/s") == 5.0


def test_thermal_diffusivity_derives_from_its_constituents() -> None:
    # The worked example from the Aequitas README, through Python.
    density = aq.mass_density(1000.0)
    heat_capacity = aq.specific_heat_capacity(4000.0)
    conductivity = aq.thermal_conductivity(0.6)

    diffusivity = conductivity / (density * heat_capacity)

    assert diffusivity.dimension == aq.dimension_of("thermal_diffusivity")
    assert diffusivity.base == pytest.approx(1.5e-7)


def test_a_dimension_does_not_identify_a_quantity() -> None:
    # Seven dimensions carry several aliases. `m^2/s` is the widest: a derived
    # value reports the first name, and `quantities` reports all of them, so a
    # caller is never handed one alias as though it were an identity.
    diffusivity = aq.thermal_conductivity(0.6) / (
        aq.mass_density(1000.0) * aq.specific_heat_capacity(4000.0)
    )
    assert diffusivity.quantities == [
        "area_per_time",
        "thermal_diffusivity",
        "kinematic_viscosity",
    ]
    assert diffusivity.quantity == "area_per_time"
    assert aq.absorbed_dose(1.0).quantities == ["absorbed_dose", "specific_energy"]


def test_an_unnamed_dimension_reports_no_names() -> None:
    exotic = aq.length(1.0) * aq.length(1.0) * aq.length(1.0) * aq.length(1.0)
    assert exotic.quantities == []
    assert exotic.quantity is None


def test_adding_unlike_dimensions_raises() -> None:
    with pytest.raises(ValueError, match="dimensions differ"):
        aq.length(1.0, "m") + aq.time(1.0, "s")


def test_stress_and_pressure_do_not_add() -> None:
    # Identical exponents; only the semantic marker separates them.
    assert aq.stress(1.0, "Pa").dimension[0] == aq.pressure(1.0, "Pa").dimension[0]
    assert aq.stress(1.0, "Pa").dimension[1] == "stress"
    assert aq.pressure(1.0, "Pa").dimension[1] == "base"
    with pytest.raises(ValueError):
        aq.stress(1.0, "Pa") + aq.pressure(1.0, "Pa")


def test_absolute_temperature_and_difference_do_not_add() -> None:
    with pytest.raises(ValueError):
        aq.thermodynamic_temperature(310.0) + aq.temperature_difference(2.0)


def test_an_angle_is_dimensionless_without_being_a_bare_scalar() -> None:
    angle = aq.angle(math.pi, "rad")
    assert angle.is_dimensionless
    assert angle.dimension[1] == "angle"
    with pytest.raises(ValueError):
        angle + aq.dimensionless(1.0)


def test_degrees_convert_through_the_rust_scale() -> None:
    assert aq.angle(180.0, "deg").base == pytest.approx(math.pi)
    assert aq.angle(math.pi, "rad").in_unit("deg") == pytest.approx(180.0)


def test_scalar_multiplication_is_commutative() -> None:
    length = aq.length(3.0)
    assert (2.0 * length).base == (length * 2.0).base == 6.0
    assert (2.0 * length).quantity == "length"


def test_scalar_over_quantity_inverts_the_dimension() -> None:
    assert (1.0 / aq.time(4.0)).quantity == "frequency"
    assert (1.0 / aq.time(4.0)).base == 0.25


def test_roots_and_powers() -> None:
    assert (aq.length(2.0) ** 3).quantity == "volume"
    assert aq.area(9.0).sqrt().quantity == "length"
    assert aq.volume(-8.0).cbrt().base == -2.0
    with pytest.raises(ValueError, match="not a multiple of 2"):
        aq.length(4.0).sqrt()


def test_a_fractional_power_is_refused() -> None:
    with pytest.raises(ValueError, match="integer power"):
        aq.length(4.0) ** 0.5


def test_ordering_within_a_dimension_and_refusal_across_one() -> None:
    assert aq.length(1.0) < aq.length(2.0)
    assert aq.length(2.0) >= aq.length(2.0)
    with pytest.raises(ValueError):
        _ = aq.length(1.0) < aq.time(1.0)


def test_equality_is_total() -> None:
    assert aq.length(1.0) == aq.length(1.0)
    assert aq.length(1.0) != aq.time(1.0)
    assert aq.length(1.0) != "not a quantity"


def test_equal_quantities_hash_equally() -> None:
    assert hash(aq.length(1.5)) == hash(aq.length(1.5))
    assert len({aq.length(1.0), aq.length(1.0), aq.length(2.0)}) == 2


def test_division_by_zero_raises() -> None:
    with pytest.raises(ZeroDivisionError):
        aq.length(1.0) / aq.time(0.0)


def test_unknown_unit_names_the_known_ones() -> None:
    with pytest.raises(ValueError) as caught:
        aq.length(1.0, "furlong")
    assert "mm" in str(caught.value)


def test_a_unit_of_another_dimension_does_not_resolve() -> None:
    with pytest.raises(ValueError):
        aq.length(1.0, "Pa")


def test_shared_symbols_resolve_per_quantity() -> None:
    # `Pa` is registered for both, and resolves to a different dimension each
    # time -- the reason unit lookup is quantity-scoped.
    assert "Pa" in aq.unit_symbols("pressure")
    assert "Pa" in aq.unit_symbols("stress")
    assert aq.pressure(1.0, "Pa").dimension != aq.stress(1.0, "Pa").dimension


class ForeignQuantity:
    """An object from outside this package that satisfies the protocol."""

    def __init__(self, base: float, dimension: object) -> None:
        self.__aequitas_base__ = base
        self.__aequitas_dimension__ = dimension


def test_a_foreign_object_participates_in_arithmetic() -> None:
    foreign = ForeignQuantity(2.0, ((0, 0, 1, 0, 0, 0, 0), "base"))
    speed = aq.length(8.0) / foreign
    assert speed.quantity == "velocity"
    assert speed.base == 4.0


def test_base_value_of_validates_the_dimension() -> None:
    foreign = ForeignQuantity(1500.0, ((1, 0, -1, 0, 0, 0, 0), "base"))
    assert aq.base_value_of(foreign, "velocity") == 1500.0
    with pytest.raises(ValueError, match="expected length"):
        aq.base_value_of(foreign, "length")


def test_base_value_of_accepts_a_native_quantity() -> None:
    assert aq.base_value_of(aq.length(2.5, "mm"), "length") == 0.0025


def test_a_non_conforming_object_is_a_type_error() -> None:
    with pytest.raises(TypeError, match="__aequitas_base__"):
        aq.base_value_of("not a quantity", "length")


def test_a_malformed_tag_is_rejected() -> None:
    with pytest.raises(ValueError):
        aq.base_value_of(ForeignQuantity(1.0, ((1, 0, 0), "base")), "length")
    with pytest.raises(ValueError):
        aq.base_value_of(
            ForeignQuantity(1.0, ((1, 0, 0, 0, 0, 0, 0), "vibes")), "length"
        )


def test_the_protocol_attributes_are_exported_under_their_own_names() -> None:
    assert aq.BASE_ATTR == "__aequitas_base__"
    assert aq.DIMENSION_ATTR == "__aequitas_dimension__"
    assert aq.PROTOCOL_VERSION == 1
    quantity = aq.length(1.0)
    assert getattr(quantity, aq.BASE_ATTR) == 1.0
    assert getattr(quantity, aq.DIMENSION_ATTR) == quantity.dimension


def test_dimension_of_matches_a_constructed_quantity() -> None:
    assert aq.dimension_of("velocity") == aq.velocity(1.0).dimension


def test_repr_and_str_are_informative() -> None:
    assert repr(aq.length(2.0)) == "Quantity(2, 'length')"
    assert str(aq.length(2.0)) == "2 m"
    assert "[stress]" in str(aq.stress(1.0, "Pa"))


def test_the_package_is_typed() -> None:
    import pyaequitas

    root = __import__("pathlib").Path(pyaequitas.__file__).parent
    assert (root / "py.typed").exists()
    assert (root / "__init__.pyi").exists()
    assert (root / "_pyaequitas.pyi").exists()
