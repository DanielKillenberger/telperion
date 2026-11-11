#!/usr/bin/env python3
"""fn-352: the scoreboard's sheets, tables and comparison.

    python3 -I board.py sheets  <label dir> <raw stills dir>
    python3 -I board.py tables  <label dir>
    python3 -I board.py compare <label dir A> <label dir B>

The label dir holds `measures.jsonl` (one TREE line per tree, `run.sh`),
`cost.jsonl`, `many.json` and `wasm.jsonl`. Sheets need ImageMagick
(`montage`). Published ranges are those fn-339 (R2-REAL-TREES.md) and fn-341
(R1-MEASURES.md) checked against their sources; each carries its source.
"""
import json
import statistics
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from ranges import PUBLISHED  # noqa: E402

NAMES = {
    "oregon-white-oak": "Oregon white oak", "norway-spruce": "Norway spruce",
    "european-beech": "European beech", "silver-birch": "Silver birch",
    "date-palm": "Date palm", "ordinary": "The ordinary tree",
    "laurelin": "Laurelin", "telperion": "Telperion",
}
SHORT = {"oregon-white-oak": "oak", "norway-spruce": "spruce", "european-beech": "beech",
         "silver-birch": "birch", "date-palm": "palm", "ordinary": "ordinary",
         "laurelin": "laurelin", "telperion": "telperion"}
MEASURE_SEEDS = range(1001, 1013)
# (key, heading, how to print); crown ratio is derived below.
MEASURES = [
    ("height", "height, m", "{:.1f}"),
    ("spread_x", "spread x, m", "{:.1f}"),
    ("spread_z", "spread z, m", "{:.1f}"),
    ("dbh", "trunk at 1.3 m, m", "{:.2f}"),
    ("lowest_live", "lowest living lateral, m", "{:.1f}"),
    ("lowest_live5", "lowest living limb (5 cm+), m", "{:.1f}"),
    ("crown_ratio", "crown ratio (1 - lateral / height)", "{:.2f}"),
    ("leaf_area", "leaf area, m2", "{:,.0f}"),
    ("lai", "leaf area index", "{:.1f}"),
    ("new_shoots", "new shoots, last year", "{:,.0f}"),
    ("nodes", "drawn nodes", "{:,.0f}"),
]
VIEWS = {"whole": ["whole"], "mid": ["crown"], "close": ["spray", "low"]}


def trees(label):
    out = []
    for line in (Path(label) / "measures.jsonl").read_text().splitlines():
        t = json.loads(line)
        if "height" in t and t.get("lowest_live") is not None:
            t["crown_ratio"] = 1 - t["lowest_live"] / t["height"]
        out.append(t)
    return out


def panel(ts):
    """(preset, age) in the panel's order, the oak's preset age once."""
    order = list(NAMES)
    keys = sorted({(t["preset"], t["age"]) for t in ts},
                  key=lambda k: (order.index(k[0]), k[1]))
    return keys


def groups(ts):
    """Each (preset, age)'s trees, one per seed: the oak at its preset age is
    drawn twice, as the preset and as 40, and is the same tree."""
    g, seen = {}, set()
    for t in ts:
        key = (t["preset"], t["age"], t["seed"])
        if t["seed"] in MEASURE_SEEDS and key not in seen:
            seen.add(key)
            g.setdefault(key[:2], []).append(t)
    return g


def summary(group, key):
    """Median and range over the drawn trees that have the measure."""
    v = [t[key] for t in group if t.get(key) is not None]
    if not v:
        return None
    return statistics.median(v), min(v), max(v), len(v)


# ---- sheets -----------------------------------------------------------------

def label_of(t, age, seed):
    head = f"{SHORT[t['preset']] if t else ''} age {age:g} seed {seed}"
    if t is None:
        return head + "\nnot drawn"
    if "refused" in t:
        return head + "\nREFUSED: " + t["refused"][:90]
    dbh = f"trunk {t['dbh'] * 100:.0f} cm" if t.get("dbh") else "no trunk at 1.3 m"
    return f"{head}\n{t['height']:.1f} m tall, {t['spread_x']:.1f} x {t['spread_z']:.1f} m, {dbh}"


