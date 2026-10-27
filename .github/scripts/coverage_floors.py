#!/usr/bin/env python3
"""Coverage floors per layer (craftbook rust/08-testing-and-quality.md §8.1).

Reads the JSON of `cargo llvm-cov --workspace --all-features --json
--summary-only` and the layer of each crate from craftbook.toml. A crate at
or above its layer's floor must stay there. A crate below it is reported,
not enforced against the layer floor; instead it may not fall below its own
measured RATCHET (only ever raised). The interface layer has no floor: it is
reported.

Usage: coverage_floors.py COVERAGE_JSON [CRAFTBOOK_TOML]
"""

import json
import re
import sys
import tomllib

LAYER_FLOOR = {"shared-kernel": 95.0, "domain": 90.0, "app": 85.0, "infra": 70.0}

# Crates below their layer's floor, with the measured line coverage they may
# not fall below (measured 2026-10 with the command above; only raised).
RATCHET = {
    "substrate-message": 92.0,
    "substrate-git-domain": 87.0,
    "substrate-dap-app": 81.0,
    "substrate-git-app": 43.0,
    "substrate-project-app": 50.0,
    "substrate-git-gitcli": 56.0,
    "substrate-process": 31.0,
}


def line_coverage(report):
    """Covered and total lines per crate, tests and benches excluded."""
    totals = {}
    for file in report["data"][0]["files"]:
        found = re.search(r"/(?:crates/)?([^/]+)/(src/.*)", file["filename"])
        if not found:
            continue
        lines = file["summary"]["lines"]
        covered, count = totals.get(found.group(1), (0, 0))
        totals[found.group(1)] = (covered + lines["covered"], count + lines["count"])
    return totals


def main():
    report = json.load(open(sys.argv[1]))
    config = tomllib.load(open(sys.argv[2] if len(sys.argv) > 2 else "craftbook.toml", "rb"))
    layers = {c["name"]: c["layer"] for c in config["crate"]}
    layers["tauri-plugin"] = layers["tauri-plugin-substrate"]  # the directory's name
    failed = False
    for name, (covered, count) in sorted(line_coverage(report).items()):
        percent = 100.0 * covered / max(count, 1)
        layer = layers.get(name, "?")
        floor = LAYER_FLOOR.get(layer)
        ratchet = RATCHET.get(name)
        if floor is None:
            verdict = "reported"
        elif percent >= floor:
            verdict = "ok"
        elif ratchet is not None and percent >= ratchet:
            verdict = f"below {floor:.0f} % (reported; ratchet {ratchet:.0f} %)"
        else:
            verdict = f"FAIL: below {floor:.0f} % and its ratchet"
            failed = True
        print(f"{name:32s} {layer:14s} {percent:5.1f} %  {verdict}")
    sys.exit(1 if failed else 0)


main()
