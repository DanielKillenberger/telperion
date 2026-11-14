"""fn-340 R1b: the diagnostic's yearly factor, the cycle's cohort upkeep over
the probe's, from R1's trace (`raw/trace-<seed>-c.log`), one per cycle from
1 to 120 (1 where either is missing). A fitted factor for one diagnostic
run, never a correction.

Run: python3 -I diag-ratios.py <trace log> <seed>
"""
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
import r1  # noqa: E402

y, _ = r1.load([sys.argv[1]])
seed = int(sys.argv[2])
b, p = y[("base", seed)], y[("probe", seed)]
print(",".join(f"{b[k]['c_c'] / p[k]['c_c']:.4f}" if k in b and k in p and p[k]["c_c"] > 0 else "1.0"
               for k in range(1, 121)))
