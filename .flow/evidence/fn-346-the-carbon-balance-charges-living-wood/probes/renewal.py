"""fn-346 renewal: tables from the renewal census logs (BIRTH, DEATH,
LIVING, ASLEEP, SHEDLIGHT lines; each BIRTH and DEATH line sums the ten
cycles up to its year).

Usage: renewal.py <log> <section> [years...]
Sections: sources, rows, inside, living, sleep, light, leaves.
"""
import re
import sys
from collections import defaultdict

ROW = {2: "trunk", 3: "fork", 4: "leader", 6: "limb", 7: "bough", 8: "sprig",
       9: "branch", 12: "shoot", 13: "twig", 14: "short"}
SRC = {0: "lateral", 1: "woken", 2: "relay", 3: "continuation", 4: "seed"}
CAUSE = {0: "balance", 1: "survival", 2: "end of row", 3: "abortion"}


def parse(path):
    ev = defaultdict(lambda: defaultdict(int))  # (kind, year) -> {(code, pa, band, inside): n}
    living, asleep, light = defaultdict(dict), {}, {}
    for line in open(path):
        w = line.split()
        if not w:
            continue
        kv = dict(re.findall(r"(\w+)=([-\w.]+)", line))
        if w[0] in ("BIRTH", "DEATH"):
            k = (int(kv["code"]), int(kv["pa"]), int(kv["band"]), kv["inside"] == "1")
            ev[(w[0], int(w[1]))][k] += int(kv["n"])
        elif w[0] == "LIVING":
            living[int(w[1])][int(kv["pa"])] = kv
        elif w[0] == "ASLEEP":
            asleep[int(w[1])] = int(kv["held"])
        elif w[0] == "SHEDLIGHT":
            light[int(w[1])] = kv
    return ev, living, asleep, light


def years(ev):
    return sorted({y for (_, y) in ev})


def total(ev, kind, y, f=lambda k: True):
    return sum(n for k, n in ev[(kind, y)].items() if f(k))


def sources(ev, ys):
    print("| decade to | births: lateral | woken | relay | continuation | deaths: balance | survival | end of row | abortion |")
    print("|---|---|---|---|---|---|---|---|---|")
    for y in ys:
        b = [total(ev, "BIRTH", y, lambda k, s=s: k[0] == s) for s in range(4)]
        d = [total(ev, "DEATH", y, lambda k, c=c: k[0] == c) for c in range(4)]
        print(f"| {y} | " + " | ".join(f"{v:,}" for v in b + d) + " |")


def rows(ev, ys):
    """Births over deaths by row: births are every source but continuations
    into the row are counted as births of that row; deaths every cause."""
    print("| decade to | " + " | ".join(f"{ROW[p]} B/D" for p in (6, 7, 9, 12, 13, 14)) + " |")
    print("|---|" + "---|" * 6)
    for y in ys:
        cells = []
        for p in (6, 7, 9, 12, 13, 14):
            b = total(ev, "BIRTH", y, lambda k: k[1] == p)
            d = total(ev, "DEATH", y, lambda k: k[1] == p)
            cells.append(f"{b:,} / {d:,}" + (f" = {b / d:.2f}" if d else ""))
        print(f"| {y} | " + " | ".join(cells) + " |")


def inside(ev, ys):
    print("| decade to | births inside | share | deaths inside | share | balance deaths inside | share of balance deaths |")
    print("|---|---|---|---|---|---|---|")
    for y in ys:
        bi = total(ev, "BIRTH", y, lambda k: k[3])
        bt = total(ev, "BIRTH", y)
        di = total(ev, "DEATH", y, lambda k: k[3])
        dt = total(ev, "DEATH", y)
        si = total(ev, "DEATH", y, lambda k: k[3] and k[0] == 0)
        st = total(ev, "DEATH", y, lambda k: k[0] == 0)
        f = lambda a, b: f"{a / b:.2f}" if b else "-"
        print(f"| {y} | {bi:,} | {f(bi, bt)} | {di:,} | {f(di, dt)} | {si:,} | {f(si, st)} |")


