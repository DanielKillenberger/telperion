"""R3: F4 after the cuts against master, at the default age and at master's age."""
import json, sys

MiB = 2**20
SHORT = {"european-beech": "beech", "oregon-white-oak": "oak", "norway-spruce": "spruce",
         "silver-birch": "birch", "date-palm": "palm", "ordinary": "ordinary",
         "laurelin": "Laurelin", "telperion": "Telperion"}
ORDER = list(SHORT)

rows = {}
failed = []
for line in open(sys.argv[1]):
    r = json.loads(line)
    x = r["run"]
    if not x:
        continue
    rows[(x["version"], x["preset"], x["seed"], x["age"])] = x

orbit = {}
if len(sys.argv) > 2:
    for line in open(sys.argv[2]):
        x = json.loads(line)["run"]
        if x:
            orbit[(x["version"], x["preset"], x["seed"], x["age"])] = x["orbit"]


def mark(new, old):
    return "**" if new > old else ""


print("| Tree | seed | ages old/new | GPU process MB old→new | tree buffers MiB old→new | frame ms whole old→new | mid | close | orbit total ms whole old→new | peak RSS MB old→new |")
print("|---|--:|---|--:|--:|--:|--:|--:|--:|--:|")
for kind in ("default", "same"):
    for p in ORDER:
        for seed in (1, 42):
            old = [v for k, v in rows.items() if k[0] == "old" and k[1] == p and k[2] == seed]
            if not old:
                continue
            old = old[0]
            news = sorted([v for k, v in rows.items() if k[0] == "new" and k[1] == p and k[2] == seed], key=lambda v: v["age"])
            want = [n for n in news if (n["age"] == old["age"]) == (kind == "same")]
            if kind == "same" and not want:
                print(f"| {SHORT[p]} | {seed} | {old['age']:g}/{old['age']:g} | refused (device memory) | | | | | | |")
                continue
            if not want:
                continue
            n = want[0]
            def f(k):
                a, b = old["frame_ms"][k], n["frame_ms"][k]
                return f"{a:.2f}→{mark(b, a)}{b:.2f}{mark(b, a)}"
            ot = orbit.get(("old", p, seed, old["age"]), {}).get("whole", {}).get("total")
            nt = orbit.get(("new", p, seed, n["age"]), {}).get("whole", {}).get("total")
            orb = f"{ot:.2f}→{mark(nt, ot)}{nt:.2f}{mark(nt, ot)}" if ot and nt else "-"
            g = (old["gpu_process_mb"], n["gpu_process_mb"])
            t = (old["tree_buffer_bytes"] / MiB, n["tree_buffer_bytes"] / MiB)
            r = (old["peak_rss_mb"], n["peak_rss_mb"])
            print(f"| {SHORT[p]} | {seed} | {old['age']:g}/{n['age']:g} | {g[0]}→{mark(g[1], g[0])}{g[1]}{mark(g[1], g[0])} | {t[0]:.0f}→{mark(t[1], t[0])}{t[1]:.0f}{mark(t[1], t[0])} | {f('whole')} | {f('mid')} | {f('close')} | {orb} | {r[0]}→{mark(r[1], r[0])}{r[1]}{mark(r[1], r[0])} |")
    print("| | | | | | | | | | |")
