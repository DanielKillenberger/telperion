#!/usr/bin/env python3
"""fn-354: several runs of one panel side by side, a column per run.

    python3 -I variants.py sheets <out dir> <name>=<label dir>:<stills dir> ...
    python3 -I variants.py tables <out dir> <name>=<label dir>:<stills dir> ...

`sheets` writes one JPEG per species and view (rows by age and seed, a
column per run) and the close sheet; `tables` writes VARIANTS.md (every
measure, a column per run) and traces/*.csv, the yearly `RING` lines of
each traced tree's first grow. A tree a run did not draw is a grey tile
saying so; a refused one is a grey tile with its error.
"""
import json
import subprocess
import sys
from pathlib import Path

OAK = "oregon-white-oak"
OTHERS = ["european-beech", "silver-birch", "norway-spruce"]
SHORT = {OAK: "oak", "european-beech": "beech", "silver-birch": "birch", "norway-spruce": "spruce"}
SEEDS = [1001, 1007, 1012]
OAK_AGES = [40, 80, 120, 200, 300]
CLOSE = [(120, "low"), (120, "foot"), (120, "joint"), (300, "low"), (300, "foot"), (300, "joint")]
MEASURES = [
    ("height", "height, m", 1, "{:.1f}"),
    ("spread_x", "spread x, m", 1, "{:.1f}"),
    ("spread_z", "spread z, m", 1, "{:.1f}"),
    ("dbh", "trunk at 1.3 m, cm", 100, "{:.0f}"),
    ("limbs", "base diameter of the three largest limbs, cm", 100, None),
    ("limb_chord", "largest limb's chord from vertical, degrees", 1, "{:.0f}"),
    ("leaf_area", "drawn leaf area, m2", 1, "{:,.0f}"),
    ("leaves", "drawn leaves", 1, "{:,.0f}"),
    ("nodes", "drawn nodes", 1, "{:,.0f}"),
    ("under_finest", "phytomers under 0.2 mm", 1, "{:,.0f}"),
]


def runs(args):
    out = []
    for a in args:
        name, rest = a.split("=", 1)
        label, stills = rest.split(":", 1)
        out.append((name, Path(label), Path(stills)))
    return out


def measures(label):
    by = {}
    path = label / "measures.jsonl"
    if path.exists():
        for line in path.read_text().splitlines():
            t = json.loads(line)
            by.setdefault((t["preset"], float(t["age"]), t["seed"]), t)
    return by


def stem(preset, age):
    return f"{preset}-preset" if preset in OTHERS else f"{preset}-{age:g}"


def refused(stills, preset, age, seed):
    log = stills / f"{stem(preset, age)}-{seed}.log"
    if not log.exists():
        return None
    lines = [l for l in log.read_text().splitlines() if "rror" in l or "refused" in l or "KILLED" in l]
    return lines[-1][:90] if lines else "no TREE line"


def tile(name, t, preset, age, seed, shot):
    head = f"{name}: {SHORT[preset]} {age:g}, seed {seed} ({shot})"
    if t is None:
        return head + "\nnot drawn in this run"
    if "refused" in t:
        return head + "\nREFUSED: " + str(t["refused"])[:90]
    dbh = f"trunk {t['dbh'] * 100:.0f} cm" if t.get("dbh") else "no trunk at 1.3 m"
    return f"{head}\n{t['height']:.1f} m tall, {t['spread_x']:.1f} x {t['spread_z']:.1f} m, {dbh}"


def montage(rows, cols, title, dest):
    subprocess.run(["montage", *rows, "-tile", f"{cols}x", "-geometry", "480x360+4+4",
                    "-font", "Adwaita-Sans", "-pointsize", "13", "-title", title,
                    "-quality", "80", str(dest)], check=True)
    print(dest, f"{dest.stat().st_size // 1024} KiB")


def tiles_for(rs, ms, keys):
    tiles = []
    for preset, age, seed, shot in keys:
        for (name, _, stills), m in zip(rs, ms):
            t = m.get((preset, float(age), seed))
            png = stills / f"{stem(preset, age)}-{seed}-{shot}.png"
            if t is None and (r := refused(stills, preset, age, seed)):
                t = {"refused": r}
            src = str(png) if png.exists() else "xc:gray80[960x720]"
            text = tile(name, t, preset, age, seed, shot)
            if not png.exists() and t is not None and "refused" not in t:
                text = text.split(chr(10))[0] + chr(10) + "this shot not taken in this run"
            tiles += ["-label", text, src]
    return tiles


