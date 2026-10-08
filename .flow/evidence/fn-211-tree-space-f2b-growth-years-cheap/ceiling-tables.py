#!/usr/bin/env python3
"""fn-211.7: option A's ceiling from the probe's raw lines.
usage: ceiling-tables.py <run file>...  (the warm repeat, rep 1, where present)

Movable (A puts it on the device): the items' share of balance and
allotment, their growth and waking, their sketch, the fine leaves'
deposits, the fine buds' reads, and the re-lay's passes at the fine
roots' share of the re-laid phytomers.
Stays (CPU): the coarse grower's balance, allotment, growth and sketch,
the handover, the lattice's box and density, the sweep, the coarse
reads, the re-lay's coarse share, and the settle.
Glue (the stepped executor's own, gone under A): leaf and tip gathering,
writing light back, the re-lay's assembly and dispersal, and the
residual of the years outside the timed phases.
The per-phase hash (`mark`) is excluded throughout, as in HYBRID.md."""
import json, sys

T = ["balance", "allot", "coarse", "items_grow", "items_sketch", "handover", "relay", "shade", "mark"]
W = ["bal_items", "bal_coarse", "allot_items", "allot_coarse", "glue", "span", "dep_coarse",
     "dep_fine", "sweep", "read_coarse", "read_fine", "assemble", "relay_tree", "disperse",
     "r_settle", "r_place", "r_levers", "r_sag", "r_write"]
C = ["leaves_coarse", "leaves_fine", "tips_coarse", "tips_fine", "nodes", "relay_fine", "relay_all"]


def load(path):
    reps, cur, end, loads = {}, None, {}, []
    for line in open(path):
        if line.startswith("REP "):
            cur = int(line.split()[1]); reps[cur] = []
        elif line.startswith("HYB "):
            reps[cur].append(json.loads(line[4:]))
        elif line.startswith("END "):
            e = json.loads(line[4:]); end[e["rep"]] = e
        elif line.startswith("LOAD"):
            loads.append(line.split()[1])
    rep = max(k for k in reps if k in end)
    return reps[rep], end[rep], loads, rep


def split(path):
    rows, end, loads, rep = load(path)
    t = {k: sum(r["t"][i] for r in rows) for i, k in enumerate(T)}
    w = {k: sum(r["w"][i] for r in rows) for i, k in enumerate(W)}
    c = {k: sum(r["c"][i] for r in rows) for i, k in enumerate(C)}
    share = c["relay_fine"] / c["relay_all"]
    build = end["build"] - t["mark"]
    years = end["years"] - t["mark"]
    timed = sum(v for k, v in t.items() if k != "mark")
    m = {
        "balance, items": w["bal_items"],
        "allotment, items": w["allot_items"],
        "growth and waking, items": t["items_grow"],
        "sketch, items": t["items_sketch"],
        "deposits, fine leaves": w["dep_fine"],
        "reads, fine buds": w["read_fine"],
        "re-lay, fine share": w["relay_tree"] * share,
    }
    s = {
        "balance, coarse": w["bal_coarse"],
        "allotment, coarse": w["allot_coarse"],
        "growth and sketch, coarse": t["coarse"],
        "handover": t["handover"],
        "lattice box and density": w["span"],
        "deposits, coarse leaves": w["dep_coarse"],
        "sweep": w["sweep"],
        "reads, coarse buds": w["read_coarse"],
        "re-lay, coarse share": w["relay_tree"] * (1 - share),
        "settle (with assembly)": end["settle"],
    }
    g = {
        "gather and write-back": w["glue"],
        "re-lay assembly and dispersal": w["assemble"] + w["disperse"],
        "years outside the phases": years - timed,
        "start (before the first year)": end["build"] - end["years"] - end["settle"],
    }
    return dict(path=path, rep=rep, loads=loads, build=build, m=m, s=s, g=g, w=w, c=c,
                share=share, t=t, end=end)


def pct(x, b):
    return f"{x:.3f} s ({100 * x / b:.1f} %)"


for path in sys.argv[1:]:
    d = split(path)
    b = d["build"]
    M, S, G = (sum(d[k].values()) for k in "msg")
    print(f"## {path} (rep {d['rep']}; load {' -> '.join(d['loads'])})")
    print(f"build without marks {b:.3f} s; marks {d['t']['mark']:.3f}; roots {d['end']['roots']}")
    print(f"re-lay: fine share of re-laid phytomers {d['share']:.4f}; passes s: " + ", ".join(
        f"{k} {d['w'][k]:.3f}" for k in ["r_settle", "r_place", "r_levers", "r_sag", "r_write"]))
    c = d["c"]
    print(f"leaves coarse/fine {c['leaves_coarse']:,}/{c['leaves_fine']:,}; tips {c['tips_coarse']:,}/{c['tips_fine']:,}")
    for name, part in (("MOVABLE", d["m"]), ("STAYS", d["s"]), ("GLUE", d["g"])):
        tot = sum(part.values())
        print(f"{name} {pct(tot, b)}")
        for k, v in part.items():
            print(f"    {k}: {pct(v, b)}")
    print(f"check: movable+stays+glue {M + S + G:.3f} against build {b:.3f}")
    low = M - d["m"]["re-lay, fine share"]
    print(f"SHARE movable {100 * M / b:.1f} %; without the re-lay {100 * low / b:.1f} %; "
          f"with the glue counted movable {100 * (M + G) / b:.1f} %")
    print()
