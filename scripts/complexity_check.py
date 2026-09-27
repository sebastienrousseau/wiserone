#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
"""Enforce the per-function complexity ceilings (~/Code/AGENTS.md §0).

Every function in the Rust sources is measured with rust-code-analysis-cli
and held to these ceilings:

    cyclomatic <= 10    cognitive <= 15    Halstead difficulty <= 30
    lines of code <= 60 (and <= 500 per file)

A function already over a ceiling is listed in complexity-baseline.json
with its measured values. It may not get worse, and it must be removed
from the baseline once it is brought under, so the list only shrinks.
Any other function over a ceiling fails the check.

Usage:
  python3 scripts/complexity_check.py            # check (CI)
  python3 scripts/complexity_check.py --report   # print every offender
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASELINE = ROOT / "complexity-baseline.json"
PATHS = ["src", "examples", "benches", "tests", "build.rs", "xtask/src", "fuzz/fuzz_targets"]
CEILINGS = {"cyclomatic": 10, "cognitive": 15, "halstead": 30, "sloc": 60}
FILE_SLOC = 500


def measure() -> tuple[dict[str, dict], dict[str, float]]:
    """Per-function metrics keyed "file::name", and per-file line counts."""
    with tempfile.TemporaryDirectory() as out:
        args = ["rust-code-analysis-cli", "-m", "-O", "json", "-o", out]
        for path in PATHS:
            args += ["-p", str(ROOT / path)]
        subprocess.run(args, check=True, capture_output=True)
        functions: dict[str, dict] = {}
        files: dict[str, float] = {}
        for report in Path(out).rglob("*.json"):
            data = json.loads(report.read_text(encoding="utf-8"))
            name = Path(data["name"]).resolve().relative_to(ROOT).as_posix()
            files[name] = data["metrics"]["loc"]["sloc"]
            collect(data, name, functions)
    return functions, files


def collect(node: dict, file: str, out: dict[str, dict]) -> None:
    if node.get("kind") == "function":
        m = node["metrics"]
        metrics = {
            "cyclomatic": m["cyclomatic"]["sum"],
            "cognitive": m["cognitive"]["sum"],
            "halstead": round(m["halstead"].get("difficulty") or 0, 1),
            "sloc": m["loc"]["sloc"],
        }
        # Same-named functions in one file (test helpers, closures) share
        # a key; keep the worst of each metric so none can hide.
        key = f"{file}::{node['name']}"
        seen = out.get(key, metrics)
        out[key] = {k: max(v, seen[k]) for k, v in metrics.items()}
    for child in node.get("spaces", []):
        collect(child, file, out)


def over(metrics: dict) -> bool:
    return any(metrics[k] > limit for k, limit in CEILINGS.items())


def problems(functions: dict, files: dict, baseline: dict) -> list[str]:
    found = [f"{f}: {n:.0f} lines (file ceiling {FILE_SLOC})"
             for f, n in sorted(files.items()) if n > FILE_SLOC]
    for key, metrics in sorted(functions.items()):
        allowed = baseline.get(key)
        if allowed is None and over(metrics):
            found.append(f"{key}: {metrics} is over {CEILINGS}")
        elif allowed is not None:
            worse = [k for k in CEILINGS if metrics[k] > allowed[k]]
            if worse:
                found.append(f"{key}: worse than its baseline on {', '.join(worse)}: {metrics}")
    for key in sorted(set(baseline) - {k for k, m in functions.items() if over(m)}):
        found.append(f"{key}: now under every ceiling or gone; remove it from {BASELINE.name}")
    return found


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--report", action="store_true", help="print every offender")
    args = parser.parse_args(argv)
    functions, files = measure()
    if args.report:
        for key, metrics in sorted(functions.items()):
            if over(metrics):
                print(f"{key}: {metrics}")
        return 0
    baseline = json.loads(BASELINE.read_text(encoding="utf-8"))
    found = problems(functions, files, baseline)
    if found:
        print(f"[complexity] {len(found)} problem(s):", file=sys.stderr)
        for line in found:
            print(f"  - {line}", file=sys.stderr)
        return 1
    print(f"[complexity] {len(functions)} functions within the ceilings "
          f"({len(baseline)} baselined)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
