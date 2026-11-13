#!/usr/bin/env python3
"""fn-361: the oak's sheets, a column per run, height, trunk and leaf under each tile.

    python3 -I sheet.py <out.jpg> <view> <ages> <name>=<label dir>:<stills dir> ...

<view> is a shot (whole, bare); <ages> is comma-separated. Rows are each age's
seeds 7, 1001, 1007 and 1012. Built on fn-352's variants.py (its montage and
its refusal read); a tree a run did not draw is a grey tile saying so.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "fn-352-the-scoreboard-a-fixed-panel-of-trees" / "scoreboard"))
import variants as v  # noqa: E402

SEEDS = [7, 1001, 1007, 1012]


def label(name, t, age, seed, shot):
    head = f"{name}: oak {age}, seed {seed} ({shot})"
    if t is None:
        return head + "\nnot drawn in this run"
    if "refused" in t:
        return head + "\nREFUSED: " + str(t["refused"])[:90]
    dbh = f"trunk {t['dbh'] * 100:.0f} cm" if t.get("dbh") else "no trunk at 1.3 m"
    return f"{head}\n{t['height']:.1f} m tall, {dbh}, leaf {t['leaf_area']:,.0f} m2"


def main():
    out, shot, ages, *args = sys.argv[1:]
    rs = v.runs(args)
    ms = [v.measures(lab) for _, lab, _ in rs]
    tiles = []
    for age in (int(a) for a in ages.split(",")):
        for seed in SEEDS:
            for (name, _, stills), m in zip(rs, ms):
                t = m.get((v.OAK, float(age), seed))
                png = stills / f"{v.OAK}-{age}-{seed}-{shot}.png"
                if t is None and (r := v.refused(stills, v.OAK, age, seed)):
                    t = {"refused": r}
                text = label(name, t, age, seed, shot)
                if not png.exists() and t is not None and "refused" not in t:
                    text = text.split("\n")[0] + "\nthis shot not taken in this run"
                tiles += ["-label", text, str(png) if png.exists() else "xc:gray80[960x720]"]
    names = ", ".join(n for n, _, _ in rs)
    leaf = "leafless" if shot == "bare" else "leafed"
    v.montage(tiles, len(rs), f"oak {ages}, {shot} view, {leaf}; columns {names}", Path(out))


if __name__ == "__main__":
    main()
