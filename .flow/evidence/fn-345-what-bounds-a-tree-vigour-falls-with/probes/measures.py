"""fn-345 R3: each drawn tree's measures, today (A) beside the probe (N),
from raw/stills/<name>-<age>-<seed>-<v>.log, and the one-thread seconds
from the matching -time.log where one was run.

Usage: python3 -I probes/measures.py <evidence dir>
Columns: height, spread, trunk at 1.3 m, the settled stem line's lowest
living limb and its living limbs (oak rows only; PA 6), current-year
leaves, held phytomers (settled), peak resident GiB of the drawn run, and
one-thread grow seconds.
"""
import re, sys
from pathlib import Path

S = Path(sys.argv[1]) / "raw" / "stills"
TREES = ["oak-40-1001", "oak-120-1001", "oak-200-1001", "oak-300-1001",
         "oak-40-1007", "oak-120-1007", "oak-200-1007", "oak-300-1007",
         "beech-40-1001", "beech-80-1001", "birch-40-1001", "spruce-40-1001",
         "palm-40-1001", "ordinary-40-1001"]


def read(tree, v):
    log = S / f"{tree}-{v}.log"
    if not log.exists():
        return None
    text = log.read_text()
    out = {}
    m = re.search(r"(\d+) leaves; ([\d.]+) m tall, ([\d.]+) x ([\d.]+) m.*trunk ([\d.]+) cm", text)
    if m:
        out.update(leaves=int(m[1]), h=float(m[2]), w=f"{m[3]} × {m[4]}", dbh=float(m[5]))
    m = re.search(r"^SETTLED top=\S+ limbs_live=(\d+) lowest_limb=(\S+) phytomers=(\d+)", text, re.M)
    if m:
        out.update(limbs=int(m[1]), lowest=float(m[2]), held=int(m[3]))
    m = re.search(r"^PEAK seconds ([\d.]+) rss_gib ([\d.]+)", text, re.M)
    if m:
        out.update(gib=float(m[2]))
    t = S / f"{tree}-{v}-time.log"
    if t.exists():
        m = re.search(r"^PEAK seconds ([\d.]+)", t.read_text(), re.M)
        if m:
            out["s1"] = float(m[1])
    m = re.search(r"needs (\d+) bytes, above the device", text)
    if m:
        out["refused"] = True
    return out


def cell(r, k, f):
    return f.format(r[k]) if r and k in r else "-"


print("| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for t in TREES:
    for v in "AN":
        r = read(t, v)
        if r is None:
            continue
        oak = t.startswith("oak")
        print("| " + " | ".join([
            t if v == "A" else "", v, cell(r, "h", "{:.1f}"), r.get("w", "-"), cell(r, "dbh", "{:.0f}"),
            cell(r, "lowest", "{:.1f}") if oak else "n/a", cell(r, "limbs", "{}") if oak else "n/a",
            cell(r, "leaves", "{:,}"), cell(r, "held", "{:,}"), cell(r, "gib", "{:.1f}"), cell(r, "s1", "{:.1f}"),
        ]) + " |")
