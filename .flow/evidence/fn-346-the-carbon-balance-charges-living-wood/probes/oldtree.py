"""fn-346 old-tree tables from probes/oldtree.sh logs.

Usage: oldtree.py <raw/oldtree dir> <seed> <section>
Sections: ends (every limb and bough line that ended from 119 on),
endsum (counts by cause and row), shape, reach, losses.
"""
import re
import sys
from collections import Counter, defaultdict

d, seed, sec = sys.argv[1], sys.argv[2], sys.argv[3]


def kv(line):
    return dict(re.findall(r"(\w+)=([-\w.]+)", line))


def hist():
    return [l for l in open(f"{d}/history-{seed}.log")]


if sec in ("ends", "endsum"):
    rows = []
    for l in hist():
        if l.startswith("LINEEND"):
            y = int(l.split()[1])
            r = kv(l)
            r["year"] = y
            rows.append(r)
    if sec == "endsum":
        c = Counter((r["pa"], r["cause"]) for r in rows if r["year"] >= 119)
        print("| row | cause | lines ended 119 to 299 |")
        print("|---|---|---|")
        for (pa, cause), n in sorted(c.items()):
            print(f"| {'limb' if pa == '6' else 'bough'} | {cause} | {n} |")
        lim = [r for r in rows if r["year"] >= 119 and r["pa"] == "6"]
        part = sum(1 for r in lim if int(r["kids_paying"]) > 0)
        print(f"\nLimbs ended 119 to 299: {len(lim)}; with at least one own bough above zero the year before: {part}.")
        bo = [r for r in rows if r["year"] >= 119 and r["pa"] == "7"]
        part = sum(1 for r in bo if int(r["kids_paying"]) > 0)
        print(f"Boughs ended 119 to 299: {len(bo)}; with at least one own branch above zero: {part}.")
    else:
        print("| year | row | cause | born | on stem | base path m | base height m | memory | light | charge | length m | tip path m | living tips | own boughs/branches | of them paying |")
        print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
        for r in rows:
            if r["year"] < 119 or r["pa"] != "6":
                continue
            print(f"| {r['year']} | limb | {r['cause']} | {r['birth']} | {r['main_bearer']} | {float(r['base_path']):.1f} | {float(r['base_z']):.1f} | "
                  f"{float(r['memory']):+.3f} | {float(r['light']):,.0f} | {float(r['charge']):,.0f} | {float(r['length']):.1f} | "
                  f"{float(r['tip_path']):.1f} | {int(r['tips']):,} | {r['kids']} | {r['kids_paying']} |")
elif sec == "shape":
    print("| age | limb born | base m | base radius cm | length m | reach m | tip height against base m | lowest point of the line m | lowest of its subtree m | phytomers below ground | extension m/yr | unsagged: tip height against base m | unsagged lowest of the line m |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for age in (120, 160, 200, 240, 280):
        try:
            sag = [kv(l) for l in open(f"{d}/shape-{age}-{seed}.log") if l.startswith("SHAPE")]
            flat = {r["lineage"]: r for r in (kv(l) for l in open(f"{d}/shape-{age}-{seed}-unsagged.log") if l.startswith("SHAPE"))}
        except FileNotFoundError:
            continue
        for r in sorted(sag, key=lambda r: float(r["base_z"])):
            f = flat.get(r["lineage"], {})
            print(f"| {age} | {r['birth']} | {float(r['base_z']):.1f} | {r['base_radius_cm']} | {float(r['length']):.1f} | {float(r['reach']):.1f} | "
                  f"{float(r['tip_dz']):+.1f} | {float(r['line_low']):.1f} | {float(r['sub_low']):.1f} | {r['below_ground_phytomers']} | {r['ext_per_year']} | "
                  f"{float(f['tip_dz']):+.1f} | {float(f['line_low']):.1f} |" if f else "")
elif sec == "reach":
    bal = {int(l.split()[1]): kv(l) for l in hist() if l.startswith("BAL ")}
    yr = {int(l.split()[1]): kv(l) for l in hist() if l.startswith("YEAR ")}
    print("| year | longest path m | height m | spread m | living tips | share of tips by height band (5 m) |")
    print("|---|---|---|---|---|---|")
    for l in hist():
        if l.startswith("REACH"):
            y = int(l.split()[1])
            r = kv(l)
            b = bal.get(y, {})
            raw = re.search(r"bands=(\S+)", l).group(1)
            bands = ", ".join(f"{int(k)}–{int(k) + 5} m {float(v) * 100:.0f}%" for k, v in (x.split(":") for x in raw.split(",")) if float(v) >= 0.01)
            print(f"| {y} | {r['longest_path']} | {float(yr[y]['height']):.1f} | {float(b.get('spread_x', 'nan')):.1f} × {float(b.get('spread_y', 'nan')):.1f} | {int(r['tips']):,} | {bands} |")
elif sec == "losses":
    print("| limb lost in | base path m | on the stem | framework born within 3 m of its base on its bearer in the 20 years after: limbs | boughs | branches |")
    print("|---|---|---|---|---|---|")
    for l in hist():
        if l.startswith("LOSS"):
            r = kv(l)
            print(f"| {r['lost']} | {r['base_path']} | {r['bearer_stem']} | {r['births20_limb']} | {r['bough']} | {r['branch']} |")
    sl = defaultdict(int)
    for l in hist():
        if l.startswith("LINEEND") and " pa=6 " in l:
            sl[int(l.split()[1])] = int(kv(l)["stem_sleepers"])
    print("\nSleeping buds held on the stem line in the year a limb was lost: " + ", ".join(f"{y}: {n}" for y, n in sorted(sl.items()) if y >= 119))
