"""fn-335 R3: one preset's read, reduced to cell sums as its lines arrive.

Usage: python3 -I read.py <test-binary> <preset> <cpu> <out.json>
Grows the preset at its own age, full cycle, through `telperion-heavy`, one
thread pinned to <cpu>, by `lateral_records`, its stdout read as a pipe so no
record line is written to disk (PROTOCOL.md, answer 10). Pass 1 grows the
fit seeds 1001 to 1006 and keeps only each lateral age's lights, for the five
equal-count light bins. Pass 2 grows seeds 1001 to 1012 and sums each lateral
into its cells. Writes the cell sums, each fit cell's percentiles and each
test tree's sums to <out.json>.
"""
import bisect, json, os, subprocess, sys
from array import array
from collections import defaultdict

FIT, TEST = list(range(1001, 1007)), list(range(1007, 1013))
# Spans 1 to 8 each its own bin, then 9-16, then 17 and over: ten bins.
SPAN_EDGES = [1, 2, 3, 4, 5, 6, 7, 8, 16]

binary, preset, cpu, out = sys.argv[1:5]


def run(seeds):
    env = dict(os.environ, RAYON_NUM_THREADS="1", LATERAL_PRESET=preset,
               LATERAL_SEEDS=",".join(map(str, seeds)))
    cmd = ["telperion-heavy", "4", "--threads", "1", "--", "taskset", "-c", cpu,
           binary, "lateral_records", "--ignored", "--nocapture", "--test-threads=1"]
    p = subprocess.Popen(cmd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                         text=True, bufsize=1 << 20)
    seed, tail = None, []
    for raw in p.stdout:
        k = raw.find("FINE ")
        if k >= 0:
            r = raw[k:].split()
            seed = int(r[2])
            yield "FINE", seed, r
            continue
        if raw.startswith("REC "):
            yield "REC", seed, [float(x) for x in raw.split()[1:]]
        else:
            tail = (tail + [raw])[-20:]
    rc = p.wait()
    if rc != 0:
        raise SystemExit(f"{preset}: exit {rc}\n" + "".join(tail))


def span_bin(span):
    return bisect.bisect_left(SPAN_EDGES, span)


def light_of(r):
    return r[2] if r[2] >= 0 else r[3]


# Pass 1: the fit seeds' lights by lateral age (span 0 left out).
lights = defaultdict(lambda: array("f"))
for kind, seed, r in run(FIT):
    if kind == "REC" and r[1] > 0:
        lights[int(r[0])].append(light_of(r))
cuts, ranges = {}, {}
for age, v in lights.items():
    s = sorted(v)
    cuts[age] = [s[len(s) * k // 5] for k in range(1, 5)]
    ranges[age] = [s[0], s[-1], len(s)]

# Pass 2: every seed, summed into cells.
# Per lateral: grown leaf share, standing leaf, section, wood, and the three
# standing shares of the potential's grown value.
NV = 7
fit = defaultdict(lambda: [0] + [0.0] * NV)       # (age, sb, lb, order, bearer)
dist = defaultdict(lambda: [array("f") for _ in range(NV)])  # (age, sb, lb)
test = defaultdict(lambda: [0] + [0.0] * 6)       # (seed, age, sb, lb, order, bearer)
counts = defaultdict(lambda: defaultdict(int))
fine, ages = {}, {}
for kind, seed, r in run(FIT + TEST):
    if kind == "FINE":
        ages[seed] = int(r[4])
        fine[seed] = [int(c.split(":")[0]) for c in r[5:] if float(c.split(":")[2]) > 1.0]
        continue
    age, span = int(r[0]), int(r[1])
    c = counts[age]
    c["laterals"] += 1
    if span == 0:
        c["span0"] += 1
        continue
    if r[2] < 0:
        c["grew_nothing"] += 1
    pot = r[18:22]
    if pot[0] < 0 or min(pot[1:4]) <= 0:
        c["no_potential"] += 1
        continue
    sleaf, ssec, swood = r[14], r[15], r[16]
    vals = [r[9] / pot[1], sleaf, ssec, swood, sleaf / pot[1], ssec / pot[2], swood / pot[3]]
    lb = bisect.bisect_right(cuts.get(age, []), light_of(r))
    key = (age, span_bin(span), lb, int(r[4]), int(r[5]))
    if seed in FIT:
        cell = fit[key]
        cell[0] += 1
        for k, v in enumerate(vals):
            cell[1 + k] += v
        d = dist[key[:3]]
        for k, v in enumerate(vals):
            d[k].append(v)
    else:
        cell = test[(seed,) + key]
        cell[0] += 1
        for k, v in enumerate([sleaf, ssec, swood, pot[1], pot[2], pot[3]]):
            cell[1 + k] += v


def pct(v, q):
    s = sorted(v)
    return s[min(len(s) - 1, int(q * (len(s) - 1) + 0.5))]


summary = {}
for key, d in dist.items():
    summary[",".join(map(str, key))] = {
        "n": len(d[0]),
        "mean": [sum(x) / len(x) for x in d],
        "p10": [pct(x, 0.1) for x in d],
        "p50": [pct(x, 0.5) for x in d],
        "p90": [pct(x, 0.9) for x in d],
    }
json.dump({
    "preset": preset, "age": ages, "fine": fine, "cuts": cuts, "light_range": ranges,
    "counts": {a: dict(c) for a, c in counts.items()},
    "values": ["grown leaf share", "standing leaf", "standing section", "standing wood",
               "standing leaf share", "standing section share", "standing wood share"],
    "fit": {",".join(map(str, k)): v for k, v in fit.items()},
    "test": {",".join(map(str, k)): v for k, v in test.items()},
    "cells": summary,
}, open(out, "w"))
print(preset, "done", flush=True)
