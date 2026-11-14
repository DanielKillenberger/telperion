"""fn-341 R2: the paper arithmetic on stated branches of the oak at 80,
seed 1001 (raw/r1-oregon-white-oak-80.log). Every input is printed with
its origin; nothing is fitted.

Run: python3 -I r2.py
"""
import math

G = 9.81

# The stated branches, measured (r1 log, oak 80, seed 1001).
TRUNK = dict(dbh=1.1016, height=17.368)
FORK = dict(below=1.0958, limbs=[0.7084, 0.5268], exponent=2.6)  # d at the 4.72 m fork
LIMB = dict(d=0.5268, length=12.73, out=7.96, up=9.10, base_from_vertical=25.6)


def posture():
    """Alméras and Fournier 2009, eq. 39: the steepest lean a stem holds,
    sin(phi_max) = E dalpha f (m+1)(2n+1) D / (12 g L H^2). Their Fig. 8
    mean parameters (15 tropical angiosperms, Jaouen et al. 2007), as
    fn-338's R1-NOTE read them; no oak or beech value exists in the paper."""
    E, da, f, m, n, L = 1.23e10, 0.0017, 1.54, 1.62, 0.66, 1340.0
    k = E * da * f * (m + 1) * (2 * n + 1) / (12 * G * L)
    print(f"posture: E dalpha f (m+1)(2n+1) / (12 g L) = {k:.4g} m")
    check = k * 0.20 / 30.0 ** 2
    print(f"  the source's own check, D 0.20 m at H 30 m: sin phi_max {check:.3f} "
          f"({math.degrees(math.asin(min(check, 1))):.1f} deg; Fig. 8 says 'exceeds 15 deg')")
    for name, d, h in [("limb, as grown", LIMB["d"], LIMB["length"]),
                       ("limb, half the girth", LIMB["d"] / 2, LIMB["length"]),
                       ("trunk", TRUNK["dbh"], TRUNK["height"])]:
        s = k * d / h ** 2
        lean = math.degrees(math.asin(min(s, 1.0)))
        print(f"  {name}: D {d:.3f} m, H {h:.2f} m, sin phi_max {s:.2f} -> holds up to {lean:.0f} deg from vertical"
              + (" (any lean)" if s >= 1 else ""))
    chord = math.degrees(math.atan2(LIMB["out"], LIMB["up"]))
    print(f"  the limb's chord leans {chord:.0f} deg from vertical; its base {LIMB['base_from_vertical']} deg")
    # The length at which a branch of diameter d stops holding a horizontal lean.
    for d in (0.05, 0.10, 0.30):
        print(f"  a branch of {d:.2f} m holds horizontal while shorter than {math.sqrt(k * d):.1f} m")


def vigour():
    """Borchert-Honda as Pałubicki et al. 2009 state it, at the oak's fork,
    each wood's light taken in proportion to its section (r^2), a stand-in:
    the probe read no light per subtree."""
    e = FORK["exponent"]
    rest = (FORK["below"] / 2) ** e - sum((d / 2) ** e for d in FORK["limbs"])
    carrier = 2 * rest ** (1 / e)
    qm = carrier ** 2
    ql = sum(d ** 2 for d in FORK["limbs"])
    print(f"vigour: the fork's carrier on, from the pipe sum at exponent {e}: d {carrier:.3f} m")
    for lam in (0.45, 0.5, 0.55):
        vm = lam * qm / (lam * qm + (1 - lam) * ql)
        print(f"  lambda {lam}: carrier's share {vm:.3f}, the two limbs' {1 - vm:.3f} (light share {qm / (qm + ql):.3f})")


def girth():
    """The pipe model per leaf (Shinozaki et al. 1964): sapwood at breast
    height from the leaf above over a measured leaf-to-sapwood ratio,
    Q. garryana 0.23 m2/cm2 at 40 yr and 10 m, 0.37 at 250 yr and 25 m
    (Phillips et al. 2003, Table 1). The cycle's leaf on seed 1001 is
    fn-340's READ line (raw/r4-read-oregon-white-oak-*.log there)."""
    for age, leaf, dbh in [(40, 121.6, 0.578), (80, 442.6, 1.102), (120, 637.2, 1.427)]:
        model = math.pi * (dbh * 100 / 2) ** 2
        for ratio in (0.23, 0.37):
            sap = leaf / ratio
            d = 2 * math.sqrt(sap / math.pi)
            print(f"girth: age {age}, leaf {leaf} m2 / {ratio} m2 cm-2 = sapwood {sap:.0f} cm2, "
                  f"a sapwood-only DBH {d:.0f} cm; the model's section {model:.0f} cm2 ({model / sap:.1f} times)")
    for cat, area in [("good open 80/4 at 80", math.pi * 32 ** 2)]:
        print(f"girth: White {cat}: section {area:.0f} cm2; the model's at 80 over it "
              f"{math.pi * 55.1 ** 2 / area:.2f}")


if __name__ == "__main__":
    posture()
    vigour()
    girth()
