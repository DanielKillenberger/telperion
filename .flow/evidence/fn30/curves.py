#!/usr/bin/env python3
"""fn-30 reference curves: composed open-grown height and trunk-diameter
references by age for the oak (Quercus garryana) and the spruce (Picea abies),
and the Chapman-Richards fit of the generator's growth traits against them.

Source tables and coefficients below are transcribed from checksummed files
named in .flow/evidence/fn30/REPORT.md. Anchors, interpolation, extrapolation
and site-class selection are the composition choices documented there. Usage:
  curves.py [--oak-dbh <m>] [--spruce-dbh <m>]   (the generator's measured mature trunk DBH)
Prints the composed tables, the fit and the three-age comparison as JSON.
"""
import json
import math
import sys

# --- ERTRAG (E1): Bavarian yield-table extracts, stand-grown mean height of the
# remaining stand by age and height site class, moderate thinning. -------------
JUETTNER_OAK_HEIGHT = {  # Jüttner 1955, oak (Quercus robur/petraea): age -> (I, II, III) m
    20: (7.8, None, None), 30: (12.6, 9.0, 5.7), 40: (15.6, 12.0, 8.8),
    50: (18.4, 14.6, 11.5), 60: (20.5, 16.9, 13.7), 70: (22.3, 18.8, 15.5),
    80: (23.8, 20.3, 16.9), 90: (25.0, 21.6, 18.1), 100: (26.0, 22.8, 19.4),
    110: (27.1, 23.8, 20.6), 120: (28.1, 24.8, 21.8), 130: (29.1, 25.8, 22.7),
    140: (30.0, 26.8, 23.5), 150: (30.9, 27.7, 24.1), 160: (31.7, 28.3, 24.5),
    170: (32.6, 28.9, 24.9), 180: (33.4, 29.4, 25.3), 190: (33.8, 29.8, 25.6),
    200: (34.2, 30.2, 25.9),
}
JUETTNER_OAK_DG_I = {  # Jüttner 1955, oak site class I, mean diameter DG of the remaining stand, cm
    30: 7.2, 40: 10.4, 50: 13.8, 60: 17.0, 70: 20.5, 80: 24.0, 90: 27.3, 100: 30.7,
    110: 34.1, 120: 37.5, 130: 41.0, 140: 44.5, 150: 47.7, 160: 50.8, 170: 54.0,
    180: 57.1, 190: 60.3, 200: 63.2,
}
WIEDEMANN_SPRUCE_HEIGHT = {  # Wiedemann 1936/42, spruce: age -> (I, II, III, IV, V) m
    20: (7.1, 5.1, 3.9, None, None), 30: (11.5, 8.6, 6.2, 4.2, None),
    40: (16.6, 12.8, 9.3, 6.9, 4.5), 50: (21.2, 16.9, 13.1, 9.8, 6.8),
    60: (24.7, 20.5, 16.2, 12.7, 9.3), 70: (27.4, 23.3, 18.9, 15.2, 11.7),
    80: (29.7, 25.6, 21.2, 17.3, 13.8), 90: (31.6, 27.6, 23.2, 19.2, 15.7),
    100: (33.3, 29.3, 25.0, 21.0, 17.2), 110: (34.8, 30.8, 26.7, 22.6, None),
    120: (35.9, 32.1, 28.2, 24.0, None),
}
WIEDEMANN_SPRUCE_DG = {  # Wiedemann 1936/42, spruce, DG of the remaining stand, cm: age -> (I, III)
    20: (7.5, 4.6), 30: (11.5, 7.5), 40: (15.5, 10.3), 50: (19.3, 12.8), 60: (23.0, 15.3),
    70: (26.9, 18.1), 80: (30.7, 20.7), 90: (34.2, 23.3), 100: (37.6, 25.9),
    110: (40.9, 28.4), 120: (44.3, 31.2),
}
# --- GOULD (G1): Gould, Harrington & Devine 2011, Appendix, FVS large-tree
# diameter growth of Oregon white oak (English units, 10-year change in squared
# DBH), the explicit open-grown case being BA = BAL = 0; worked example in the
# paper: 2.8 cm per decade at 40 cm DBH with Douglas-fir SI = 35 m. ------------
GOULD = dict(intercept=-1.33299, ln_dbh=1.66609, dbh2=-0.00154, bal=-0.00326, ba=-0.00204, ln_si=0.14995)
GOULD_SI_FT = 35.0 / 0.3048
# --- VOSPERNIK (V1): Vospernik, Monserud & Sterba 2010, open access. The
# diameter of an open-grown tree is about twice that of a mean stem at maximum
# density (Sterba 1975, confirmed by Lässig 1991); Lässig's reference open-grown
# spruce is 91 cm DBH at age 100 on good sites; five 300-year open-grown spruce
# on a poor site averaged 81 cm DBH and 23 m height. ----------------------------
OPEN_GROWN_OVER_STAND_DG = 2.0
# --- SILVICS (S1): Stein 1990. Mature Oregon white oak 15-27 m and 60-100 cm
# DBH; forest-grown 95-135 years to 24 m and 48 cm; 6-8 rings per cm common. ----
SILVICS_OAK = dict(mature_height_m=(15.0, 27.0), mature_dbh_m=(0.60, 1.00),
                   forest_pair=dict(years=(95, 135), height_m=24.0, dbh_m=0.48))


