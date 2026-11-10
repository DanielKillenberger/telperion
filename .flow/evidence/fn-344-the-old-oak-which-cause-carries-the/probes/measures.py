"""fn-344: the measures of each still run beside A's, from raw/stills logs.

Usage: python3 -I probes/measures.py <evidence dir>
Per tree and variant: height, spread, trunk at 1.3 m, the settled stem
line's lowest living limb and living limbs, current-year leaves, peak
resident GiB and wall seconds; held phytomers at the end against A's, the
first year the held count stands 1.25 times A's and never falls back under
it (the year it left today's curve), and the census stop where one fired.
"""
import re, sys
from pathlib import Path

E = Path(sys.argv[1])
S = E / "raw" / "stills"
TREES = ["120-1001", "200-1001", "300-1001", "300-1007"]


def read(tree, v):
    log = S / f"oak-{tree}-{v}.log"
    if not log.exists():
        return None
    text = log.read_text()
    out = {"census": {}}
    for m in re.finditer(r"^CENSUS (\d+) (\d+)", text, re.M):
        out["census"][int(m[1])] = int(m[2])
    m = re.search(r"(\d+) leaves; ([\d.]+) m tall, ([\d.]+) x ([\d.]+) m.*trunk ([\d.]+) cm", text)
    if m:
        out.update(leaves=int(m[1]), h=float(m[2]), w=f"{m[3]} x {m[4]}", dbh=float(m[5]))
    m = re.search(r"^SETTLED top=\S+ limbs_live=(\d+) lowest_limb=(\S+) phytomers=(\d+)", text, re.M)
    if m:
        out.update(limbs=int(m[1]), lowest=float(m[2]))
    m = re.search(r"^PEAK seconds ([\d.]+) rss_gib ([\d.]+) exit (\d+)", text, re.M)
    if m:
        out.update(secs=float(m[1]), gib=float(m[2]), exit=int(m[3]))
    m = re.search(r"^STOP (.*)$", text, re.M)
    if m:
        out["stop"] = m[1]
    m = re.search(r"needs (\d+) bytes, above the device", text)
    if m:
        out["refused"] = f"wood points {int(m[1]) / 2**20:.0f} MiB over the device limit"
    return out


def left(a, v):
    years = sorted(set(a) & set(v))
    out = None
    for y in years:
        if v[y] >= 1.25 * a[y]:
            out = out if out is not None else y
        else:
            out = None
    return out


print("| tree | variant | height m | spread m | trunk cm | lowest living limb m | living limbs | leaves | peak GiB | s | held / A at end | left A's curve |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|")
for t in TREES:
    a = read(t, "A")
    if a is None:
        continue
    for v in "ABCDEF":
        r = read(t, v)
        if r is None:
            continue
        last = max(r["census"]) if r["census"] else None
        ratio = r["census"][last] / a["census"][last] if last is not None and last in a["census"] else float("nan")
        note = r.get("stop") or r.get("refused") or ""
        cells = [
            t, v,
            f"{r['h']:.1f}" if "h" in r else "-",
            r.get("w", "-"),
            f"{r['dbh']:.0f}" if "dbh" in r else "-",
            f"{r['lowest']:.1f}" if "lowest" in r else "-",
            str(r.get("limbs", "-")),
            f"{r['leaves']:,}" if "leaves" in r else "-",
            f"{r['gib']:.1f}" if "gib" in r else "-",
            f"{r['secs']:.0f}" if "secs" in r else "-",
            f"{ratio:.2f}" + (f" (year {last})" if note else ""),
            (str(left(a["census"], r["census"])) if v != "A" else "") + (f"; {note}" if note else ""),
        ]
        print("| " + " | ".join(cells) + " |")
