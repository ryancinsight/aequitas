"""The shipped stubs are checked, not asserted.

`py.typed` plus a `.pyi` is a claim that a type checker can use this package.
This runs mypy over a fixture whose good half must pass and whose bad half must
fail with the exact error codes named there, so a stub that drifts from the
extension is a test failure rather than a surprise in a consumer's editor.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

import pytest

CASES = Path(__file__).parent / "typing" / "cases.py"
MARKER = "# --- MUST FAIL BELOW ---"

def _has_module(name: str) -> bool:
    try:
        __import__(name)
    except ImportError:
        return False
    return True


def _run_mypy() -> str:
    """Type-check the fixture, returning mypy's output.

    `encoding` is explicit: without it `subprocess` decodes through the
    Windows ANSI codepage and drops the output entirely on a non-ASCII byte.
    """
    result = subprocess.run(
        [sys.executable, "-m", "mypy", "--strict", "--no-error-summary", str(CASES)],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    return result.stdout + result.stderr


def _expectations() -> dict[int, str]:
    """Line number to expected error code, for every line below the marker."""
    lines = CASES.read_text(encoding="utf-8").splitlines()
    start = lines.index(MARKER)
    expected: dict[int, str] = {}
    for offset, line in enumerate(lines[start + 1 :], start=start + 2):
        match = re.search(r"#\s*([a-z-]+)$", line.strip())
        if match:
            expected[offset] = match.group(1)
    return expected


@pytest.fixture(scope="module")
def report() -> str:
    if not _has_module("mypy"):
        pytest.skip("mypy is not installed")
    return _run_mypy()


def test_the_fixture_declares_failures() -> None:
    expected = _expectations()
    assert expected, "the fixture lists no expected failures, so it proves nothing"


def test_the_clean_half_type_checks(report: str) -> None:
    first_bad = min(_expectations())
    offenders = [
        line
        for line in report.splitlines()
        if (match := re.search(r"cases\.py:(\d+):", line))
        and int(match.group(1)) < first_bad
    ]
    assert not offenders, "valid usage was rejected by the stubs:\n" + "\n".join(
        offenders
    )


def test_each_bad_line_fails_with_its_declared_code(report: str) -> None:
    reported: dict[int, set[str]] = {}
    for line in report.splitlines():
        match = re.search(r"cases\.py:(\d+):.*\[([a-z-]+)\]", line)
        if match:
            reported.setdefault(int(match.group(1)), set()).add(match.group(2))

    missing = []
    for number, code in _expectations().items():
        if code not in reported.get(number, set()):
            missing.append(f"cases.py:{number}: expected [{code}], got {reported.get(number)}")

    assert not missing, "the stubs did not reject:\n" + "\n".join(missing)
