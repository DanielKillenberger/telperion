"""fn-343: the host's sharing rule worked on the logged walk, before any code.
At each walked node, every lateral of section A_l = r^2 on a bearer whose own
wood past the node has section A_b keeps the angle theta between them (its
heading against the segment below) and shares it:
  linear (host):  x_l = theta * A_b / (A_b + A_l),  x_b = theta - x_l
  sine (Murray 1927 eq. 3 at a fixed total angle): A_l sin x_l = A_b sin x_b
The lateral's subtree turns back toward the line below by theta - x_l; the
bearer's own wood turns x_b away from it, in the plane of the lateral and the
line below (several laterals' turns summed as rotation vectors). The walked
joint's new turn is the angle between the segment below and its taken wood,
so turned. Joints >= 10 deg per m on walked wood 1 cm or more, as project.py.
Run: python3 -I share.py <fork log> [...]"""
import math
import sys
from collections import Counter


def sub(a, b): return [a[i] - b[i] for i in range(3)]
def dot(a, b): return sum(a[i] * b[i] for i in range(3))
def cross(a, b): return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
def norm(a): return math.sqrt(dot(a, a))
def unit(a):
    n = norm(a)
    return [x / n for x in a] if n > 1e-12 else None
def ang(a, b):
    c = dot(a, b) / max(norm(a) * norm(b), 1e-24)
    return math.degrees(math.acos(max(-1.0, min(1.0, c))))
def rot(v, w):
    """v turned by the rotation vector w (axis times radians), Rodrigues."""
    t = norm(w)
    if t < 1e-12:
        return v
    k = [x / t for x in w]
    c, s = math.cos(t), math.sin(t)
    kv = cross(k, v)
    return [v[i] * c + kv[i] * s + k[i] * dot(k, v) * (1 - c) for i in range(3)]


def share(theta, ab, al, form):
    if ab + al <= 0:
        return theta, 0.0
    if form == "linear":
        xl = theta * ab / (ab + al)
    else:
        xl = math.atan2(ab * math.sin(theta), al + ab * math.cos(theta))
    return xl, theta - xl


def parse(tok):
    star = tok.startswith("*")
    kind, r, sv, hv = tok.lstrip("*").split(":")
    return star, kind, float(r), [float(x) for x in sv.split(",")], [float(x) for x in hv.split(",")]


def run(path, form):
    metres, today, rule = 0.0, Counter(), Counter()
    worse = Counter()
    pending = None
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "SEG":
            if float(w[10].split("=")[1]) >= 0.01:
                x = [float(v) for v in w[4:10]]
                metres += norm(sub(x[:3], x[3:]))
        elif w[0] == "FORK":
            pending = w
        elif w[0] == "JOINT":
            d = dict(t.split("=", 1) for t in w[4:] if "=" in t)
            f, pending = pending, None
            if float(d["r"]) < 0.01:
                continue
            below = [float(x) for x in f[4].split("=")[1].split(",")]
            woods = [parse(t) for t in f[5:]]
            own = [x for x in woods if x[1] in ("own", "continuation")]
            ab = max((x[2] for x in own), default=0.0) ** 2
            bear = [0.0, 0.0, 0.0]
            taken_new = None
            for star, kind, r, sv, hv in woods:
                if kind != "lateral":
                    continue
                n = unit(cross(below, hv))
                if n is None:
                    continue
                theta = math.radians(ang(below, hv))
                xl, xb = share(theta, ab, r * r, form)
                if star:
                    taken_new = rot(sv, [-(theta - xl) * c for c in n])
                bear = [bear[i] - xb * n[i] for i in range(3)]
            taken = next(x for x in woods if x[0])
            if taken_new is None:
                taken_new = rot(taken[3], bear) if taken[1] in ("own", "continuation") else taken[3]
            t0 = float(d["turn"])
            t1 = ang(below, taken_new)
            limb = d["pa"] in ("6", "7")
            cls = "take-over" if d["cause"] in ("fork", "lateral_at_end") else (
                "bearer runs on" if taken[1] == "own" else d["cause"])
            for scope in (("all",) + (("limb+bough",) if limb else ())):
                today[(scope, cls)] += t0 >= 10
                rule[(scope, cls)] += t1 >= 10
                if t0 < 10 <= t1:
                    worse[(scope, cls)] += 1
    return metres, today, rule, worse


if __name__ == "__main__":
    for path in sys.argv[1:]:
        for form in ("linear", "sine"):
            metres, today, rule, worse = run(path, form)
            print(f"\n### {path.split('/')[-1]}, {form}: {metres:.0f} m walked wood >= 1 cm")
            print("| scope | class | today >= 10 deg | rule >= 10 deg | made worse (today < 10, rule >= 10) |")
            print("|---|---|---|---|---|")
            for scope in ("all", "limb+bough"):
                keys = sorted({k[1] for k in today if k[0] == scope} | {k[1] for k in rule if k[0] == scope})
                for c in keys:
                    print(f"| {scope} | {c} | {today[(scope, c)]} | {rule[(scope, c)]} | {worse[(scope, c)]} |")
                t = sum(v for k, v in today.items() if k[0] == scope)
                r = sum(v for k, v in rule.items() if k[0] == scope)
                wv = sum(v for k, v in worse.items() if k[0] == scope)
                print(f"| {scope} | **total** | {t} ({t / metres:.3f}/m) | {r} ({r / metres:.3f}/m, x{r / max(t, 1):.2f}) | {wv} |")
