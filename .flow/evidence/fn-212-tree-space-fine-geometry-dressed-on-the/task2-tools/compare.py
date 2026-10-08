#!/usr/bin/env python3
"""fn-212 task 2: each state's digests against the one before it; two
dumps of leaves compared word by word; the timings tabulated.
   compare.py digests | leaves <a.bin> <b.bin> | times"""
import json, pathlib, re, statistics, sys
from array import array

RAW = pathlib.Path(__file__).resolve().parent.parent
STATES = ["s0", "s1", "s2", "s3", "s4"]
DRAWN = [("pipeline", "tree"), ("pipeline", "wood"), ("pipeline", "leaves"), ("pipeline", "plan"),
         ("pipeline", "structure"), ("mesh",), ("gpuCpu", "instances"), ("gpuCpu", "count"),
         ("gpuCpu", "bounds"), ("gpuCpu", "curve"), ("gpuResident", "instances"),
         ("gpuResident", "count"), ("gpuResident", "bounds"), ("gpuResident", "curve"),
         ("gpuResident", "backend")]


def get(d, path):
    for k in path:
        d = d.get(k) if isinstance(d, dict) else None
    return d


def lines(st, age):
    p = RAW / f"digest-{st}-{age}.jsonl"
    out = {}
    if p.exists():
        for l in p.read_text().splitlines():
            if l.startswith("{"):
                j = json.loads(l)
                out[(j["preset"], j["seed"])] = j
    return out


def digests():
    for age in ("40", "80"):
        prev, pst = None, None
        for st in STATES:
            cur = lines(st, age)
            if prev and cur:
                for key in sorted(set(prev) | set(cur)):
                    a, b = prev.get(key), cur.get(key)
                    if a is None or b is None:
                        print(age, st, key, "missing")
                        continue
                    # A field one side did not record (the CPU at 80) is not compared.
                    moved = [".".join(p) for p in DRAWN
                             if get(a, p) is not None and get(b, p) is not None and get(a, p) != get(b, p)]
                    compared = sum(get(a, p) is not None and get(b, p) is not None for p in DRAWN)
                    print(f"{age} {pst}->{st} {key[0]} s{key[1]}: "
                          f"{'same' if not moved else 'moved ' + ','.join(moved)} ({compared} fields)")
            if cur:
                prev, pst = cur, st


def leaves(a, b):
    x = array("I"); x.frombytes((RAW / a).read_bytes())
    y = array("I"); y.frombytes((RAW / b).read_bytes())
    print(a, b, "leaves", len(x) // 3, len(y) // 3)
    if len(x) != len(y):
        return
    differing = rot = scale = 0
    worst = [0, 0, 0]
    over = [0, 0, 0]
    for i in range(0, len(x), 3):
        if x[i] == y[i] and x[i + 1] == y[i + 1] and x[i + 2] == y[i + 2]:
            continue
        differing += 1
        rot += x[i] != y[i]
        scale += (x[i + 2] >> 16) != (y[i + 2] >> 16)
        codes = [(x[i + 1] & 0xffff, y[i + 1] & 0xffff), (x[i + 1] >> 16, y[i + 1] >> 16),
                 (x[i + 2] & 0xffff, y[i + 2] & 0xffff)]
        for k, (p, q) in enumerate(codes):
            d = abs(p - q)
            worst[k] = max(worst[k], d)
            over[k] += d > 1
    print(f" differing {differing}; rotation words {rot}; scale {scale}")
    print(f" position code steps, most (x, y, z): {worst}; leaves past one step: {over}")


def times():
    rows = {}
    for p in sorted(RAW.glob("time-s*-r*.jsonl")):
        st = p.name.split("-")[1]
        for l in p.read_text().splitlines():
            if not l.startswith("{"):
                continue
            # The probe prints Rust's Option debug form inside quotes.
            l = re.sub(r'"(Some\("[^"]*"\)|None)"', lambda m: json.dumps(m.group(1)), l)
            j = json.loads(l)
            key = (f"{j['preset']} {j['age']} s{j['seed']}", st)
            rows.setdefault(key, []).append(j)
    print("| tree | state | runs | load | cold ms | warm ms (median) | placement wait | positions prep | peak RSS MB | GPU positions | fallback |")
    print("|---|---|--:|--:|--:|--:|--:|--:|--:|---|---|")
    for (tree, st), js in sorted(rows.items()):
        cold = [j["grown_to_resident_ms"] for j in js if j["build"] == 0]
        warm = [j["grown_to_resident_ms"] for j in js if j["build"] > 0]
        print(f"| {tree} | {st} | {len(js)} | {statistics.median(float(j['load']) for j in js):.0f} | "
              f"{statistics.median(cold):,.0f} | {statistics.median(warm):,.0f} | "
              f"{statistics.median(j['placement_wait_ms'] for j in js):,.0f} | "
              f"{statistics.median(j['position_prepare_ms'] for j in js):,.0f} | "
              f"{max(j['peak_rss_mb'] for j in js):,} | {js[-1]['gpu_positions']} | {js[-1]['wood_fallback']} |")


if __name__ == "__main__":
    what = sys.argv[1] if len(sys.argv) > 1 else "digests"
    {"digests": digests, "times": times}.get(what, lambda: leaves(sys.argv[2], sys.argv[3]))()
