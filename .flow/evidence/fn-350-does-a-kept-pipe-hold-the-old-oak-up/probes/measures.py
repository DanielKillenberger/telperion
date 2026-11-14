"""fn-350 R2 and R5: each tree's measures at retained 0, 0.5 and 1, from
the logs run.sh wrote (raw/stills/<sp>-<age>-<seed>-R<r>.log).

Usage: python3 -I probes/measures.py <evidence dir>
Prints one markdown table: height, spread, trunk diameter at 1.3 m (a
star where the stem line never stood 1.3 m up), the largest limb's base
diameter, nodes, leaves, grow seconds (8 threads) and peak GiB of the
drawn run, or the refusal's text.
"""
import re
import sys
from pathlib import Path

E = Path(sys.argv[1])
S = E / "raw" / "stills"
TREES = [("oak", a, s) for s in (1001, 1007, 1012) for a in (120, 200, 300)]
TREES += [("oak", 40, 1001), ("oak", 80, 1001), ("beech", 40, 1001)]
VALUES = ("0", "0.5", "1")


def read(sp, age, seed, r):
    log = S / f"{sp}-{age}-{seed}-R{r}.log"
    if not log.exists():
        return None
    text = log.read_text()
    m = re.search(r'Error: "(.*)"', text)
    if m:
        return {"error": m[1].split(": invalid input: ")[-1]}
    out = {}
    m = re.search(r"grown in ([\d.]+) ms.*?; (\d+) nodes, (\d+) leaves; ([\d.]+) m tall, "
                  r"([\d.]+) x ([\d.]+) m", text)
    if m:
        out.update(s=float(m[1]) / 1e3, nodes=int(m[2]), leaves=int(m[3]), h=m[4],
                   w=f"{m[5]} x {m[6]}")
    m = re.search(r"trunk ([\d.]+) cm at 1.3 m \(reached (\w+)\); largest limb ([\d.]+) cm", text)
    if m:
        out.update(dbh=m[1] + ("" if m[2] == "true" else "*"), limb=m[3])
    m = re.search(r"^PEAK seconds ([\d.]+) rss_gib ([\d.]+)", text, re.M)
    if m:
        out.update(gib=m[2])
    return out


print("| tree | retained | height m | spread m | trunk at 1.3 m, cm | largest limb base, cm "
      "| nodes | leaves | grown s (8 thr) | peak GiB |")
print("|---|---|---|---|---|---|---|---|---|---|")
for sp, age, seed in TREES:
    for r in VALUES:
        d = read(sp, age, seed, r)
        name = f"{sp} {age} {seed}"
        if d is None:
            print(f"| {name} | {r} | not run | | | | | | | |")
        elif "error" in d:
            print(f"| {name} | {r} | refused: {d['error']} | | | | | | | |")
        else:
            g = lambda k, f="{}": f.format(d[k]) if k in d else "-"
            print(f"| {name} | {r} | {g('h')} | {g('w')} | {g('dbh')} | {g('limb')} | "
                  f"{g('nodes')} | {g('leaves')} | {g('s', '{:.1f}')} | {g('gib')} |")
