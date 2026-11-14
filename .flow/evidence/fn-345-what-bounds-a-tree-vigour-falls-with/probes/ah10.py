"""fn-345 R1: A beside H every 10 years to 120 from fn-344's 300-year grow
logs on seed 1001 (YEAR: rough-layout height, held, tips; BAL: spread).
Usage: python3 -I ah10.py <A log> <H log>
"""
import re, sys


def read(path):
    rows = {}
    for line in open(path):
        m = re.match(r"YEAR (\d+) held=(\d+) height=(\S+) stem_top=\S+ apexes=(\d+)", line)
        if m:
            rows.setdefault(int(m[1]), {}).update(held=int(m[2]), h=float(m[3]), tips=int(m[4]))
        m = re.match(r"BAL (\d+) .*spread_x=(\S+) spread_y=(\S+) cells_m2=(\d+)", line)
        if m:
            rows.setdefault(int(m[1]), {}).update(sx=float(m[2]), sy=float(m[3]), cells=int(m[4]))
    return rows


a, h = read(sys.argv[1]), read(sys.argv[2])
print("| year | A height m | H height m | H/A height | A spread m | H spread m | H/A held |")
print("|---|---|---|---|---|---|---|")
for y in range(9, 130, 10):
    ra, rh = a[y], h[y]
    print(f"| {y} | {ra['h']:.1f} | {rh['h']:.1f} | {rh['h']/ra['h']:.2f} | "
          f"{ra['sx']:.1f} × {ra['sy']:.1f} | {rh['sx']:.1f} × {rh['sy']:.1f} | {rh['held']/ra['held']:.2f} |")