def gould_dds_in2(dbh_in, ba=0.0, bal=0.0, si_ft=GOULD_SI_FT):
    g = GOULD
    return math.exp(g["intercept"] + g["ln_dbh"] * math.log(dbh_in) + g["dbh2"] * dbh_in**2
                    + g["bal"] * bal + g["ba"] * ba + g["ln_si"] * math.log(si_ft))


def oak_open_grown_dbh(start_age, start_dbh_cm, end_age):
    """Open-grown Gould integration (BA = BAL = 0), yearly steps of DDS/10, from an anchor."""
    d_in = start_dbh_cm / 2.54
    out = {start_age: start_dbh_cm}
    for age in range(start_age + 1, end_age + 1):
        d_in = math.sqrt(d_in**2 + gould_dds_in2(d_in) / 10.0)
        out[age] = d_in * 2.54
    return out


def interp(table, age, col):
    ages = sorted(a for a in table if table[a][col] is not None)
    val = lambda a: table[a][col]
    if age <= ages[0]:
        return val(ages[0]) * age / ages[0]  # a seedling has no height; the first row anchors it
    if age >= ages[-1]:
        a0, a1 = ages[-2], ages[-1]
        return val(a1) + (val(a1) - val(a0)) * (age - a1) / (a1 - a0)
    for a0, a1 in zip(ages, ages[1:]):
        if a0 <= age <= a1:
            return val(a0) + (val(a1) - val(a0)) * (age - a0) / (a1 - a0)


def age_at(fn, target):
    lo, hi = 1.0, 400.0
    for _ in range(60):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if fn(mid) < target else (lo, mid)
    return (lo + hi) / 2


# --- The generator's growth rule (transcribed from crates/telperion-core/src/growth.rs) --
LIFETIME_UNITS = 250_000.0


def fraction(rate, shape, year):
    f = (1.0 - math.exp(-rate * year)) ** shape
    return 1.0 if f >= 1.0 - 0.5 / LIFETIME_UNITS else f


def mature_age(rate, shape):
    lo, hi = 0, 1_000_000
    while lo < hi:
        mid = (lo + hi) // 2
        lo, hi = (lo, mid) if fraction(rate, shape, mid) == 1.0 else (mid + 1, hi)
    return lo


