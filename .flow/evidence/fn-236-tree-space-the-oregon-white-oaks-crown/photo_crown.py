"""fn-236: O-WHOLE's crown read by code, in oak_measure.rs's terms.

    convert raw/ref/o-whole.jpg -colorspace Gray -compress none pgm:- | python3 photo_crown.py

The tree is the pixels darker than 110 of 255 inside the box x 100..1040,
y 0..735 (the tree, its trunk to the ground line), less the hills right of
x 960 below y 560. Crown pixels are those above y 650, the lowest foliage
(the right lobe), read from the thresholded mask: below it the fences and
sheds are as dark as the tree. Reported: height (ground line to
the top), crown width, width over height, crown base over height, and, on a
grid of H/60, fill (occupied cells over each row outline's cells) and solid
(the row outlines' area over their convex hull's).
"""
import sys

tokens = sys.stdin.read().split()
assert tokens[0] == "P2"
w, h, _ = int(tokens[1]), int(tokens[2]), int(tokens[3])
px = list(map(int, tokens[4:]))
dark = lambda x, y: px[y * w + x] < 110


def tree(x, y):
    if not (100 <= x < 1040 and y < 735):
        return False
    if x > 960 and y > 560:
        return False
    return dark(x, y)


rows = {}
for y in range(735):
    xs = [x for x in range(100, 1040) if tree(x, y)]
    if xs:
        rows[y] = (min(xs), max(xs), len(xs))
top = min(rows)
ground = 735
base = 650
height = ground - top
crown = {y: r for y, r in rows.items() if y <= base}
width = max(r[1] for r in crown.values()) - min(r[0] for r in crown.values())
cell = height / 60.0
cells = set()
for y in crown:
    for x in range(100, 1040):
        if tree(x, y):
            cells.add((int(x // cell), int(y // cell)))
outline_rows = {}
for u, v in cells:
    a, b = outline_rows.get(v, (u, u))
    outline_rows[v] = (min(a, u), max(b, u))
outline = sum(b - a + 1 for a, b in outline_rows.values())
pts = sorted({p for v, (a, b) in outline_rows.items() for p in ((a, v), (b + 1, v), (a, v + 1), (b + 1, v + 1))})


def cross(o, a, b):
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


hull = []
for seq in (pts, pts[::-1]):
    start = len(hull)
    for p in seq:
        while len(hull) >= start + 2 and cross(hull[-2], hull[-1], p) <= 0:
            hull.pop()
        hull.append(p)
    hull.pop()
area = abs(sum(hull[i][0] * hull[(i + 1) % len(hull)][1] - hull[(i + 1) % len(hull)][0] * hull[i][1] for i in range(len(hull)))) / 2
print(
    f"height_px {height} width_px {width} wh {width / height:.2f} "
    f"base/H {(ground - base) / height:.2f} fill {len(cells) / outline:.2f} solid {outline / area:.2f}"
)