def sheets(label, stills):
    label, stills = Path(label), Path(stills)
    out = label / "sheets"
    out.mkdir(exist_ok=True)
    ts = trees(label)
    by = {(t["preset"], t["age"], t["seed"]): t for t in ts}
    for preset in NAMES:
        keys = [k for k in panel(ts) if k[0] == preset]
        if not keys:
            continue
        # The oak's sheet holds its preset age and its five ages: the preset
        # rows run under the age `preset`, the others under the number.
        seeds = [7, 1001, 1007, 1012] if preset == "oregon-white-oak" else [1001, 1007, 1012]
        rows = rows_of(preset, keys, stills)
        views = dict(VIEWS)
        if preset == "oregon-white-oak":
            views["bare"] = ["bare"]
        for view, shots in views.items():
            tiles = []
            for age, stem in rows:
                for shot in shots:
                    for seed in seeds:
                        t = by.get((preset, age, seed))
                        png = stills / f"{stem}-{seed}-{shot}.png"
                        if not png.exists():
                            if t is None and not (stills / f"{stem}-{seed}.log").exists():
                                continue  # a seed this row never draws
                            png = "xc:gray80[960x720]"
                            if t is None:
                                t = refusal(stills / f"{stem}-{seed}.log", preset)
                        tiles += ["-label", f"{label_of(t, age, seed)} ({shot})", str(png)]
            if not tiles:
                continue
            leaf = "leafless" if view == "bare" else "leafed"
            title = f"{NAMES[preset]} ({preset}), {view} view, {leaf}; run {label.name}"
            dest = out / f"{SHORT[preset]}-{view}.jpg"
            subprocess.run(["montage", *tiles, "-tile", f"{len(seeds)}x",
                            "-geometry", "480x360+4+4", "-pointsize", "13",
                            "-title", title, "-quality", "80", str(dest)], check=True)
            print(dest, f"{dest.stat().st_size // 1024} KiB")


def rows_of(preset, keys, stills):
    """Each age with the file stem its stills carry, an age drawn twice once."""
    rows, seen = [], set()
    for _, age in keys:
        if age in seen:
            continue
        seen.add(age)
        stem = f"{preset}-{age:g}"
        if not list(stills.glob(f"{stem}-*.log")):
            stem = f"{preset}-preset"
        rows.append((age, stem))
    return rows


def refusal(log, preset):
    if not log.exists():
        return None
    lines = [l for l in log.read_text().splitlines() if "rror" in l or "KILLED" in l]
    return {"preset": preset, "refused": lines[-1] if lines else "no TREE line"}


# ---- tables -----------------------------------------------------------------

def fmt(fmt_, v):
    return fmt_.format(v)


def tables(label):
    label = Path(label)
    ts = trees(label)
    g = groups(ts)
    lines = [f"# Scoreboard tables, run `{label.name}`", "",
             "Median and range (lowest to highest) over the seeds drawn of 1001 to 1012; "
             "`n` is how many had the measure. Published ranges beside, each with its source.", ""]
    for key in panel(ts):
        group = g.get(key, [])
        preset, age = key
        drawn = [t for t in group if "height" in t]
        refused = [t for t in group if "refused" in t]
        lines += [f"## {NAMES[preset]} at {age:g}", "",
                  f"Drawn {len(drawn)} of {len(group)}" +
                  (f"; refused: {refused[0]['refused']}" if refused else "") + ".", "",
                  "| measure | median | range | n | published |", "|---|---|---|---|---|"]
        for m, head, f in MEASURES:
            s = summary(drawn, m)
            pub = "; ".join(PUBLISHED.get((preset, m, age), PUBLISHED.get((preset, m, None), [])))
            if s is None:
                lines.append(f"| {head} | - | - | 0 | {pub} |")
            else:
                med, lo, hi, n = s
                lines.append(f"| {head} | {fmt(f, med)} | {fmt(f, lo)} to {fmt(f, hi)} | {n} | {pub} |")
        lines.append("")
    lines += cost_tables(label)
    (label / "TABLES.md").write_text("\n".join(lines) + "\n")
    print(label / "TABLES.md")


