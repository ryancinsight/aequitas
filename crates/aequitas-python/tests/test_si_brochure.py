"""Exercise the brochure surface through the installed wheel."""

import math

import pyaequitas as aq
import pytest


def test_named_and_prefixed_units() -> None:
    assert aq.length(1.0, "au").in_unit("m") == 149_597_870_700.0
    assert aq.angle(1.0, "°").in_unit("″") == pytest.approx(3600.0)
    assert aq.angle(1.0, "mas").in_unit("as") == pytest.approx(0.001)
    assert aq.mass(2.0, "kg").in_unit("g") == 2000.0
    assert aq.volume(3.0, "mL").in_unit("ml") == 3.0
    assert aq.unit_symbols("length").count("mm") == 1
    assert aq.length(2.0, "qm").base == 2e-30
    assert aq.inductance(2.0, "QH").base == 2e30
    assert aq.information(2.0, "Qibit").base == 2.0**101
    assert aq.dimensionless(2.0, "%").in_unit("ppm") == 20_000.0

    assert aq.thermodynamic_temperature(0.0, "°C").base == 273.15
    assert aq.thermodynamic_temperature(1000.0, "m°C").base == 274.15
    assert aq.temperature_difference(1000.0, "m°C").base == 1.0
    assert aq.reactive_power(4.0, "var").in_unit("V·A") == 4.0
    with pytest.raises(ValueError):
        aq.angular_frequency(1.0, "Hz")
    with pytest.raises(ValueError):
        aq.mass(1.0, "mkg")


def test_logarithmic_levels_use_an_explicit_reference_and_kind() -> None:
    signal = aq.electric_potential(3.0, "V")
    reference = aq.electric_potential(300.0, "mV")
    level = aq.AmplitudeLevel.from_ratio(signal, reference)
    assert level.in_unit("Np") == pytest.approx(math.log(10.0))
    power = level.square_law_power()
    assert power.in_unit("dB") == pytest.approx(20.0)
    assert power.in_unit("B") == pytest.approx(2.0)
    assert power.ratio() == pytest.approx(100.0)
    assert power.square_law_amplitude().ratio() == pytest.approx(10.0)
    with pytest.raises(ValueError, match="same quantity kind"):
        aq.PowerLevel.from_ratio(aq.power(1.0, "W"), reference)
    with pytest.raises(ValueError, match="positive"):
        aq.AmplitudeLevel.from_ratio(signal, aq.electric_potential(0.0, "V"))
    with pytest.raises(ValueError, match="amplitude"):
        level.in_unit("dB")
