#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
"""Validate README.md against the workspace README standard.

AGENTS.md section 7.3 requires CI to reject a primary README whose required
headings are missing or out of order, or that still carries unresolved
template tokens. That rule had no enforcement path, so a README could drift
from /Users/seb/Code/README-TEMPLATE.md and nothing would fail.

The heading list and structural signals below mirror the workspace audit
tool (Private/Other/code-portfolio-compliance/tools/audit_compliance.py), so
this gate and the portfolio audit cannot disagree about what conforms.

Exit status: 0 conforming, 1 otherwise. Usage: validate_readme.py [PATH]
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

# Exact set and order of level-2 headings. "{project}" is substituted with the
# centred <h1>, so the ecosystem and limitations headings name the project.
REQUIRED_HEADINGS = [
    "Contents",
    "Install",
    "Requirements",
    "Quick Start",
    "The {project} ecosystem",
    "Capabilities at a glance",
    "Ecosystem comparison",
    "Benchmarks",
    "Features",
    "Configuration",
    "Examples",
    "When not to use {project}",
    "Development",
    "Security",
    "Documentation",
    "Stability guarantees",
    "License",
]

# Structural markers the audit tool also requires verbatim.
# REUSE-IgnoreStart
# The first marker is a literal to search for, not a licence declaration for
# this file; without these markers `reuse lint` parses it as one and reports
# an invalid SPDX expression.
REQUIRED_STRUCTURE = [
    "<!-- SPDX-License-Identifier:",
    '<p align="center">',
    '<h1 align="center">',
    "ossf-scorecard",
]
# REUSE-IgnoreEnd


def validate(path: Path) -> list[str]:
    if not path.is_file():
        return [f"{path} is missing"]
    text = path.read_text(encoding="utf-8")
    issues: list[str] = []

    if "{{" in text or "}}" in text:
        issues.append("unresolved template variables ('{{' or '}}') remain")

    for marker in REQUIRED_STRUCTURE:
        if marker not in text:
            issues.append(f"missing required structure: {marker}")

    match = re.search(r'<h1 align="center">([^<]+)</h1>', text)
    if not match:
        # Without the project name the heading list cannot be resolved.
        issues.append('missing centred project heading: <h1 align="center">')
        return issues

    project = match.group(1).strip()
    required = [heading.format(project=project) for heading in REQUIRED_HEADINGS]
    actual = re.findall(r"(?m)^## ([^\n]+)$", text)

    if actual != required:
        for heading in required:
            if heading not in actual:
                issues.append(f"missing heading: ## {heading}")
        for heading in actual:
            if heading not in required:
                issues.append(f"unexpected heading: ## {heading}")
        if not issues:
            issues.append(
                "required headings are present but out of order; expected: "
                + " -> ".join(required)
            )

    return issues


def main() -> int:
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("README.md")
    issues = validate(path)
    if issues:
        print(f"[readme] {path}: {len(issues)} problem(s)", file=sys.stderr)
        for issue in issues:
            print(f"  - {issue}", file=sys.stderr)
        return 1
    print(f"[readme] {path}: conforms to the workspace template")
    return 0


if __name__ == "__main__":
    sys.exit(main())
