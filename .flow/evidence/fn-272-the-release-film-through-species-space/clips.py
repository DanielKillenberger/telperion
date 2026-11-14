"""Short clip paths cut from the release path (fn-272, owner 2026-10-19):
one segment each, its camera keys shifted to the clip's start, and the year
fade varied. Writes <out>/<name>.json."""
import json, sys

release, out = sys.argv[1], sys.argv[2]
base = json.load(open(release))

def clip(name, segment, start, fade, seconds=None):
    p = json.loads(json.dumps(base))
    s = dict(base["segments"][segment])
    s.pop("hold", None); s.pop("line", None)
    if seconds: s["seconds"] = seconds
    end = start + s["seconds"]
    keys = [k for k in base["camera"]["keys"] if start <= k["at"] <= end]
    p["camera"]["keys"] = [dict(k, at=k["at"] - start) for k in keys]
    p["camera"]["azimuth"] = base["camera"]["azimuth"] + base["camera"]["orbit"] * start
    p["segments"], p["fade"] = [s], fade
    json.dump(p, open(f"{out}/{name}.json", "w"), indent=1)

for fade in (0, 0.25, 1):
    clip(f"beech-fade{fade}", 0, 0, fade)
    clip(f"laurelin-fade{fade}", 7, 24, fade)
clip("walk-beech-spruce", 1, 7, 1, seconds=1)
clip("walk-laurelin-telperion", 8, 31, 1, seconds=1)
