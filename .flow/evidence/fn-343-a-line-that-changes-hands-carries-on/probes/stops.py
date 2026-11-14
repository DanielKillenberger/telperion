"""fn-343 R1: the take-over joints (fork and lateral_at_end, turn >= 10 deg)
of one tagged walk log, with the bearer's stop as the grower tagged it when
its apex_end was set. Shares by stop cause, by the bearer's PA, by the wood's
radius, and where the lateral sits below the bearer's tip.

Run: python3 -I stops.py <log> [<log> ...]
"""
import sys
from collections import Counter, defaultdict

PA = {6: "limb", 7: "bough", 8: "sprig", 9: "branch", 12: "shoot", 13: "twig", 14: "short"}
RB = ((0.0, 0.01), (0.01, 0.025), (0.025, 0.05), (0.05, 9))


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


def cause(d):
    s = d.get("stop", "none")
    if d["apex_end"] < 0:
        return "living"
    return s.split("@")[0]


def table(title, rows, key, keys=None):
    n = len(rows)
    c = defaultdict(Counter)
    for d in rows:
        c[key(d)][cause(d)] += 1
    causes = sorted({k for v in c.values() for k in v}, key=lambda k: -sum(v[k] for v in c.values()))
    keys = keys or sorted(c, key=lambda k: -sum(c[k].values()))
    print(f"\n### {title} ({n} joints)\n")
    print("| | n | " + " | ".join(causes) + " |")
    print("|---|---|" + "---|" * len(causes))
    for k in keys:
        t = sum(c[k].values())
        if t:
            print(f"| {k} | {t} | " + " | ".join(f"{c[k][x] / t:.2f}" for x in causes) + " |")
    t = Counter()
    for v in c.values():
        t.update(v)
    print("| all | {} | ".format(n) + " | ".join(f"{t[x] / n:.2f}" for x in causes) + " |")


for path in sys.argv[1:]:
    rows, age = [], None
    for line in open(path):
        w = line.split()
        if w and w[0] == "TREE":
            age = int(kv(w[2:])["age"])
        elif w and w[0] == "JOINT" and ("cause=fork" in line or "cause=lateral_at_end" in line):
            d = kv(w[4:])
            if d["turn"] >= 10:
                d["seed"] = int(w[1])
                rows.append(d)
    print(f"## oak at {age}, seeds 1001 and 1007: take-over joints >= 10 deg")
    print(f"\nfork {sum(d['cause'] == 'fork' for d in rows)}, lateral_at_end "
          f"{sum(d['cause'] == 'lateral_at_end' for d in rows)}, "
          f"bearer apex stopped {sum(d['apex_end'] >= 0 for d in rows)}, "
          f"stops untagged {sum(d['apex_end'] >= 0 and d.get('stop') == 'none' for d in rows)}, "
          f"bearers with >1 stop record {sum(d.get('stop_recs', 0) > 1 for d in rows)}")
    table("by the bearer's rank", rows, lambda d: PA.get(int(d["bearer_pa"]), int(d["bearer_pa"])))
    rb = lambda d: next(f"{a * 100:g}-{b * 100:g} cm" for a, b in RB if a <= d["bearer_r"] < b)
    table("by the bearer's radius at the joint", rows, rb, [f"{a * 100:g}-{b * 100:g} cm" for a, b in RB])
    thick = [d for d in rows if d["bearer_r"] >= 0.01]
    table("by the bearer's rank, bearer 1 cm or more at the joint", thick,
          lambda d: PA.get(int(d["bearer_pa"]), int(d["bearer_pa"])))
    # Years from the lateral's birth to the bearer's stop, and the take-over
    # pull the lay gave the lateral's first phytomer.
    for lab, sel in (("all", rows), ("bearer >= 1 cm", thick)):
        yrs = sorted(d["apex_end"] - d["lat_birth"] for d in sel if d["apex_end"] >= 0)
        q = lambda f: yrs[int(f * (len(yrs) - 1))]
        print(f"\n- {lab}: years from the lateral's birth to the bearer's stop, quartiles "
              f"{q(0.25):.0f}, {q(0.5):.0f}, {q(0.75):.0f} (n {len(yrs)}); stop within 1 year of its birth "
              f"{sum(y <= 1 for y in yrs) / len(yrs):.2f}; lateral's first phytomer with take-over pull "
              f"(over > 0) {sum(d['over'] > 0 for d in sel)}, lift > 0 {sum(d['lift'] > 0 for d in sel)}; "
              f"turn median {sorted(d['turn'] for d in sel)[len(sel) // 2]:.1f} deg, "
              f"heading vs bearer median {sorted(d['frame'] for d in sel)[len(sel) // 2]:.1f} deg")
    # The lateral taken: its rank, where it stands below the tip.
    print("\n### the lateral taken\n")
    lat = Counter(PA.get(int(d["lat_pa"]), int(d["lat_pa"])) for d in rows)
    print("lateral PA:", ", ".join(f"{k} {v / len(rows):.2f}" for k, v in lat.most_common()))
    pair = Counter((PA.get(int(d["bearer_pa"])), PA.get(int(d["lat_pa"]))) for d in rows)
    print("bearer -> lateral:", ", ".join(f"{a}->{b} {v}" for (a, b), v in pair.most_common(8)))
    for lab, sel in (("all", rows), ("bearer >= 1 cm", thick),
                     ("bearer >= 1 cm, moved_on", [d for d in thick if cause(d) == "moved_on"]),
                     ("bearer >= 1 cm, failed or aborted", [d for d in thick if cause(d).startswith(("fail", "abort"))]),
                     ("bearer >= 1 cm, living", [d for d in thick if cause(d) == "living"])):
        if not sel:
            continue
        s = sorted(sel, key=lambda d: d["left_m"])
        q = lambda f: s[int(f * (len(s) - 1))]["left_m"]
        nl = sorted(d["left"] for d in sel)
        sub = sum(d["above"] == 0 for d in sel) / len(sel)
        same = sum(d["lat_birth"] == d["node_cycle"] for d in sel) / len(sel)
        later = sum(d["lat_birth"] > d["node_cycle"] + 1 for d in sel) / len(sel)
        ratio = sorted(d["alt"] for d in sel)
        print(f"- {lab} ({len(sel)}): wood left past the lateral median {q(0.5):.3f} m "
              f"(quartiles {q(0.25):.3f}, {q(0.75):.3f}), median {nl[len(nl) // 2]:.0f} phytomers; "
              f"no lateral with wood above it {sub:.2f}; lateral born the cycle its node was laid {same:.2f}, "
              f"woke 2+ cycles later {later:.2f}; bearer's own wood past it over the lateral's, "
              f"median {ratio[len(ratio) // 2]:.2f}")
    succ = Counter()
    for d in rows:
        s = d["succ"].strip("[]")
        if not s:
            succ["none"] += 1
            continue
        for e in s.split(","):
            kind, pa, r, n = e.split(":")
            succ[f"{kind} {PA.get(int(pa), pa)} {'no wood' if int(n) == 0 else 'wood'}"] += 1
    print("bearer's own successors:", ", ".join(f"{k} {v}" for k, v in succ.most_common()))
    print()
