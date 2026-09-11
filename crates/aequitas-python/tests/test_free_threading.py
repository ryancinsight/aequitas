"""The module is safe without the GIL, and says so.

A free-threaded interpreter re-enables the GIL for the whole process when it
imports an extension that does not declare itself safe. The declaration rests
on the module holding no shared mutable state -- every class is frozen over
plain data, and the unit and class tables are constants -- so these tests
check both halves: the declaration takes, and concurrent use of shared
quantities gives the results sequential use does.
"""

from __future__ import annotations

import subprocess
import sys
import sysconfig
from concurrent.futures import ThreadPoolExecutor

import pytest

import pyaequitas as aq

FREE_THREADED = bool(sysconfig.get_config_var("Py_GIL_DISABLED"))


@pytest.mark.skipif(not FREE_THREADED, reason="a GIL build has no GIL to keep off")
def test_importing_the_module_leaves_the_gil_off() -> None:
    # A fresh interpreter, so nothing else this session imported can be what
    # turned the GIL back on -- and without PYTHON_GIL=0, which would keep it
    # off whatever the module declared and so prove nothing.
    env = {key: value for key, value in __import__("os").environ.items() if key != "PYTHON_GIL"}
    result = subprocess.run(
        [sys.executable, "-c", "import sys, pyaequitas; print(sys._is_gil_enabled())"],
        capture_output=True,
        text=True,
        check=True,
        env=env,
    )
    assert result.stdout.strip() == "False", result.stderr


def _work(seed: int, length: aq.Length, time: aq.Time) -> tuple[float, str]:
    """Arithmetic on shared quantities, returning an exact summary."""
    total = 0.0
    kind = ""
    for step in range(2000):
        speed = length * float(seed + step) / time
        total += (speed * time).base
        kind = type(speed).__name__
    return total, kind


def test_shared_quantities_give_exact_results_across_threads() -> None:
    length = aq.length(1.5)
    time = aq.time(0.5)
    seeds = range(32)
    with ThreadPoolExecutor(max_workers=8) as pool:
        concurrent = list(pool.map(lambda seed: _work(seed, length, time), seeds))
    sequential = [_work(seed, length, time) for seed in seeds]
    # The same operations on the same operands: bitwise equal, not approximately.
    assert concurrent == sequential
    assert {kind for _, kind in concurrent} == {"Velocity"}


def test_every_class_instantiates_concurrently() -> None:
    names = aq.quantity_names()

    def build(name: str) -> type:
        return type(getattr(aq, name)(1.0) * 2.0)

    with ThreadPoolExecutor(max_workers=8) as pool:
        concurrent = list(pool.map(build, names))
    assert concurrent == [build(name) for name in names]