def bands(ev, y):
    by = defaultdict(lambda: [0] * 8)
    for (kind, yy), m in ev.items():
        if yy != y:
            continue
        for (code, pa, band, ins), n in m.items():
            by[band][(0 if kind == "BIRTH" else 4) + code] += n
    print(f"\nYear {y}, by band of path (5 m; -1 = no path yet):\n")
    print("| L m | births lateral | woken | relay | continuation | deaths balance | survival | end of row | abortion |")
    print("|---|---|---|---|---|---|---|---|---|")
    for b in sorted(by):
        print(f"| {5*b} to {5*b+5} | " + " | ".join(f"{v:,}" for v in by[b]) + " |")


def living(lv, ys):
    print("| year | " + " | ".join(ROW[p] for p in (2, 4, 6, 7, 8, 9, 12, 13, 14)) + " |")
    print("|---|" + "---|" * 9)
    for y in ys:
        cells = []
        for p in (2, 4, 6, 7, 8, 9, 12, 13, 14):
            kv = lv[y].get(p)
            cells.append("-" if not kv else f"{int(kv['tips']):,} ({float(kv['ext_prev'])*100:.1f} cm, {float(kv['nodes_prev']):.1f} n)")
        print(f"| {y} | " + " | ".join(cells) + " |")


def sleep(ev, asl, ys):
    print("| decade to | sleeping buds held | woken in the decade | woken by row |")
    print("|---|---|---|---|")
    for y in ys:
        w = {p: total(ev, "BIRTH", y, lambda k, p=p: k[0] == 1 and k[1] == p) for p in ROW}
        ws = ", ".join(f"{ROW[p]} {n}" for p, n in w.items() if n)
        print(f"| {y} | {asl.get(y, '-')} | {sum(w.values()):,} | {ws or '-'} |")


def light(li, ys):
    print("| year | tips shed by the balance | light at their places after: mean | median | living tips: mean | median |")
    print("|---|---|---|---|---|---|")
    for y in ys:
        kv = li.get(y)
        if kv:
            print(f"| {y} | {int(kv['shed']):,} | {kv['shed_mean']} | {kv['shed_median']} | {kv['living_mean']} | {kv['living_median']} |")


def leaves(ev, lv, ys):
    """Leaves a year lost with each tip that died, as its row's mean nodes
    laid the cycle before (one leaf a node, on the current year's wood);
    against the leaves a year born tips of that row would make."""
    print("| decade to | leaf-years lost: balance | survival | end of row | abortion | births' leaf-years | living tips' leaves (census) |")
    print("|---|---|---|---|---|---|---|")
    for y in ys:
        prev = lv.get(y - 10) or lv.get(y)
        nodes = {p: float(kv["nodes_prev"]) for p, kv in (prev or {}).items()}
        lost = [sum(n * nodes.get(k[1], 0.0) for k, n in ev[("DEATH", y)].items() if k[0] == c) for c in range(4)]
        born = sum(n * nodes.get(k[1], 0.0) for k, n in ev[("BIRTH", y)].items() if k[0] != 3)
        now = sum(int(kv["tips"]) * float(kv["nodes_prev"]) for kv in lv[y].values())
        print(f"| {y} | " + " | ".join(f"{v:,.0f}" for v in lost) + f" | {born:,.0f} | {now:,.0f} |")


if __name__ == "__main__":
    log, sec = sys.argv[1], sys.argv[2]
    ev, lv, asl, li = parse(log)
    ys = [int(y) for y in sys.argv[3:]] or years(ev)
    if sec == "sources":
        sources(ev, ys)
    elif sec == "rows":
        rows(ev, ys)
    elif sec == "inside":
        inside(ev, ys)
    elif sec == "bands":
        for y in ys:
            bands(ev, y)
    elif sec == "living":
        living(lv, ys)
    elif sec == "sleep":
        sleep(ev, asl, ys)
    elif sec == "light":
        light(li, ys)
    elif sec == "leaves":
        leaves(ev, lv, ys)
