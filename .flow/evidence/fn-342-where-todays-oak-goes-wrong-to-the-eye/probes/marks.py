"""fn-342 R2: one sheet per tree, a panel per cause: the walked wood (laterals
1 cm or more at their base, and the trunk) drawn from the whole still's side
(azimuth 115 degrees, no perspective), each joint of 10 deg or more of that
cause on wood 2 px or more wide in the whole still ringed in red. Writes an SVG; convert with rsvg-convert.

Run: python3 -I marks.py <turns log> <seed> <out.svg>
"""
import math
import sys

T = 10.0
AZ = math.radians(115.0)
PANELS = ["relay", "fork-kink", "fork-Y", "lateral_at_end"]
W, H = 900, 900


def kv(words):
    out = {}
    for w in words:
        if "=" in w:
            k, v = w.split("=", 1)
            try:
                out[k] = float(v)
            except ValueError:
                out[k] = v
    return out


def screen(x, y, z):
    # The engine (x, y, z up) is the pipeline's (x, z, -y); the hero camera
    # looks from azimuth 115 degrees.
    return (-math.cos(AZ) * x + math.sin(AZ) * y, z)


def main():
    log, seed, out = sys.argv[1], sys.argv[2], sys.argv[3]
    segs, trunk, marks = [], [], {p: [] for p in PANELS}
    age, height = "?", 1.0
    for line in open(log):
        w = line.split()
        if not w or len(w) < 2 or w[1] != seed:
            continue
        if w[0] == "TREE":
            age = w[2].split("=")[1]
            height = kv(w[2:])["height"]
        elif w[0] == "SEG":
            xs = [float(v) for v in w[4:10]]
            segs.append((w[2], screen(*xs[:3]), screen(*xs[3:]), kv(w[10:])["r"]))
        elif w[0] == "TRUNK":
            d = kv(w[3:])
            trunk.append((screen(d["x"], d["y"], d["z"]), d["r"]))
        elif w[0] == "JOINT":
            d = kv(w[4:])
            if d["turn"] < T:
                continue
            c = d["cause"]
            if c == "fork":
                c = "fork-kink" if d["alt"] < 0.7 else "fork-Y"
            if c in marks:
                marks[c].append(d)
    # Joint positions: a joint is at the tip of its path's segment k.
    tips = {}
    for line in open(log):
        w = line.split()
        if w and w[0] == "SEG" and w[1] == seed:
            xs = [float(v) for v in w[7:10]]
            tips[(w[2], w[3])] = screen(*xs)
    pts = [p for s in segs for p in s[1:3]] + [t[0] for t in trunk]
    x0, x1 = min(p[0] for p in pts), max(p[0] for p in pts)
    z0, z1 = min(p[1] for p in pts), max(p[1] for p in pts)
    k = 0.9 * min(W / (x1 - x0), H / (z1 - z0))
    def px(p):
        return (W / 2 + (p[0] - (x0 + x1) / 2) * k, H * 0.95 - (p[1] - z0) * k)
    body = []
    def wood():
        # One polyline per run of a path's segments in one width band.
        g, run, key = [], [], None
        def flush():
            if len(run) > 1:
                pts = " ".join(f"{x:.1f},{y:.1f}" for x, y in run)
                g.append(f'<polyline points="{pts}" stroke-width="{key[1]:.2f}"/>')
        for path, a, b, r in segs:
            width = round(max(0.4, 2 * r * k) * 2) / 2
            if (path, width) != key or not run or run[-1] != px(a):
                flush()
                run, key = [px(a)], (path, width)
            run.append(px(b))
        flush()
        tr = [px(a) for a, _ in trunk]
        for i in range(len(tr) - 1):
            g.append(f'<line x1="{tr[i][0]:.1f}" y1="{tr[i][1]:.1f}" x2="{tr[i+1][0]:.1f}" y2="{tr[i+1][1]:.1f}" stroke-width="{max(0.4, 2 * trunk[i][1] * k):.2f}"/>')
        return '<g fill="none" stroke="#5a4a3a" stroke-linecap="round" stroke-linejoin="round">' + "".join(g) + "</g>"
    trees = wood()
    # Joints per panel, by their path and index.
    for i, p in enumerate(PANELS):
        ox, oy = (i % 2) * W, (i // 2) * H
        ring = []
        for line in open(log):
            w = line.split()
            if not (w and w[0] == "JOINT" and w[1] == seed):
                continue
            d = kv(w[4:])
            if d["turn"] < T:
                continue
            c = d["cause"]
            if c == "fork":
                c = "fork-kink" if d["alt"] < 0.7 else "fork-Y"
            if c != p or (w[2], w[3]) not in tips:
                continue
            if 2 * d["r"] * 0.78 * 720 / height < 2:
                continue
            x, y = px(tips[(w[2], w[3])])
            ring.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{4 + d["turn"] / 10:.1f}"/>')
        body.append(f'<g transform="translate({ox},{oy})"><rect width="{W}" height="{H}" fill="#f4f1ea"/>{trees}'
                    f'<g fill="none" stroke="#d01010" stroke-width="2">{"".join(ring)}</g>'
                    f'<text x="16" y="34" font-size="26" font-family="sans-serif">oak {age}, seed {seed}: {p} ({len(ring)} joints of 10 deg or more)</text></g>')
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="{2 * W}" height="{2 * H}">' + "".join(body) + "</svg>"
    open(out, "w").write(svg)


if __name__ == "__main__":
    main()
