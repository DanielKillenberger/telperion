"""Judges a twenty-step blend walk from `dump blend`: per metric, the largest
single step against a uniform share of the total change (the step ratio) and
against the metric's mean (the step share). A metric is smooth when its
largest step is at most three uniform shares, or under 2% of its mean."""
import json
import sys

def judge(path: str) -> bool:
    rows = [json.loads(line) for line in open(path)]
    names = list(rows[0]["metrics"])
    smooth = True
    print(f"{path}")
    for name in names:
        values = [r["metrics"][name] for r in rows]
        steps = [abs(b - a) for a, b in zip(values, values[1:])]
        total = sum(steps)
        mean = sum(abs(v) for v in values) / len(values)
        worst = max(steps)
        ratio = worst / (total / len(steps)) if total else 0.0
        share = worst / mean if mean else 0.0
        ok = ratio <= 3.0 or share <= 0.02
        smooth &= ok
        print(f"  {name:13s} {values[0]:>12.2f} -> {values[-1]:>12.2f}  "
              f"largest step {worst:>10.2f} at t={steps.index(worst) / 20 + 0.05:.2f}  "
              f"ratio {ratio:4.2f}  share {share:6.1%}  {'smooth' if ok else 'JUMP'}")
    print(f"  verdict: {'smooth' if smooth else 'not smooth'}")
    return smooth

if __name__ == "__main__":
    results = [judge(p) for p in sys.argv[1:]]
    sys.exit(0 if all(results) else 1)
