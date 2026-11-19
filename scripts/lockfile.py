#!/usr/bin/env python3
"""Verify the crates.io scalar provider and regenerate the lock outside Atlas."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import subprocess
import sys
import tempfile

REPOSITORY = Path(__file__).resolve().parent.parent
LOCKFILE = REPOSITORY / "Cargo.lock"
MANIFEST = REPOSITORY / "Cargo.toml"
PROVIDER_PACKAGES = ("eunomia", "eunomia-derive")
REGISTRY_SOURCE = 'source = "registry+https://github.com/rust-lang/crates.io-index"'
CHECKSUM = re.compile(r'^checksum = "[0-9a-f]{64}"$', re.M)


def provider_errors(lockfile: str) -> list[str]:
    """Require one registry identity for each provider crate in Cargo's lock."""
    packages = lockfile.split("[[package]]\n")[1:]
    errors = []
    for name in PROVIDER_PACKAGES:
        entries = [entry for entry in packages if entry.startswith(f'name = "{name}"\n')]
        if len(entries) != 1:
            errors.append(f"expected one {name} package, found {len(entries)}")
            continue
        entry = entries[0]
        if REGISTRY_SOURCE not in entry.splitlines() or CHECKSUM.search(entry) is None:
            errors.append(f"{name} must have a crates.io source and checksum")
    return errors


def check_provider(lockfile: str) -> int:
    errors = provider_errors(lockfile)
    if not errors:
        return 0
    for error in errors:
        print(f"error: {error}", file=sys.stderr)
    print("Regenerate with python3 scripts/lockfile.py --regenerate.", file=sys.stderr)
    return 1


def run_outside_the_overlay(arguments: list[str]) -> subprocess.CompletedProcess[str]:
    """Keep Cargo from loading a parent workspace's local dependency overrides."""
    with tempfile.TemporaryDirectory() as neutral_directory:
        return subprocess.run(
            ["cargo", *arguments, "--manifest-path", str(MANIFEST)],
            cwd=neutral_directory,
            capture_output=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )


def check() -> int:
    if not LOCKFILE.is_file():
        print(f"error: {LOCKFILE} does not exist", file=sys.stderr)
        return 1
    if check_provider(LOCKFILE.read_text(encoding="utf-8")) != 0:
        return 1

    completed = run_outside_the_overlay(
        ["metadata", "--locked", "--format-version", "1", "--all-features"]
    )
    if completed.returncode != 0:
        print(
            "error: Cargo.lock does not resolve under --locked.\n"
            "Regenerate with python3 scripts/lockfile.py --regenerate.\n\n"
            f"{completed.stderr.strip()}",
            file=sys.stderr,
        )
        return 1
    print("Cargo.lock resolves under --locked with one crates.io Eunomia provider.")
    return 0


def check_staged() -> int:
    """Read the staged lock without running Cargo."""
    staged = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--", "Cargo.lock"],
        cwd=REPOSITORY,
        capture_output=True,
        encoding="utf-8",
        check=True,
    )
    if not staged.stdout.strip():
        return 0
    blob = subprocess.run(
        ["git", "show", ":Cargo.lock"],
        cwd=REPOSITORY,
        capture_output=True,
        encoding="utf-8",
        check=False,
    )
    if blob.returncode != 0:
        print("error: the staged change removes Cargo.lock", file=sys.stderr)
        return 1
    return check_provider(blob.stdout)


def regenerate() -> int:
    completed = run_outside_the_overlay(["generate-lockfile"])
    if completed.returncode != 0:
        print(f"error: regeneration failed:\n{completed.stderr.strip()}", file=sys.stderr)
        return 1
    print("Cargo.lock regenerated outside the Atlas overlay.")
    return check()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true", help="verify the committed lock")
    mode.add_argument("--check-staged", action="store_true", help="verify the staged lock")
    mode.add_argument("--regenerate", action="store_true", help="regenerate the lock")
    arguments = parser.parse_args()
    if arguments.regenerate:
        return regenerate()
    if arguments.check_staged:
        return check_staged()
    return check()


if __name__ == "__main__":
    raise SystemExit(main())
