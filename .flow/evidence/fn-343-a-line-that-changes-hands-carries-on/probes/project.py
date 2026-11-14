"""fn-343 R1: joints >= 10 deg per m on walked wood 1 cm or more (the joint's
taken wood r >= 1 cm, fn-342's measure), split into take-overs whose bearer
stopped, take-overs whose bearer lives, and the rest; and the relay's own
turns on that wood, for an on-paper projection.
Run: python3 -I project.py <log> [<log> ...]"""
import sys
from collections import Counter
for path in sys.argv[1:]:
    metres, c, relay = 0.0, Counter(), []
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "SEG":
            r = float(w[10].split("=")[1])
            if r >= 0.01:
                x = [float(v) for v in w[4:10]]
                metres += sum((a - b) ** 2 for a, b in zip(x[:3], x[3:])) ** 0.5
        elif w[0] == "JOINT":
            d = dict(t.split("=", 1) for t in w[4:] if "=" in t)
            if float(d["r"]) < 0.01:
                continue
            t = float(d["turn"])
            if d["cause"] == "relay":
                relay.append(t)
            if t < 10:
                continue
            if d["cause"] in ("fork", "lateral_at_end"):
                k = "take-over, bearer living" if d["apex_end"] == "-1" else "take-over, bearer stopped"
                if k.endswith("stopped") and d["stop"].startswith("moved_on"):
                    k += " (moved_on)"
            else:
                k = d["cause"]
            c[k] += 1
    n = sum(c.values())
    relay.sort()
    print(f"{path.split('/')[-1]}: {metres:.0f} m of walked wood >= 1 cm, {n} joints >= 10 deg, {n / metres:.3f} per m")
    for k, v in c.most_common():
        print(f"  {k:<36} {v:>5}  {v / metres:.3f} per m  ({v / n:.2f})")
    print(f"  relay joints on that wood: {len(relay)}, median {relay[len(relay) // 2]:.1f} deg, "
          f">= 10 deg {sum(t >= 10 for t in relay) / len(relay):.2f}")
