"""A blend scanned at a fine step: 200 steps of t (0.005 each), run as ten
spans of the dump tool in parallel, written as one JSONL. Smooth when no
fine step moves height or spread by more than 3% or nodes or leaves by more
than 10%: a steep stretch passes, a jump does not.
Usage: finescan.py <out.jsonl> <from> <to> <seed> [overlay.json]"""
import json
import os
import subprocess
import sys

DUMP = "raw/dump-target/release/fn167-dump"
LIMITS = {"height": 0.03, "spread": 0.03, "nodes": 0.10, "leaves": 0.10}

def scan(out: str, args: list[str]) -> list[dict]:
    spans = [(i / 10, (i + 1) / 10) for i in range(10)]
    procs = [subprocess.Popen([DUMP, "blend", *args], stdout=subprocess.PIPE,
                              env={**os.environ, "BLEND_SPAN": f"{a},{b}"})
             for a, b in spans]
    rows: dict[float, dict] = {}
    for p in procs:
        text, _ = p.communicate()
        if p.returncode:
            sys.exit(f"dump failed for {args}")
        for line in text.decode().splitlines():
            row = json.loads(line)
            rows[round(row["t"], 6)] = row
    ordered = [rows[t] for t in sorted(rows)]
    with open(out, "w") as f:
        f.writelines(json.dumps(r) + "\n" for r in ordered)
    return ordered

def judge(rows: list[dict]) -> bool:
    smooth = True
    for name, limit in LIMITS.items():
        worst, at = 0.0, 0.0
        for a, b in zip(rows, rows[1:]):
            x, y = a["metrics"][name], b["metrics"][name]
            change = abs(y - x) / max(abs(x), abs(y), 1e-9)
            if change > worst:
                worst, at = change, b["t"]
        ok = worst <= limit
        smooth &= ok
        print(f"  {name:7s} largest fine step {worst:6.1%} at t={at:.3f} "
              f"(limit {limit:.0%}) {'ok' if ok else 'JUMP'}")
    print(f"  verdict: {'smooth' if smooth else 'not smooth'}")
    return smooth

if __name__ == "__main__":
    out, *args = sys.argv[1:]
    print(out)
    sys.exit(0 if judge(scan(out, args)) else 1)