def fit(height_ref, ages, envelope):
    """Grid search of rate (0.001 step) and shape (0.1 step) minimising the summed
    squared log error of envelope * fraction(age) against the reference height."""
    best = None
    for shape_i in range(10, 81):
        shape = shape_i / 10
        for rate_i in range(1, 400):
            rate = rate_i / 1000
            err = sum(math.log(max(envelope * fraction(rate, shape, a), 1e-3) / height_ref(a)) ** 2 for a in ages)
            if best is None or err < best[0]:
                best = (err, rate, shape)
    return best[1], best[2]


def arg(name, default):
    return float(sys.argv[sys.argv.index(name) + 1]) if name in sys.argv else default


def main():
    oak_dbh, spruce_dbh = arg("--oak-dbh", 0.836), arg("--spruce-dbh", 0.45)
    oak_d_table = oak_open_grown_dbh(30, JUETTNER_OAK_DG_I[30], 600)
    species = {
        "oregon-white-oak": dict(
            envelope=24.0, mature_dbh=oak_dbh,
            # height: Jüttner site class II, the class Silvics' forest-grown pair (24 m at 95-135 years) lands on
            h=lambda a: interp(JUETTNER_OAK_HEIGHT, a, 1),
            # diameter: Gould open-grown integration anchored on Jüttner site I DG at age 30 (7.2 cm)
            d=lambda a: (oak_d_table[min(600, round(a))] if a >= 30 else JUETTNER_OAK_DG_I[30] * a / 30) / 100.0,
            sensitivity={"I": lambda a: interp(JUETTNER_OAK_HEIGHT, a, 0), "III": lambda a: interp(JUETTNER_OAK_HEIGHT, a, 2)}),
        "norway-spruce": dict(
            envelope=15.0, mature_dbh=spruce_dbh,
            # height: Wiedemann site class I, paired with its DG column (III is also tabulated)
            h=lambda a: interp(WIEDEMANN_SPRUCE_HEIGHT, a, 0),
            # diameter: twice the stand DG (Sterba 1975 via Vospernik 2010); no age-indexed open-grown allometry for Picea abies was found
            d=lambda a: OPEN_GROWN_OVER_STAND_DG * interp(WIEDEMANN_SPRUCE_DG, a, 0) / 100.0,
            sensitivity={"II": lambda a: interp(WIEDEMANN_SPRUCE_HEIGHT, a, 1), "III": lambda a: interp(WIEDEMANN_SPRUCE_HEIGHT, a, 2)}),
    }
    out = {}
    for name, s in species.items():
        env = s["envelope"]
        young, middle, envelope_age = (age_at(s["h"], env * k) for k in (1 / 3, 2 / 3, 1.0))
        ages = [young, middle, envelope_age]
        rate, shape = fit(s["h"], ages, env)
        mature = mature_age(rate, shape)
        rows = []
        for label, a in zip(("young", "middle", "envelope", "derived-mature"), ages + [mature]):
            f = fraction(rate, shape, a)
            mh, md = env * f, s["mature_dbh"] * f
            rows.append(dict(point=label, age=round(a, 1), ref_height_m=round(s["h"](a), 2), ref_dbh_m=round(s["d"](a), 3),
                             ref_h_over_d=round(s["h"](a) / s["d"](a), 1), model_height_m=round(mh, 2),
                             model_dbh_m=round(md, 3), height_dev_pct=round(100 * (mh / s["h"](a) - 1), 1),
                             dbh_dev_pct=round(100 * (md / s["d"](a) - 1), 1)))
        out[name] = dict(envelope_height_m=env, rate=rate, shape=shape, derived_mature_age=mature,
                         envelope_age_by_site_class={k: round(age_at(fn, env), 1) for k, fn in s["sensitivity"].items()},
                         rows=rows)
    d = 40 / 2.54
    out["gould_check_cm_per_decade_at_40cm"] = round((math.sqrt(d**2 + gould_dds_in2(d)) - d) * 2.54, 2)
    out["spruce_check_open_grown_dbh_at_100_m"] = dict(composed=round(species["norway-spruce"]["d"](100), 3), laessig_1991=0.91)
    json.dump(out, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