def cost_tables(label):
    out = []
    cost = label / "cost.jsonl"
    if cost.exists():
        out += ["## Cost, native, one thread", "",
                "Seed 1001; grown, expanded and dressed in one process pinned to one core "
                "(`taskset -c 3`, RAYON_NUM_THREADS=1). `seconds` is the probe's own clock "
                "around the grow and dress; `process` adds start-up; `peak` is the process's "
                "peak resident memory.", "",
                "| tree | seconds | process s | peak GiB | nodes | leaves |", "|---|---|---|---|---|---|"]
        for line in cost.read_text().splitlines():
            c = json.loads(line)
            name = f"{SHORT[c['preset']]} at {c['age']:g}"
            if "refused" in c:
                out.append(f"| {name} | refused: {c['refused']} | | | | |")
            else:
                out.append(f"| {name} | {c['seconds']:.2f} | {c['process_seconds']:.1f} | "
                           f"{c['rss_gib']:.2f} | {c['nodes']:,} | {c['leaves']:,} |")
        out.append("")
    many = label / "many.json"
    if many.exists() and many.read_text().strip():
        m = json.loads(many.read_text())
        out += [f"A thousand distinct oaks: {m['count']} oaks at {m['age']}, seeds {m['first']} "
                f"to {m['first'] + m['count'] - 1}, across {m['threads']} threads: "
                f"**{m['seconds']:.1f} s** ({m['refused']} refused), peak {m['rss_gib']:.1f} GiB.", ""]
    wasm = label / "wasm.jsonl"
    if wasm.exists():
        out += wasm_table(wasm, "## Cost, Wasm (fn-329's protocol)")
    return out


def wasm_table(path, title):
    rows = {}
    for line in path.read_text().splitlines():
        if not line.startswith("{"):
            continue
        w = json.loads(line)
        rows.setdefault((w["id"], w["fidelity"]), []).append(w)
    out = [title, "",
           "Median warm build (five, after one cold) per seed, `build(family, { field: { n: 36 } })`, "
           "Node, one thread pinned; seeds 1, 1234, 5678.", "",
           "| preset | fidelity | age | ms per seed | median ms |", "|---|---|---|---|---|"]
    for (pid, fid), ws in rows.items():
        ok = [w for w in ws if "ms" in w]
        if not ok:
            out.append(f"| {pid} | {fid} | | refused: {ws[0].get('refused', '')[:80]} | |")
            continue
        ms = [w["ms"] for w in ok]
        out.append(f"| {pid} | {fid} | {ok[0]['age']} | {', '.join(f'{x:.0f}' for x in ms)} | "
                   f"{statistics.median(ms):.0f} |")
    return out + [""]


# ---- compare ------------------------------------------------------------------

def medians(label):
    label = Path(label)
    out = {}
    for (preset, age), group in groups(trees(label)).items():
        drawn = [t for t in group if "height" in t]
        out[(SHORT[preset], age, "drawn")] = len(drawn)
        for m, _, _ in MEASURES:
            s = summary(drawn, m)
            if s:
                out[(SHORT[preset], age, m)] = s[0]
    cost = label / "cost.jsonl"
    if cost.exists():
        for line in cost.read_text().splitlines():
            c = json.loads(line)
            for k in ("seconds", "rss_gib"):
                if k in c:
                    out[(SHORT[c["preset"]], c["age"], f"cost {k}")] = c[k]
    many = label / "many.json"
    if many.exists() and many.read_text().strip():
        out[("oak", 40, "thousand seconds")] = json.loads(many.read_text())["seconds"]
    wasm = label / "wasm.jsonl"
    if wasm.exists():
        ws = {}
        for line in wasm.read_text().splitlines():
            if line.startswith("{") and "ms" in line:
                w = json.loads(line)
                ws.setdefault((w["id"], w["fidelity"]), []).append(w["ms"])
        for (pid, fid), ms in ws.items():
            out[(pid, "preset", f"wasm ms fidelity {fid}")] = statistics.median(ms)
    return out


def compare(a, b):
    ma, mb = medians(a), medians(b)
    moved = 0
    print(f"{'tree':<22} {'measure':<34} {Path(a).name:>14} {Path(b).name:>14}  change")
    for k in sorted(set(ma) | set(mb), key=str):
        va, vb = ma.get(k), mb.get(k)
        tree = f"{k[0]} at {k[1]}"
        if va is None or vb is None:
            print(f"{tree:<22} {k[2]:<34} {str(va):>14} {str(vb):>14}  only in one")
            moved += 1
            continue
        change = (vb - va) / abs(va) if va else (0.0 if vb == va else float("inf"))
        if abs(change) > 0.05:
            print(f"{tree:<22} {k[2]:<34} {va:>14.4g} {vb:>14.4g}  {change:+.1%}")
            moved += 1
    print(f"{moved} of {len(set(ma) | set(mb))} measures moved by more than 5%")


if __name__ == "__main__":
    what = sys.argv[1] if len(sys.argv) > 1 else ""
    if what == "sheets":
        sheets(sys.argv[2], sys.argv[3])
    elif what == "tables":
        tables(sys.argv[2])
    elif what == "compare":
        compare(sys.argv[2], sys.argv[3])
    else:
        sys.exit(__doc__)
