"""fn-334 R3: the raw logs as Markdown tables.

Usage: python3 table.py [presets...]. Reads raw/r3-<preset>.log and prints the
read per (preset, candidate, fidelity), every refusal, and each age's rows.
"""
import sys

EV = ".flow/evidence/fn-334-the-dials-rows-are-derived-from-the/raw/"
presets = sys.argv[1:]
STATS = ["height", "spread", "trunk", "limbs", "shed", "area"]

print("| Preset | Read | f | " + " | ".join(STATS) + " | Worst | Seconds | Speed-up | Axes | Apex-cycles |")
print("| --- | --- | --- |" + " --- |" * (len(STATS) + 5))
rows, other = [], []
for p in presets:
    for line in open(f"{EV}r3-{p}.log"):
        i = min((line.find(t) for t in ("REFERENCE", "CAND", "FAILED", "NOSPECIES", "NOREFERENCE", "ROWS") if line.find(t) >= 0), default=-1)
        if i < 0:
            continue
        line = line[i:].rstrip()
        w = line.split()
        if w[0] == "CAND":
            r = {w[k]: w[k + 1] for k in range(5, len(w) - 1, 2)}
            cells = [r[s] for s in STATS]
            print(f"| {w[1]} | {w[2]} | {w[4]} | " + " | ".join(cells)
                  + f" | {r['worst']} {r['at']} | {r['seconds']} | {r['speedup']} | {r['axes']} | {r['apexes']} |")
        elif w[0] == "ROWS":
            rows.append(line)
        else:
            other.append(line)
print()
for line in other:
    print("    " + line)
print()
for line in rows:
    print("    " + line)
