#!/usr/bin/env python3
"""fn-363 R6: one sheet per species, a column per run, height, trunk and leaf
under each tile; a tree a run refused is a grey tile with what it reached.

    python3 -I sheet.py <out.jpg> <preset> <ages> <shots> <name>=<label dir>:<stills dir> ...

<ages> is comma-separated (`preset` for the preset's own age); <shots> is
comma-separated (whole, bare). Rows are each age's seeds 1001 and 1007, each
shot. Built on fn-352's variants.py (its measures, refusal read and montage).
"""
import sys
from pathlib import Path

E = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(E / "fn-352-the-scoreboard-a-fixed-panel-of-trees" / "scoreboard"))
import variants as v  # noqa: E402

SEEDS = [1001, 1007]


def text(name, preset, t, age, seed, shot):
    head = f"{name}: {preset} {age}, seed {seed} ({shot})"
    if t is None:
        return head + "\nnot drawn in this run"
    if "refused" in t:
        return head + "\nREFUSED: " + str(t["refused"])[:90]
    dbh = f"trunk {t['dbh'] * 100:.0f} cm" if t.get("dbh") else "no trunk at 1.3 m"
    return f"{head}\n{t['height']:.1f} m tall, {dbh}, leaf {t['leaf_area']:,.0f} m2"


def main():
    out, preset, ages, shots, *args = sys.argv[1:]
    rs = v.runs(args)
    ms = [v.measures(label) for _, label, _ in rs]
    tiles = []
    for age in ages.split(","):
        stem = f"{preset}-{age}"
        for seed in SEEDS:
            for shot in shots.split(","):
                for (name, _, stills), m in zip(rs, ms):
                    key = next((k for k in m if k[0] == preset and k[2] == seed
                                and (age == "preset" or k[1] == float(age))), None)
                    t = m.get(key) if key else None
                    png = stills / f"{stem}-{seed}-{shot}.png"
                    log = stills / f"{stem}-{seed}.log"
                    if t is None and log.exists():
                        bad = [l for l in log.read_text().splitlines()
                               if "rror" in l or "refused" in l or "KILLED" in l or "oom" in l]
                        t = {"refused": bad[-1] if bad else "no TREE line"}
                    label = text(name, preset, t, age, seed, shot)
                    tiles += ["-label", label, str(png) if png.exists() else "xc:gray80[960x720]"]
    names = ", ".join(n for n, _, _ in rs)
    v.montage(tiles, len(rs), f"{preset} {ages}, {shots}; columns {names}", Path(out))


if __name__ == "__main__":
    main()
