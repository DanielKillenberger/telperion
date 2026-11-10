"""fn-340 R1c (c): the replay's DECADE lines, the law at the bearing unit's
size over the cycle's own cohort-age laterals, per channel, by the bearing
unit's age in decades, at the tree's age.

Run: python3 -I r1c-replay.py <replay log> [...]
"""
import sys

Q = ["grew", "living", "standing", "leaf_cur", "leaf_drawn", "wood", "section", "phyto", "roots", "lit"]
SHOW = ["grew", "living", "lit", "leaf_cur", "wood", "standing", "roots"]
UPKEEP = 0.35  # the oak's; upkeep is wood times it on both sides, so its ratio is wood's


def main():
    for path in sys.argv[1:]:
        rows, head = {}, ""
        for line in open(path):
            w = line.split()
            if w and w[0] == "HEAD":
                head = " ".join(w[1:])
            if w and w[0] == "DECADE":
                rows[(int(w[2]), w[3])] = dict(zip(Q, map(float, w[4:])))
        print(f"## {head}\n")
        print("| unit age | " + " | ".join(SHOW) + " |")
        print("|---|" + "---|" * len(SHOW))
        for d in sorted({k[0] for k in rows}):
            c, l = rows[(d, "cycle")], rows[(d, "law")]
            if not any(c[q] or l[q] for q in SHOW):
                continue
            cells = []
            for q in SHOW:
                if c[q] == 0 and l[q] == 0:
                    cells.append("-")
                elif c[q] == 0:
                    cells.append(f"0 / {l[q]:.3g}")
                else:
                    cells.append(f"{l[q] / c[q]:.3f} ({c[q]:.3g})")
            print(f"| {10 * d}-{10 * d + 9} | " + " | ".join(cells) + " |")
        print()


if __name__ == "__main__":
    main()
