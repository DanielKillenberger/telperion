"""What a surfacing budget sized from measured demand would hold, against the shared buffers now."""
import json, math, sys

MiB = 2**20
SHARED = ("curve counts", "curve offsets", "curve blocks", "curve records", "curve vertices", "curve indices")


def slice_words(elements, slices=2):
    return math.ceil(max(math.ceil(elements / slices), 1) / 64) * 64


def bytes_for(d):
    rings, verts, tube, ribbon = d
    records = 2 * math.ceil(max(math.ceil(math.ceil(rings / 2) / 32), 1)) * 32 * 22 * 4
    vertices = 2 * slice_words(verts * 9) * 4
    indices = 2 * slice_words(tube + ribbon) * 4
    return records + vertices + indices


print("| Tree | age | shared now MiB | largest measured demand ×1.25, MiB | share of now |")
print("|---|--:|--:|--:|--:|")
for line in open(sys.argv[1]):
    x = json.loads(line)["run"]
    if not x or x["version"] != "new" or x["seed"] != 1 or not x.get("curves"):
        continue
    now = sum(x["alloc_resident"]["by"].get(n, [0])[0] for n in SHARED
              if n not in ("curve counts", "curve offsets", "curve blocks"))
    most = [0, 0, 0, 0]
    for v in x["curves"].values():
        for side in ("camera", "sun"):
            if v[side]:
                most = [max(a, b) for a, b in zip(most, v[side]["demand"])]
    wanted = bytes_for([math.ceil(1.25 * m) for m in most])
    print(f"| {x['preset']} | {x['age']:g} | {now/MiB:.0f} | {wanted/MiB:.0f} | {wanted/now:.0%} |")
