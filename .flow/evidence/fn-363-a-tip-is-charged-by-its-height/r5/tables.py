"""fn-363 R5: the before/after table from the runner logs and the clock.

    python3 -I tables.py   (prints Markdown)
"""
import re
from pathlib import Path

D = Path(__file__).resolve().parent
YIELD = D.parent / "yield.log"
POTENTIAL = {"oregon-white-oak": 34.8, "norway-spruce": 61.0, "european-beech": 40.0,
             "silver-birch": 30.0, "laurelin": 40.0, "telperion": 40.0}
WHITE = {40: (0.28, 0.40), 80: (0.56, 0.79), 120: (0.83, 1.09), 200: (1.21, 1.52), 300: (1.56, 1.92)}


def stamp(s):
    return s.strip()


def pauses():
    out, start = [], None
    for line in YIELD.read_text().splitlines() if YIELD.exists() else []:
        t = line[:19]
        if "PAUSE" in line:
            start = t
        elif "RESUME" in line and start:
            out.append((start, t))
            start = None
    if start:
        out.append((start, "9999"))
    return out


def windows():
    w, starts = {}, {}
    for line in (D / "times.txt").read_text().splitlines():
        m = re.match(r"(\S+) start (.+)$", line)
        if m:
            starts[m.group(1)] = m.group(2)
        m = re.match(r"(\S+) end rc=(\d+) (.+)$", line)
        if m:
            w[m.group(1)] = (starts.get(m.group(1)), m.group(3), int(m.group(2)))
    return w


def read(name):
    log, out = D / "logs" / f"{name}.log", D / "logs" / f"{name}.out"
    if not log.exists():
        return None
    s = log.read_text() + (out.read_text() if out.exists() else "")
    r = {}

    def num(pat, key, f=float):
        m = re.search(pat, s)
        r[key] = f(m.group(1)) if m else None
    num(r"OO\|crown\|top tip height above ground ([-\d.]+)", "h")
    num(r"SP\|T1\|spread_x ([\d.]+)", "sx")
    num(r"SP\|T1\|spread_x [\d.]+\|spread_y ([\d.]+)", "sy")
    num(r"\|r13 (\S+?)\|", "r13")
    num(r"OO\|A\|axes \d+\|phytomers (\d+)", "ph", int)
    num(r"OO\|A\|.*\|leafed (\d+)", "leafed", int)
    num(r"leaf_m2 ([\d.]+)", "leaf")
    num(r"tallest_living_tip ([-\d.]+)", "tall")
    num(r"PEAK_MIB (\d+)", "peak", int)
    num(r"WALL_S (\d+)", "wall", int)
    num(r"EXIT (-?\d+)", "exit", int)
    r["sheds"] = sum(sum(int(x) for x in m.split(",")) for m in re.findall(r"SP\|T3\|\d+\|([\d,]+)", s))
    r["killed"] = "oom-kill" in s or "Killed" in s or r["exit"] in (-9, 137)
    return r


def secs(name, r, w, ps):
    if r["wall"] is None or name not in w or w[name][0] is None:
        return "-"
    a, b, _ = w[name]
    if any(p0 <= b and p1 >= a for p0, p1 in ps):
        return "not timed"
    return str(r["wall"])


def fmt(v, f):
    return "-" if v is None or v != v else f.format(v)


def row(preset, age, seed, side, w, ps):
    name = f"{side}-{preset}-a{age}-s{seed}"
    r = read(name)
    if r is None:
        return f"| {preset} | {age} | {seed} | {side} | not run |" + " |" * 10
    if r["exit"] != 0 or r["ph"] is None:
        why = "not grown in 24 GB (killed)" if r["killed"] else ("in progress when written" if r["exit"] is None else f"failed, exit {r['exit']}")
        return f"| {preset} | {age} | {seed} | {side} | {why} |" + " |" * 9 + f" {fmt(r['peak'] and r['peak'] / 1024, '{:.1f}')} | - |"
    dbh = 2 * r["r13"] if r["r13"] == r["r13"] and r["r13"] is not None else None
    white = ""
    if preset == "oregon-white-oak" and int(age) in WHITE and dbh is not None:
        lo, hi = WHITE[int(age)]
        white = " (in)" if lo <= dbh <= hi else f" (out of {lo} to {hi})"
    pot = POTENTIAL.get(preset)
    tall = fmt(r["tall"], "{:.1f}") + (f" / {pot}" if pot else "")
    return (f"| {preset} | {age} | {seed} | {side} | {fmt(r['h'], '{:.1f}')} | "
            f"{fmt((r['sx'] + r['sy']) / 2 if r['sx'] is not None else None, '{:.1f}')} | "
            f"{fmt(dbh, '{:.2f}')}{white} | {fmt(r['leaf'], '{:,.0f}')} | {r['leafed']:,} | {r['ph']:,} | "
            f"{tall} | {r['sheds']:,} | {r['peak'] / 1024:.1f} | {secs(name, r, w, ps)} |")


def main():
    w, ps = windows(), pauses()
    print("| preset | age | seed | run | height m | mean crown spread m | trunk at 1.3 m, m | settled leaf m² "
          "| leafed phytomers | phytomers | tallest living tip m / potential | apexes shed by the balance, last 20 cycles "
          "| peak GiB | seconds |")
    print("|" + "---|" * 14)
    trees = []
    for p in ["ordinary", "oregon-white-oak", "norway-spruce", "european-beech", "silver-birch",
              "date-palm", "laurelin", "telperion"]:
        for a in (["40", "80", "82"] if p == "telperion" else ["40", "80"]):
            for s in ["1", "7"]:
                trees.append((p, a, s))
    for a in ["120", "200", "300"]:
        for s in ["1001", "1007", "1012"]:
            trees.append(("oregon-white-oak", a, s))
    for t in trees:
        for side in ["before", "after"]:
            print(row(*t, side, w, ps))


if __name__ == "__main__":
    main()
