"""Compact table of probe rows: memory split and per-pass frame time."""
import json, sys

MiB = 2**20
GROUPS = {
    "targets": lambda n: n in ("probe", "probe depth", "probe resolve", "shadow map"),
    "wgpu": lambda n: n.startswith("(wgpu internal)"),
    "curve budgets": lambda n: n in ("curve camera", "curve sun"),
    "curve data": lambda n: n in ("curve points", "curve clusters", "curve sections"),
    "old wood": lambda n: "wood" in n,
    "foliage": lambda n: "foliage" in n or "leaf" in n or n in ("selection",),
}


def group(name):
    for g, test in GROUPS.items():
        if test(name):
            return g
    return "other:" + name


def split(alloc):
    out = {}
    for name, (b, _) in alloc["by"].items():
        g = group(name)
        if g.startswith("other:") and b < MiB:
            g = "small"
        out[g] = out.get(g, 0) + b
    return out


rows = [json.loads(l) for l in open(sys.argv[1])]
for r in rows:
    x = r["run"]
    if not x:
        print("FAILED row (rc %s)" % r["rc"])
        continue
    a = x["alloc_resident"]
    s = split(a)
    parts = " ".join(f"{k}={v/MiB:.1f}" for k, v in sorted(s.items(), key=lambda kv: -kv[1]) if v >= 0.05 * MiB)
    p = x["passes"]
    def pt(v):
        q = p[v]
        f = lambda k: "-" if q[k] is None else f"{q[k]:.2f}"
        return f"{x['frame_ms'][v]:.2f}[v{f('vegetation')} s{f('selection')} sh{f('shadow')} c{f('surfacing')}]"
    print(f"{x['version']} {x['preset'][:9]:9} s{x['seed']} a{x['age']:<4} build {min(x['builds_ms']):8.1f} | smi {x['gpu_process_mb']:5} alloc {a['allocated']/MiB:7.1f} resv {a['reserved']/MiB:7.0f} tree {x['tree_buffer_bytes']/MiB:7.1f} rss {x['rss_mb']} peak {x['peak_rss_mb']}")
    print(f"    {parts}")
    print(f"    whole {pt('whole')} mid {pt('mid')} close {pt('close')}")
    if x.get("curves"):
        for v, c in x["curves"].items():
            cam = c["camera"]
            if cam:
                print(f"    curve {v}: scale {cam['scale']} overrun {cam['overrun']} verts {cam['vertices']} tri {cam['tube_triangles']}+{cam['ribbon_triangles']} demand {cam['demand']} budget {cam['budget']}")
        sun = next(iter(x["curves"].values()))["sun"]
        if sun:
            print(f"    curve sun: verts {sun['vertices']} tri {sun['tube_triangles']}+{sun['ribbon_triangles']} demand {sun['demand']} budget {sun['budget']}")
