"""fn-257's growth profile: each case's growth by phase from
`growth_profile`'s files (raw/profile): stamped GPU ms (measured) and the
bytes each phase moves, computed by fn-256's `roofline.py` `records` from
each cycle's census (an upper estimate within about 2x; the light phase
counted lit, its upper bound). `python3 profile.py raw/profile`."""
import collections
import csv
import pathlib
import sys

DIR = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "raw/profile")
HERE = pathlib.Path(__file__).resolve().parent
# roofline.py runs its tables on import; take its definitions alone.
source = (HERE.parent / "fn-256-the-whole-grower-runs-on-the-gpu" / "roofline.py").read_text()
roofline = type(sys)("roofline")
exec(compile(source.replace("\nmain()\n", "\n"), "roofline.py", "exec"), roofline.__dict__)
PEAK = 705.0  # GB/s, ROOFLINE.md's copy kernel


def case(stem):
    stamps = collections.defaultdict(float)
    count = collections.Counter()
    for r in csv.DictReader(open(DIR / f"{stem}-stamps.csv")):
        stamps[r["phase"]] += float(r["ns"])
        count[r["phase"]] += 1
    census = [[int(r[f"w{k}"]) for k in range(256)] for r in csv.DictReader(open(DIR / f"{stem}-census.csv"))]
    kern = sum(count.values()) / len(census)
    moved = collections.defaultdict(float)
    for w in census:
        out, _ = roofline.records(w, True, kern)
        for p, (_, b, _) in out.items():
            moved[p] += 4 * b
    total_ms = sum(stamps.values()) / 1e6
    total_gb = sum(moved.values()) / 1e9
    print(f"\n### {stem}: {total_ms:.1f} ms stamped, {sum(count.values()):,} passes, {total_gb:.2f} GB moved (c), {total_gb / PEAK * 1e3:.1f} ms at 705 GB/s\n")
    print("| Phase | GPU ms (m) | passes (m) | GB (c) | ms at 705 GB/s (c) |")
    print("|---|--:|--:|--:|--:|")
    for p in sorted(stamps, key=lambda p: -stamps[p]):
        gb = moved.get(p, 0.0) / 1e9
        print(f"| {p} | {stamps[p] / 1e6:.1f} | {count[p]:,} | {gb:.2f} | {gb / PEAK * 1e3:.2f} |")


for f in sorted(DIR.glob("*-stamps.csv")):
    case(f.name[: -len("-stamps.csv")])
