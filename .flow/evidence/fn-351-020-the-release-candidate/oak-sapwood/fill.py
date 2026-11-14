"""fn-351: writes the "Measured" table of OAK-SAPWOOD-BY-SIZE.md between its
two markers from the probes' logs, beside the paper (option B), the recorded
before-rows and White's range.

    python3 -I fill.py <raw/oakwood dir> <OAK-SAPWOOD-BY-SIZE.md>
"""
import glob
import json
import os
import re
import sys

# fn-364 MEASURES.md "fn-363" rows: (trunk cm, height m), the candidate's grower.
RECORD = {
    (120, 1001): (115, 27.4), (120, 1007): (165, 28.4), (120, 1012): (134, 26.0),
    (200, 1001): (180, 36.8), (200, 1007): (259, 37.1), (200, 1012): (227, 39.4),
    (300, 1001): (220, 36.0), (300, 1007): (313, 38.8), (300, 1012): (290, 38.2),
}
# paper.py, "today" and option B, cm.
PAPER = {
    1001: {40: (28, 33), 80: (71, 78), 120: (115, 108), 200: (175, 151), 300: (215, 179)},
    1007: {40: (35, 42), 80: (99, 102), 120: (163, 146), 200: (251, 211), 300: (306, 250)},
    1012: {40: (34, 40), 80: (86, 93), 120: (138, 131), 200: (221, 190), 300: (285, 237)},
}
WHITE = {40: (28, 40), 80: (56, 79), 120: (83, 109), 200: (121, 152), 300: (156, 192)}

runs = {}
for path in glob.glob(os.path.join(sys.argv[1], "*-oregon-white-oak-*.log")):
    side, age, seed = re.match(r"(cand|oakwood)-oregon-white-oak-(\d+)-(\d+)\.log$", os.path.basename(path)).groups()
    text = open(path).read()
    tree = re.search(r"^TREE (.*)$", text, re.M)
    if tree and re.search(r"^PEAK .* exit 0$", text, re.M):
        runs[(int(age), int(seed), side)] = json.loads(tree.group(1))


def cell(t, key, scale=1.0, digits=1):
    return "-" if t is None or t.get(key) is None else f"{t[key] * scale:.{digits}f}"


lines = [
    "| Age | Seed | Trunk before, cm | Trunk after, cm | On paper, before to after | White, cm | After against White | Height before to after, m | Drawn leaf before to after, m² |",
    "|---|---|---|---|---|---|---|---|---|",
]
for age in (40, 80, 120, 200, 300):
    for seed in (1001, 1007, 1012):
        before, after = runs.get((age, seed, "cand")), runs.get((age, seed, "oakwood"))
        rec = RECORD.get((age, seed))
        trunk0 = cell(before, "dbh", 100) if before else (f"{rec[0]} [record]" if rec else "not run")
        height0 = cell(before, "height") if before else (f"{rec[1]} [record]" if rec else "not run")
        low, high = WHITE[age]
        if after and after.get("dbh") is not None:
            d = after["dbh"] * 100
            verdict = "in" if low <= d <= high else f"{d / high:.2f} times its top" if d > high else f"{d / low:.2f} of its low end"
        else:
            verdict = "not run"
        paper = PAPER[seed][age]
        lines.append(
            f"| {age} | {seed} | {trunk0} | {cell(after, 'dbh', 100) if after else 'not run'} | {paper[0]} to {paper[1]} | {low} to {high} | {verdict} | "
            f"{height0} to {cell(after, 'height') if after else 'not run'} | "
            f"{cell(before, 'leaf_area', 1, 0) if before else '-'} to {cell(after, 'leaf_area', 1, 0) if after else '-'} |"
        )

doc = open(sys.argv[2]).read()
head, rest = doc.split("<!-- measured -->", 1)
_, tail = rest.split("<!-- /measured -->", 1)
open(sys.argv[2], "w").write(head + "<!-- measured -->\n" + "\n".join(lines) + "\n<!-- /measured -->" + tail)
print("\n".join(lines))
