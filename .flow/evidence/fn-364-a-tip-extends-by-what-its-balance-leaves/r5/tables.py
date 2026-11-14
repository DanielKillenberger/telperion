"""fn-364 R5: fn-363's trees (its builder's logs) against this branch's.

    python3 -I tables.py [out.md]
"""
import re
from pathlib import Path

D = Path(__file__).resolve().parent
BEFORE = D.parent / "fn363"
YIELD = D.parent / "yield.log"
POTENTIAL = {"oregon-white-oak": 34.8, "norway-spruce": 61.0, "european-beech": 40.0,
             "silver-birch": 30.0, "laurelin": 40.0, "telperion": 40.0}
WHITE = {40: (0.28, 0.40), 80: (0.56, 0.79), 120: (0.83, 1.09), 200: (1.21, 1.52), 300: (1.56, 1.92)}


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


def windows(folder):
    w, starts = {}, {}
    f = folder / "times.txt"
    for line in f.read_text().splitlines() if f.exists() else []:
        m = re.match(r"(\S+) start (.+)$", line)
        if m:
            starts[m.group(1)] = m.group(2)
        m = re.match(r"(\S+) end rc=(\d+) (.+)$", line)
        if m:
            w[m.group(1)] = (starts.get(m.group(1)), m.group(3))
    return w


def read(folder, name):
    log, out = folder / "logs" / f"{name}.log", folder / "logs" / f"{name}.out"
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
    num(r"LENGTHS zero (\d+)", "zero", int)
    num(r"SP\|S0\|(\d+)\|", "s0", int)
    num(r"SP\|S0\|\d+\|(\d+)", "live", int)
    num(r"LEAFCHECK .* stations (\d+)", "stations", int)
    r["sheds"] = sum(sum(int(x) for x in m.split(",")) for m in re.findall(r"SP\|T3\|\d+\|([\d,]+)", s))
    return r


def secs(name, r, w, ps):
    if r["wall"] is None or name not in w or w[name][0] is None:
        return "-"
    a, b = w[name]
    return "not timed" if any(p0 <= b and p1 >= a for p0, p1 in ps) else str(r["wall"])


def fmt(v, f):
    return "-" if v is None or v != v else f.format(v)


def row(preset, age, seed, side, ps):
    folder, prefix = {"fn-363": (BEFORE, "after"), "this": (D, "this"), "this, R8": (D, "r8")}[side]
    name = f"{prefix}-{preset}-a{age}-s{seed}"
    r = read(folder, name)
    if r is not None:
        leaf = {"fn-363": "leaf363", "this": "leaf364", "this, R8": "leafr8"}[side]
        drawn = read(D, f"{leaf}-{preset}-a{age}-s{seed}")
        r["stations"] = drawn and drawn["stations"]
    head = f"| {preset} | {age} | {seed} | {side} |"
    if r is None or r["exit"] != 0 or r["ph"] is None:
        why = "not run" if r is None else ("in progress" if r["exit"] is None else f"failed, exit {r['exit']}")
        return head + f" {why} |" + " |" * 15
    dbh = 2 * r["r13"] if r["r13"] is not None and r["r13"] == r["r13"] else None
    white = ""
    if preset == "oregon-white-oak" and int(age) in WHITE and dbh is not None:
        lo, hi = WHITE[int(age)]
        white = " (in)" if lo <= dbh <= hi else f" (out of {lo} to {hi})"
    pot = POTENTIAL.get(preset)
    if side == "this, R8" and preset in ("laurelin", "telperion"):
        pot = 104.0
    tall = fmt(r["tall"], "{:.1f}") + (f" / {pot}" if pot else "")
    leaf = r["leaf"]
    per = leaf / r["leafed"] * 1e4 if leaf and r["leafed"] else None
    dens = r["ph"] / leaf if leaf else None
    s0 = "-" if r["s0"] is None else f"{r['s0']:,} of {r['live']:,}"
    zero = "-" if r["zero"] is None else f"{r['zero']:,}"
    drawn = "-" if r.get("stations") is None else f"{r['stations']:,}"
    spread = (r["sx"] + r["sy"]) / 2 if r["sx"] is not None else None
    return (head + f" {fmt(r['h'], '{:.1f}')} | {fmt(spread, '{:.1f}')} | {fmt(dbh, '{:.2f}')}{white} | "
            f"{fmt(leaf, '{:,.0f}')} | {drawn} | {r['leafed']:,} | {r['ph']:,} | {fmt(per, '{:.1f}')} | {fmt(dens, '{:,.0f}')} | "
            f"{tall} | {r['sheds']:,} | {s0} | {zero} | {r['peak'] / 1024:.1f} | "
            f"{secs(name, r, windows(folder), ps)} |")


def main():
    ps = pauses()
    print("| preset | age | seed | run | height m | mean crown spread m | trunk at 1.3 m, m | settled leaf m² | drawn leaf stations "
          "| leafed phytomers | phytomers | leaf per leafed phytomer, cm² | phytomers per m² of leaf "
          "| tallest living tip m / potential | apexes shed by the balance, last 20 cycles "
          "| living tips at s = 0 | zero-length phytomers | peak GiB | seconds |")
    print("|" + "---|" * 19)
    trees = [("oregon-white-oak", a, s) for a in ["120", "200", "300"] for s in ["1001", "1007", "1012"]]
    for p in ["oregon-white-oak", "ordinary", "norway-spruce", "european-beech", "silver-birch",
              "date-palm", "laurelin", "telperion"]:
        for a in (["40", "80", "82"] if p == "telperion" else ["40", "80"]):
            for s in ["1", "7"]:
                trees.append((p, a, s))
    for t in trees:
        two = t[0] in ("laurelin", "telperion") and t[1] in ("40", "80", "82")
        for side in ["fn-363", "this"] + (["this, R8"] if two else []):
            print(row(*t, side, ps))


if __name__ == "__main__":
    import contextlib, io, sys
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        main()
    if len(sys.argv) > 1:
        Path(sys.argv[1]).write_text(buf.getvalue())
    print(buf.getvalue(), end="")
