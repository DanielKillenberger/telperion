#!/usr/bin/env python3
"""fn-361: the oak's panel measures, a row per run and age, beside White's trunk range.

    python3 -I measures.py <yield.log> <name>=<label dir>:<stills dir> ...

Medians (lowest to highest) over seeds 1001 to 1012 of height, crown spread
(the two ground axes' mean), trunk at 1.3 m, drawn leaf and drawn phytomers;
each tree's peak memory and seconds from its log's PEAK line. A tree's
seconds count only when no PAUSE of the yield watcher fell between its start
(the log's mtime less its seconds) and its end; otherwise "not timed".
The trunk is counted against White's open-grown Quercus robur ranges, as
fn-352's SCOREBOARD.md quotes them.
"""
import json
import statistics as st
import sys
from datetime import datetime
from pathlib import Path

OAK = "oregon-white-oak"
AGES = [40, 80, 120, 200, 300]
SEEDS = range(1001, 1013)
WHITE = {40: (0.28, 0.40), 80: (0.56, 0.79), 120: (0.83, 1.09), 200: (1.21, 1.52), 300: (1.56, 1.92)}


def pauses(path):
    """Intervals the watcher held heavy jobs paused."""
    out, start = [], None
    for line in Path(path).read_text().splitlines():
        parts = line.split()
        if len(parts) < 3:
            continue
        at = datetime.fromisoformat(f"{parts[0]} {parts[1]}").timestamp()
        if parts[2] == "PAUSE" and start is None:
            start = at
        elif parts[2] == "RESUME" and start is not None:
            out.append((start, at))
            start = None
    if start is not None:
        out.append((start, float("inf")))
    return out


def tree(label, stills, age, seed, held):
    log = stills / f"{OAK}-{age}-{seed}.log"
    t = None
    for line in Path(label, "measures.jsonl").read_text().splitlines():
        r = json.loads(line)
        if r["preset"] == OAK and r["age"] == age and r["seed"] == seed:
            t = r
    if not log.exists():
        return t, None, None
    peak = [l.split() for l in log.read_text().splitlines() if l.startswith("PEAK ")]
    if not peak:
        return t, None, None
    seconds, gib, code = float(peak[-1][2]), float(peak[-1][4]), peak[-1][6]
    end = log.stat().st_mtime
    timed = not any(a < end and b > end - seconds for a, b in held)
    return (t if code == "0" else None), gib, (seconds if timed else None)


def med(xs, f="{:.1f}"):
    xs = [x for x in xs if x is not None]
    if not xs:
        return "-"
    return f"{f.format(st.median(xs))} ({f.format(min(xs))} to {f.format(max(xs))})"


def main():
    held = pauses(sys.argv[1])
    print("| tree | run | drawn | height m | crown spread m | trunk 1.3 m, m | inside White's | leaf m2 | phytomers | peak GiB | seconds |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for age in AGES:
        lo, hi = WHITE[age]
        for arg in sys.argv[2:]:
            name, rest = arg.split("=", 1)
            label, stills = (Path(p) for p in rest.split(":", 1))
            rows = [tree(label, stills, age, s, held) for s in SEEDS]
            drawn = [r[0] for r in rows if r[0]]
            if not any(r[0] or r[1] for r in rows):
                continue
            gib = [r[1] for r in rows]
            secs = [r[2] for r in rows if r[0]]
            untimed = sum(1 for r in rows if r[0] and r[2] is None)
            inside = sum(1 for t in drawn if t.get("dbh") and lo <= t["dbh"] <= hi)
            print(f"| oak {age} | {name} | {len(drawn)}/12 | {med(t['height'] for t in drawn)} "
                  f"| {med((t['spread_x'] + t['spread_z']) / 2 for t in drawn)} "
                  f"| {med((t.get('dbh') for t in drawn), '{:.2f}')} | {inside} of {len(drawn)} in {lo:.2f} to {hi:.2f} "
                  f"| {med((t['leaf_area'] for t in drawn), '{:,.0f}')} | {med((t['phytomers'] for t in drawn), '{:,.0f}')} "
                  f"| {med(gib, '{:.2f}')} | {med(secs, '{:.0f}')}{f'; {untimed} not timed' if untimed else ''} |")


if __name__ == "__main__":
    main()
