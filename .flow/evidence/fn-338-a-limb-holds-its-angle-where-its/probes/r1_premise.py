"""fn-338 R1: the arithmetic behind R1-NOTE.md, from Almeras and Fournier 2009.

Run: python3 -I r1_premise.py
"""
import math

# 1. Table 1 (poplar) reproduced: eq. 12 with dB/dD = 1.37e7 D^3.96, and
#    eq. 19 with e_r = delta-alpha read as microstrain.
rows = [  # D (m), E (MPa), delta-alpha, table dCw/dD, table dCm/dD
    (0.26, 8125, 841, 0.036, -0.052), (0.23, 6345, 609, 0.046, -0.046),
    (0.22, 5895, 417, 0.050, -0.036), (0.24, 7470, 918, 0.039, -0.064),
    (0.26, 7720, 861, 0.038, -0.051), (0.22, 7560, 1022, 0.039, -0.083),
    (0.23, 7360, 156, 0.040, -0.012), (0.27, 7580, 515, 0.039, -0.029),
    (0.26, 8715, 410, 0.034, -0.024),
]
print("tree  dCw/dD (ours, table)   dCm/dD (ours, table)   Pg (table)")
pg = []
for i, (d, e, da, tw, tm) in enumerate(rows, 1):
    cw = 1.37e7 * d**3.96 / (e * 1e6 * math.pi * d**4 / 64)
    cm = -4 * da * 1e-6 / d**2
    pg.append(-tm / tw)
    print(f"{i:>4}  {cw:.3f}  {tw:.3f}           {cm:.3f}  {tm:.3f}          {-tm / tw:.2f}")
print(f"Pg from the table: mean {sum(pg) / len(pg):.2f}, min {min(pg):.2f}, max {max(pg):.2f}")
print(f"Fig. 4 slope check: 2.76e6 * 4.96 = {2.76e6 * 4.96:.3g} (text: 1.37e7)")

# 2. Eq. 37 from eqs. 12, 19 and 30: P_g = e_r E pi D^2 / (16 dB/dD).
#    Symbolic check by numbers: both sides at arbitrary values.
E, Eb, da, f, L, phi, m, n, bH, D, H, g = 1e10, 1.1e10, 0.0017, 1.54, 1340, 0.4, 1.62, 0.66, 0.5, 0.2, 12, 9.8
er = da * f * Eb / E
dB = math.pi / 2 * g * L * math.sin(phi) * (1 + bH) / ((m + 1) * (2 * n + 1)) * H**2 * D
pg_ratio = (4 * er / D**2) / (dB / (E * math.pi * D**4 / 64))
pg_37 = Eb * da * f / (8 * g * L * math.sin(phi)) * (m + 1) * (2 * n + 1) / (1 + bH) * D / H**2
print(f"eq. 37 by its parts {pg_ratio:.6f}, as printed {pg_37:.6f}")
# Under M = a R^k the same P_g is (pi/2) E e_r R^3 / (k M), k = 2 (1 + bH).
B = math.pi / 4 * g * L * math.sin(phi) / ((m + 1) * (2 * n + 1)) * H**2 * D**2
k = 2 * (1 + bH)
print(f"(pi/2) E e_r R^3/(k M) = {math.pi / 2 * E * er * (D / 2)**3 / (k * B):.6f}")

# 3. The engine's elastic sag: curvature = sag M_e / r^4, M_e = r^2 L lever
#    (no density, no pi), so sag = 4 rho g / E.
for rho, e in [(800, 1e10), (1000, 1e10), (1050, 1.0e10)]:
    print(f"elastic sag at rho {rho}, E {e:.0e}: {4 * rho * g / e:.2e} per m; oak limb 3e-4 is {3e-4 / (4 * rho * g / e):.0f}x")

# 4. The premise against the source, on a power law M = R^k (one section's
#    life), in units where the wood's held moment M* = (pi/6) E e_r R^3 = R^3
#    (so M = M* at R = 1). Curvatures are in units of 4 / (pi E), i.e. the
#    settle's M / R^4 with sag folded out.
#    today:   M / R^4                             (no reaction, final moment)
#    premise: max(0, M - M*) / R^4                 (leftover, final radius)
#    source:  integral of max(0, dM - dM*) / R^4   (A&F eqs. 12 + 19, the
#             reaction at its maximum where needed, regulated to hold
#             otherwise: the clamp, A&F 4.3)
def source(k, R, steps=200000):
    total, r0 = 0.0, 1e-6
    h = (R - r0) / steps
    for i in range(steps):
        r = r0 + (i + 0.5) * h
        dm = k * r ** (k - 1) - 3 * r**2
        if dm > 0:
            total += dm / r**4 * h
    return total


print("k     R/R0   today    premise  source   source/premise")
for k in (4, 5, 6):
    rc = (3 / k) ** (1 / (k - 3))
    for R in (rc, 1.0, 1.5, 2.0, 4.0):
        t = R**k / R**4
        p = max(0.0, R**k - R**3) / R**4
        s = source(k, R)
        ratio = s / p if p > 0 else float("inf")
        print(f"{k}  {R:6.3f}  {t:7.3f}  {p:7.3f}  {s:7.3f}  {ratio:7.2f}")
    print(f"   crossover R_c/R0 = (3/k)^(1/(k-3)) = {rc:.3f}")

# 5. The share in the engine's units: M*_e = s R^3 with s = Eb da f / (6 rho g).
for name, da_, f_ in [("angio mean", 0.0017, 1.52), ("angio max", 0.0028, 2.22),
                      ("gymno mean", 0.0015, 1.03), ("gymno max", 0.0020, 1.25)]:
    for Eb_ in (1.0e10,):
        s = Eb_ * da_ * f_ / (6 * 1000 * g)
        print(f"{name}: e_r~{da_ * f_:.4f}, s = {s:.0f} m at Eb 10 GPa, rho 1000")
