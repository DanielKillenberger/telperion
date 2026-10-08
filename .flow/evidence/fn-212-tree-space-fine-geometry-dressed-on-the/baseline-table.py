#!/usr/bin/env python3
"""fn-212 task 1: the baseline's stage table from baseline-run.py's output.

Groups the probe's marks into the stages BASELINE in AUDIT.md names, per
build; prints cold (build 0) and the better warm (builds 1, 2) by total.
   baseline-table.py <dir> [<dir> ...]
"""
import json
import pathlib
import re
import sys

STAGES = [
    ("convert", ["convert", "drop structure"]),
    ("envelope+inputs", ["envelope", "inputs", "skeleton tail"]),
    ("clothe+element+box", ["clothe", "element", "reference", "expansion rest"]),
    ("rings (CPU)", ["rings", "rings with contacts"]),
    ("stations (CPU)", ["stations", "compact stations", "late stations or fallback"]),
    ("ring positions (GPU)", ["positions upload", "positions wait", "positions emit"]),
    ("placement (GPU)", ["placement"]),
    ("curve (CPU)", ["curve", "drop rings"]),
    ("assemble", ["assemble"]),
]


def fix(line):
    """The probe prints Rust's Option debug form; read it as JSON."""
    return re.sub(r'Some\(("[^"]*")\)', r"\1", line).replace(":None", ":null")


def row(b):
    marks = {}
    for name, ms, _ in b["marks"]:
        marks[name] = marks.get(name, 0.0) + ms
    known = {n for _, names in STAGES for n in names}
    unknown = set(marks) - known
    assert not unknown, unknown
    cells = [sum(marks.get(n, 0.0) for n in names) for _, names in STAGES]
    cells += [b["submit_ms"], b["drain_ms"], b["settled_to_resident_ms"]]
    return cells


heads = [s for s, _ in STAGES] + ["submit (upload)", "drain", "total"]
print("| tree | build | load | " + " | ".join(heads) + " | peak RSS MB | leaves |")
print("|---|---|--:|" + "--:|" * len(heads) + "--:|--:|")
for d in sys.argv[1:]:
    for f in sorted(pathlib.Path(d).glob("*.jsonl")):
        builds = [json.loads(fix(l)) for l in f.read_text().splitlines() if l.startswith("{")]
        if not builds:
            continue
        warm = min(builds[1:], key=lambda b: b["settled_to_resident_ms"]) if len(builds) > 1 else None
        for label, b in [("cold", builds[0]), ("warm", warm)]:
            if b is None:
                continue
            cells = " | ".join(f"{v:,.0f}" for v in row(b))
            print(f"| {b['preset']} {b['age']} s{b['seed']} | {label} | {b['load']} | {cells} | "
                  f"{b['peak_rss_mb_since_settled']:,} | {b['leaves']:,} |")
