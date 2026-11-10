"""fn-344 follow-up: B's yearly census every 10 years beside A's.

Usage: python3 -I probes/btable.py <A log> <B log>
Columns per side: held phytomers, living stem-line limbs, lowest living
limb (m, rough layout), height (m, rough layout), resident GiB; then B's
held over A's running peak and B's held growth a year over the decade.
"""
import re, sys


def read(path):
    rows = {}
    for line in open(path):
        m = re.match(r"CENSUS (\d+) (\d+) rss_gib=([\d.]+)", line)
        if m:
            rows.setdefault(int(m[1]), {})["rss"] = float(m[3])
        m = re.match(r"YEAR (\d+) held=(\d+) height=(\S+) stem_top=\S+ apexes=\d+ limbs_live=(\d+) lowest_limb=(\S+)", line)
        if m:
            rows.setdefault(int(m[1]), {}).update(
                held=int(m[2]), h=float(m[3]), limbs=int(m[4]), low=float(m[5]))
    return rows


a, b = read(sys.argv[1]), read(sys.argv[2])
peak, top = {}, 0
for y in sorted(a):
    top = max(top, a[y].get("held", 0))
    peak[y] = top
last = max(y for y in b if "held" in b[y])
years = sorted({y for y in range(9, last + 1, 10)} | {last})
print("| year | A held | A limbs | A lowest m | A height m | A GiB | B held | B limbs | B lowest m | B height m | B GiB | B / A's peak | B held a year |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
prev = None
for y in years:
    ra, rb = a.get(y, {}), b.get(y, {})
    rate = f"{(rb['held'] - b[prev]['held']) / (y - prev):+,.0f}" if prev is not None else ""
    cells = [str(y)]
    for r in (ra, rb):
        cells += [f"{r['held']:,}" if "held" in r else "-", str(r.get("limbs", "-")),
                  f"{r['low']:.1f}" if "low" in r else "-", f"{r['h']:.1f}" if "h" in r else "-",
                  f"{r['rss']:.1f}" if "rss" in r else "-"]
    cells += [f"{rb['held'] / peak[y]:.2f}" if y in peak and "held" in rb else "-", rate]
    print("| " + " | ".join(cells) + " |")
    prev = y