def sheets(out, rs):
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    ms = [measures(label) for _, label, _ in rs]
    names = ", ".join(n for n, _, _ in rs)
    plan = [(OAK, view, [(OAK, a, s, view) for a in OAK_AGES for s in SEEDS]) for view in ("whole", "bare")]
    for p in OTHERS:
        for view, shot in (("whole", "whole"), ("mid", "crown")):
            plan.append((p, view, [(p, 40, s, shot) for s in SEEDS]))
    for preset, view, keys in plan:
        leaf = "leafless" if view == "bare" else "leafed"
        title = f"{SHORT[preset]}, {view} view, {leaf}; columns {names}"
        montage(tiles_for(rs, ms, keys), len(rs), title, out / f"{SHORT[preset]}-{view}.jpg")
    keys = [(OAK, a, 1001, shot) for a, shot in CLOSE]
    title = f"oak seed 1001, close: trunk base leafed (low) and leafless (foot), largest limb's junction (joint); columns {names}"
    montage(tiles_for(rs, ms, keys), len(rs), title, out / "oak-close.jpg")


def cell(t, key, scale, f):
    if t is None:
        return "-"
    if "refused" in t:
        return "refused"
    v = t.get(key)
    if v is None:
        return "-"
    if f is None:
        return " / ".join(f"{x * scale:.0f}" for x in v)
    return f.format(v * scale)


def traces(label_stills, name, out):
    """Each traced tree's first grow's RING lines, as CSV; the files."""
    made = []
    for log in sorted(label_stills.glob("*.log")):
        lines = [json.loads(l[5:]) for l in log.read_text().splitlines() if l.startswith("RING ")]
        if not lines:
            continue
        first = []
        for r in lines:
            if first and r["year"] <= first[-1]["year"]:
                break
            first.append(r)
        dest = out / "traces" / f"{name}-{log.stem}.csv"
        dest.parent.mkdir(parents=True, exist_ok=True)
        keys = list(first[0])
        dest.write_text(",".join(keys) + "\n" + "\n".join(",".join(str(r[k]) for k in keys) for r in first) + "\n")
        made.append((log.stem, first))
    return made


def tables(out, rs):
    out = Path(out)
    ms = [measures(label) for _, label, _ in rs]
    keys = [(OAK, float(a), s) for a in OAK_AGES for s in SEEDS] + [(p, 40.0, s) for p in OTHERS for s in SEEDS]
    head = "| tree | " + " | ".join(n for n, _, _ in rs) + " |"
    rule = "|---|" + "---|" * len(rs)
    lines = ["# fn-354 probe: the panel, a column per run", ""]
    for key, title, scale, f in MEASURES:
        lines += [f"## {title}", "", head, rule]
        for p, a, s in keys:
            row = [cell(m.get((p, a, s)), key, scale, f) for m in ms]
            lines.append(f"| {SHORT[p]} {a:g} seed {s} | " + " | ".join(row) + " |")
        lines.append("")
    for name, _, stills in rs:
        made = traces(stills, name, out)
        if not made:
            continue
        lines += [f"## Yearly trace, {name}", "",
                  "First grow of each tree; every tenth year and the last. Leaf above 1.3 m and "
                  "the whole tree's leaf in m2, T and S at 1.3 m in cm2, the trunk the rule "
                  "draws there in cm, height in m (the rough layout's, unsagged). Full traces: "
                  "`traces/`.", ""]
        for stem_, rows in made:
            lines += [f"**{stem_}**", "", "| year | leaf whole | leaf above 1.3 m | T cm2 | S cm2 | trunk cm | height m |",
                      "|---|---|---|---|---|---|---|"]
            for r in rows:
                if r["year"] % 10 == 0 or r is rows[-1]:
                    lines.append(f"| {r['year']} | {r['leaf_whole']:.1f} | {r['leaf_13']:.1f} | "
                                 f"{r['t_13'] * 1e4:.0f} | {r['s_13'] * 1e4:.0f} | "
                                 f"{r['dbh_13'] * 100:.1f} | {r['height']:.1f} |")
            lines.append("")
    (out / "VARIANTS.md").write_text("\n".join(lines) + "\n")
    print(out / "VARIANTS.md")


if __name__ == "__main__":
    what = sys.argv[1] if len(sys.argv) > 1 else ""
    if what == "sheets":
        sheets(sys.argv[2], runs(sys.argv[3:]))
    elif what == "tables":
        tables(sys.argv[2], runs(sys.argv[3:]))
    else:
        sys.exit(__doc__)
