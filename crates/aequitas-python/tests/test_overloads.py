"""The stubs' arithmetic overloads are the runtime's algebra.

`_pyaequitas.pyi` declares, per class, which class each product, quotient,
power and root returns. The generator derives those from the dimension
declarations with its own copy of the tag rules, so this checks the shipped
stub against the extension in both directions: every declared result is the
class the runtime returns, and every product or quotient the runtime names is
declared -- a missing overload would type a named result as plain `Quantity`,
which a checker then refuses to add to the class it really is.
"""

from __future__ import annotations

import itertools
import re
import subprocess
import sys
from pathlib import Path

import pytest

import pyaequitas as aq

STUB = Path(aq.__file__).with_name("_pyaequitas.pyi")
CLASS_RE = re.compile(r"^class (\w+)\(Quantity\):$")
BINARY_RE = re.compile(
    r"^    def (__mul__|__rmul__|__truediv__|__rtruediv__)\(self, other: (\w+)\) -> (\w+): \.\.\.$"
)
POWER_RE = re.compile(
    r"^    def __pow__\(self, exponent: Literal\[(-?\d+)\], modulo: None = \.\.\.\) -> (\w+): \.\.\.$"
)
UNARY_RE = re.compile(r"^    def (sqrt|cbrt|reciprocal)\(self\) -> (\w+): \.\.\.$")

# One sample per class, built through the public constructors. The magnitude
# is nonzero and positive so every quotient, root and reciprocal is defined.
SAMPLES = {type(getattr(aq, name)(1.5)): getattr(aq, name)(1.5) for name in aq.quantity_names()}


def _declarations() -> list[tuple[str, str, object, str]]:
    """`(class, method, operand, result)` for every precise overload in the stub."""
    declared: list[tuple[str, str, object, str]] = []
    current = None
    for line in STUB.read_text(encoding="utf-8").splitlines():
        match = CLASS_RE.match(line)
        if match:
            current = match[1]
            continue
        if line and not line.startswith(" "):
            current = None
        if current is None:
            continue
        if match := BINARY_RE.match(line):
            declared.append((current, match[1], match[2], match[3]))
        elif match := POWER_RE.match(line):
            declared.append((current, "__pow__", int(match[1]), match[2]))
        elif match := UNARY_RE.match(line):
            declared.append((current, match[1], None, match[2]))
    return declared


def _sample(name: str) -> aq.Quantity:
    return SAMPLES[aq.CLASSES[name]]


def _apply(value: aq.Quantity, method: str, operand: object) -> object:
    if method in ("sqrt", "cbrt", "reciprocal"):
        return getattr(value, method)()
    if method == "__pow__":
        return value**operand
    other = 2.0 if operand == "float" else _sample(str(operand))
    return getattr(value, method)(other)


def test_the_stub_declares_overloads() -> None:
    kinds = {method for _, method, _, _ in _declarations()}
    assert kinds == {"__mul__", "__rmul__", "__truediv__", "__rtruediv__", "__pow__", "sqrt", "cbrt", "reciprocal"}


@pytest.mark.parametrize(("cls", "method", "operand", "result"), _declarations())
def test_each_declared_result_is_the_class_the_runtime_returns(
    cls: str, method: str, operand: object, result: str
) -> None:
    assert type(_apply(_sample(cls), method, operand)) is aq.CLASSES[result]


def test_every_named_product_and_quotient_is_declared() -> None:
    declared = {
        (cls, method, operand): result
        for cls, method, operand, result in _declarations()
        if method in ("__mul__", "__truediv__") and operand != "float"
    }
    classes = sorted(set(aq.CLASSES.values()), key=lambda cls: cls.__name__)
    wrong = []
    for left, right, method in itertools.product(classes, classes, ("__mul__", "__truediv__")):
        produced = type(getattr(SAMPLES[left], method)(SAMPLES[right]))
        key = (left.__name__, method, right.__name__)
        expected = None if produced is aq.Quantity else produced.__name__
        if declared.get(key) != expected:
            wrong.append(f"{key}: runtime {produced.__name__}, stub {declared.get(key)}")
    assert not wrong, "stub and runtime disagree:\n" + "\n".join(wrong[:20])


def test_the_stubs_themselves_type_check() -> None:
    pytest.importorskip("mypy")
    result = subprocess.run(
        [sys.executable, "-m", "mypy", "--strict", "--no-error-summary", "-p", "pyaequitas"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    assert result.returncode == 0, result.stdout + result.stderr
