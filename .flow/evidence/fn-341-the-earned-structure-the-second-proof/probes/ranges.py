"""fn-341 R1: the published ranges at the probe's ages, from the checked
sources' own rules, and each model median's distance from them (a multiple
of the nearer end; 1 inside). No value is fitted: each line is the source's
table or equation evaluated.

Run: python3 -I ranges.py
"""
import math

# White 1998, FC Information Note 12, Table 1a: age / ring width (mm) when
# the mature state is reached; rings of even width before it, rings of the
# same area (the last core ring's) after it. Table 2's CAI is the core's
# area less the area one ring width in.
WHITE = {
    "oak": {"champion": (70, 5), "good open": (80, 4), "parkland": (100, 3.5)},
    "beech": {"champion": (80, 6), "good open": (70, 5), "parkland": (70, 4)},
}


def white_dbh(age, core_age, ring_mm):
    """Diameter at breast height (cm) at `age` by White's method run forward."""
    w = ring_mm / 10.0
    if age <= core_age:
        return 2.0 * w * age
    r = w * core_age
    cai = math.pi * (r * r - (r - w) ** 2)
    area = math.pi * r * r + cai * (age - core_age)
    return 2.0 * math.sqrt(area / math.pi)


def pretzsch_cd(d_cm):
    """Q. robur open-grown crown diameter (m): 2 exp(-1.4370 + 0.8157 ln d),
    Pretzsch et al. 2015, Table 2, the 95% quantile."""
    return 2.0 * math.exp(-1.4370 + 0.8157 * math.log(d_cm))


def gould_cap(age):
    """Q. garryana open-grown DBH (cm) if it grew at the maximum predicted
    rate, 1.27 in per 5 years (Gould et al. 2008), every year: an upper bound."""
    return 1.27 * 2.54 / 5.0 * age


def dist(x, lo, hi):
    return 1.0 if lo <= x <= hi else (x / hi if x > hi else x / lo)


# Model medians over seeds 1001, 1007, 1012 (r1.py; the 300 row per the log).
MODEL = {
    ("oak", 40): dict(dbh=65.3, spread=7.72, height=10.26),
    ("oak", 80): dict(dbh=149.8, spread=20.41, height=17.66),
    ("oak", 120): dict(dbh=149.9, spread=25.93, height=24.93),
    ("oak", 300): dict(dbh=120.4, spread=30.40, height=39.98),
    ("beech", 40): dict(dbh=28.9, spread=5.99, height=9.75),
    ("beech", 80): dict(dbh=131.2, spread=19.75, height=17.26),
}


# The probe's medians, G and P (R4; raw/r3-*-GPr4.log through r1.py).
PROBE = {
    ("oak", 40): dict(dbh=22.9, spread=7.65, height=10.35),
    ("oak", 80): dict(dbh=66.8, spread=19.16, height=18.01),
    ("oak", 120): dict(dbh=105.1, spread=23.58, height=26.20),
    ("oak", 300): dict(dbh=170.6, spread=24.88, height=51.93),
    ("beech", 40): dict(dbh=3.7, spread=5.99, height=9.75),
    ("beech", 80): dict(dbh=21.4, spread=19.75, height=17.26),
}


def main(model=MODEL):
    for (sp, age), m in model.items():
        if m["dbh"] is None:
            continue
        ds = {k: white_dbh(age, *v) for k, v in WHITE[sp].items()}
        lo, hi = min(ds.values()), max(ds.values())
        line = (f"{sp} {age}: White DBH {lo:.0f} to {hi:.0f} cm "
                f"({', '.join(f'{k} {v:.0f}' for k, v in ds.items())}); model {m['dbh']:.0f} cm, "
                f"distance {dist(m['dbh'], lo, hi):.2f}")
        if sp == "oak":
            cap = gould_cap(age)
            line += f"; Gould cap {cap:.0f} cm, model over cap {m['dbh'] / cap:.2f}"
            cd_lo, cd_hi = pretzsch_cd(lo), pretzsch_cd(hi)
            cd_model = pretzsch_cd(m["dbh"])
            line += (f"\n  crown for White's DBH {cd_lo:.1f} to {cd_hi:.1f} m, model spread {m['spread']:.1f} "
                     f"(distance {dist(m['spread'], cd_lo, cd_hi):.2f}); crown for the model's own DBH "
                     f"{cd_model:.1f} m, spread over it {m['spread'] / cd_model:.2f}")
        print(line)


if __name__ == "__main__":
    import sys
    main(PROBE if sys.argv[1:] == ["probe"] else MODEL)
