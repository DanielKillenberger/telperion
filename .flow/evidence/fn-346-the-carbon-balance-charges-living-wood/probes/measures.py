"""fn-346: each drawn tree's measures, today (A) beside P1 and P2.

Today's logs are fn-345's (same crates, its probe dormant) but for the oak
at 80, drawn here. Usage: python3 -I probes/measures.py
Columns: height, spread, trunk at 1.3 m, the settled stem line's lowest
living limb and its living limbs (oak rows only; PA 6), current-year
leaves, held phytomers (settled), peak resident GiB of the drawn run, and
one-thread grow seconds.
"""
import re
from pathlib import Path

E = Path(".flow/evidence/fn-346-the-carbon-balance-charges-living-wood/raw/stills")
T345 = Path(".flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills")
TREES = ["oak-40-1001", "oak-80-1001", "oak-120-1001", "oak-200-1001", "oak-300-1001",
         "oak-120-1007", "oak-300-1007", "beech-40-1001", "beech-80-1001", "birch-40-1001",
         "spruce-40-1001", "palm-40-1001", "ordinary-40-1001"]


def read(tree, v):
    d = T345 if v == "A" and (T345 / f"{tree}-A.log").exists() else E
    log = d / f"{tree}-{v}.log"
    if not log.exists():
        return None
    text = log.read_text()
    out = {}
    m = re.search(r"(\d+) leaves; ([\d.]+) m tall, ([\d.]+) x ([\d.]+) m.*trunk ([\d.]+) cm", text)
    if m:
        out.update(leaves=int(m[1]), h=float(m[2]), w=f"{m[3]} × {m[4]}", dbh=float(m[5]))
    m = re.search(r"^SETTLED top=\S+ limbs_live=(\d+) lowest_limb=(\S+) phytomers=(\d+)", text, re.M)
    if m:
        out.update(limbs=int(m[1]), lowest=m[2], held=int(m[3]))
    m = re.search(r"^PEAK seconds ([\d.]+) rss_gib ([\d.]+)", text, re.M)
    if m:
        out.update(gib=float(m[2]))
    m = re.search(r'Error: "(.*)"', text)
    if m:
        out["error"] = m[1]
    t = d / f"{tree}-{v}-time.log"
    if t.exists():
        m = re.search(r"^PEAK seconds ([\d.]+)", t.read_text(), re.M)
        if m:
            out["s1"] = float(m[1])
    return out


def cell(r, k, f):
    return f.format(r[k]) if r and k in r else "-"


print("| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for t in TREES:
    for v in ["A", "P1", "P2"]:
        r = read(t, v)
        if r is None:
            continue
        oak = t.startswith("oak")
        low = r.get("lowest", "-")
        low = "none" if low.lower() == "nan" else (f"{float(low):.1f}" if low != "-" else "-")
        if "error" in r and "h" not in r:
            print(f"| {t if v == 'A' else ''} | {v} | refused: {r['error']} | | | | | | | {cell(r, 'gib', '{:.1f}')} | {cell(r, 's1', '{:.1f}')} |")
            continue
        print("| " + " | ".join([
            t if v == "A" else "", v, cell(r, "h", "{:.1f}"), r.get("w", "-"), cell(r, "dbh", "{:.0f}"),
            low if oak else "n/a", cell(r, "limbs", "{}") if oak else "n/a",
            cell(r, "leaves", "{:,}"), cell(r, "held", "{:,}"), cell(r, "gib", "{:.1f}"), cell(r, "s1", "{:.1f}"),
        ]) + " |")
