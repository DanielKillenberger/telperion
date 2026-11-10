"""fn-341 R1: reduce the `structure_measures` logs (probes/structure.patch)
to the medians R1-MEASURES.md quotes, per preset and age over the seeds.

Run: python3 -I r1.py <log> [<log> ...]
"""
import statistics
import sys

KEYS = ["height", "spread", "dbh", "dband", "d03", "fork_z", "d_fork", "taper_cm_m",
        "lowest_live", "lowest_live5", "bole_live", "bole_live5", "bole_live10", "limbs_pa", "secs"]


def kv(words):
    out = {}
    for w in words:
        if "=" in w:
            k, v = w.split("=", 1)
            try:
                out[k] = float(v)
            except ValueError:
                out[k] = v
    return out


def med(xs):
    xs = [x for x in xs if x == x]
    return statistics.median(xs) if xs else float("nan")


def main():
    trees, lats = {}, {}
    for path in sys.argv[1:]:
        for line in open(path):
            w = line.split()
            if not w or w[0] not in ("TREE", "LAT"):
                continue
            key = (w[1], int(w[3]))
            d = kv(w[4:])
            d["seed"] = int(w[2])
            (trees if w[0] == "TREE" else lats).setdefault(key, []).append(d)
    for key in sorted(trees):
        ts = trees[key]
        print(f"## {key[0]} at {key[1]}, seeds {[t['seed'] for t in ts]}")
        for k in KEYS:
            vals = [t[k] for t in ts]
            print(f"- {k}: median {med(vals):.3f}; per seed {', '.join(f'{v:.3f}' for v in vals)}")
        for t in ts:
            print(f"- seed {t['seed']}: child_q {t['child_q']}, area_q {t['area_q']}, junctions {t['junctions']:.0f}")
        dbh = [t["dbh"] for t in ts]
        sl = [t["height"] / t["dbh"] for t in ts]
        cs = [t["spread"] / t["dbh"] for t in ts]
        print(f"- height over dbh: median {med(sl):.1f}; spread over dbh: median {med(cs):.1f}; "
              f"dbh cm per year: {med(dbh) * 100 / key[1]:.2f}")
        ls = lats.get(key, [])
        print(f"- laterals of 5 cm or more off the trunk path: {len(ls)} over {len(ts)} seeds")
        for lo, hi, name in [(0, 1 / 3, "lower third"), (1 / 3, 2 / 3, "middle third"), (2 / 3, 1.01, "upper third")]:
            sel = [x for x in ls if lo <= x["rel"] < hi]
            if sel:
                print(f"  - {name}: n {len(sel)}, from vertical median {med([x['vert'] for x in sel]):.1f} deg, "
                      f"from the trunk median {med([x['from_trunk'] for x in sel]):.1f} deg, "
                      f"d over parent median {med([x['ratio'] for x in sel]):.2f}")
        by_pa = {}
        for x in ls:
            by_pa.setdefault(int(x["pa"]), []).append(x)
        for pa, sel in sorted(by_pa.items()):
            print(f"  - pa {pa}: n {len(sel)}, from vertical median {med([x['vert'] for x in sel]):.1f}, "
                  f"height median {med([x['h'] for x in sel]):.2f} m, d median {med([x['d'] for x in sel]):.3f} m, "
                  f"d over parent {med([x['ratio'] for x in sel]):.2f}, length median {med([x['length'] for x in sel]):.1f} m, "
                  f"tip out {med([x['out'] for x in sel]):.1f} m and up {med([x['up'] for x in sel]):.1f} m, "
                  f"live {sum(int(x['live']) for x in sel)}")
        print()


if __name__ == "__main__":
    main()
