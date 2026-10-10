"""Markdown tables for BYTES.md from probe rows."""
import json, sys

MiB = 2**20
SCRATCH = ("chain axes", "chain sleeping buds", "chain phytomers", "chain apexes", "chain status")
POOL = ("laid scratch", "fine laid", "packed axes", "packed phytomers")
SHORT = {"european-beech": "beech", "oregon-white-oak": "oak", "norway-spruce": "spruce",
         "silver-birch": "birch", "date-palm": "palm", "ordinary": "ordinary",
         "laurelin": "Laurelin", "telperion": "Telperion"}


def by(a, names):
    return sum(a["by"].get(n, [0])[0] for n in names) / MiB


rows = [json.loads(l)["run"] for l in open(sys.argv[1])]
rows = [r for r in rows if r]
print("| Tree | Version, age | smi MB | allocated / reserved MiB | drawn tree MiB (curve data + foliage) | camera budget | sun budget | growth scratch (regions) | pool | targets | RSS / peak MB |")
print("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for r in rows:
    a = r["alloc_resident"]
    named = set(a["by"])
    old_wood = sum(v[0] for k, v in a["by"].items() if "wood" in k) / MiB
    foliage = sum(v[0] for k, v in a["by"].items() if "foliage" in k or "leaf" in k or k in ("selection", "generated florets", "generated mass")) / MiB
    data = by(a, ("curve points", "curve clusters", "curve sections")) + old_wood + foliage
    print(f"| {SHORT[r['preset']]} | {r['version']} {r['age']:g} | {r['gpu_process_mb']} | {a['allocated']/MiB:.0f} / {a['reserved']/MiB:.0f} | {data:.1f} | {by(a, ('curve camera',)):.0f} | {by(a, ('curve sun',)):.0f} | {by(a, SCRATCH):.0f} | {by(a, POOL):.0f} | {by(a, ('probe', 'probe depth', 'probe resolve', 'shadow map')):.0f} | {r['rss_mb']} / {r['peak_rss_mb']} |")

print()
print("| Tree | Version, age | build ms | view | frame ms | vegetation | selection | shadow | surfacing |")
print("|---|---|--:|---|--:|--:|--:|--:|--:|")
for r in rows:
    for v in ("whole", "mid", "close"):
        p = r["passes"][v]
        f = lambda k: "-" if p[k] is None else f"{p[k]:.2f}"
        head = f"| {SHORT[r['preset']]} | {r['version']} {r['age']:g} | {min(r['builds_ms']):.0f}" if v == "whole" else "| | |"
        print(f"{head} | {v} | {r['frame_ms'][v]:.2f} | {f('vegetation')} | {f('selection')} | {f('shadow')} | {f('surfacing')} |")
