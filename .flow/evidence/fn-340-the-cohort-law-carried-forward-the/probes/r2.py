"""fn-340 R2: the vigour split (allocation.rs, `split`) worked on paper for one
oak branch unit, with the cohorts absent and with them standing as the
cycle's buds. Stdlib only; every input is stated below.

Run: python3 -I r2.py
"""
LAM = 0.45          # oak apical control, every age (oak.rs `state`)
L_CELL = 0.26       # cohort cell light, the oak's median (fn-335, R1-LAW.md)
L_APEX = 0.5        # the branch apex's own light (stated, not measured)
NODES = 2.5         # medial nodes a branch unit lays (zone 2..3, mean)
P_LAT = 0.95        # short 0.6 + twig 0.35 per medial node
# Living cohort apexes per medial node, by years since the node was laid:
# year 0 the buds just made (lit at presence 1 each), years 1.. the law's
# units grown that year at 0.26 (law.rs test, from law.py).
LIVING = [P_LAT, 0.724, 1.139, 1.366, 1.354, 1.253, 0.939, 0.481, 0.186]


def split_up(q_beyond, laterals, lam):
    """Vigour fraction passing a run of lateral nodes (base to tip, each
    lateral's light in `laterals`) to the axis beyond, whose light is
    `q_beyond`: allocation.rs's v_main = v lam Q_m / (lam Q_m + (1-lam) Q_l)."""
    frac, above = 1.0, q_beyond
    out = []
    for q in reversed(laterals):            # tip first, so Q_m accumulates
        out.append((q, above))
        above += q
    for q, qm in reversed(out):             # base first: the order v flows
        frac *= lam * qm / (lam * qm + (1 - lam) * q)
    return frac


def unit_nodes(age):
    """The lit presence of each cohort on a unit laid `age` years ago, by node
    (whole nodes plus a half node for the mean of 2.5)."""
    q = LIVING[age] * L_CELL if age > 0 else LIVING[0] * 1.0 * L_CELL
    return [q, q, 0.5 * q]


def main():
    print("One oak branch unit laid 3 years ago, its axis beyond it three younger units and the apex.\n")
    unit = unit_nodes(3)
    beyond_cohorts = sum(sum(unit_nodes(a)) for a in (0, 1, 2))
    for name, qb in [("cycle (cohorts as buds)", L_APEX + beyond_cohorts),
                     ("probe (cohorts absent)", L_APEX)]:
        lat = unit if name.startswith("cycle") else [0.0] * len(unit)
        f = split_up(qb, lat, LAM)
        print(f"- {name}: Q beyond = {qb:.3f}; vigour passing the unit = {f:.4f} of what enters it")
    f_cyc = split_up(L_APEX + beyond_cohorts, unit, LAM)
    f_unb = split_up(L_APEX + beyond_cohorts, unit, 0.5)
    print(f"- unbiased split (lam 0.5) on the cycle's lights: {f_unb:.4f}; the cycle's lam 0.45 over it: {f_cyc / f_unb:.4f}")
    # The apex itself, of what enters the unit's base: every cohort node on
    # the way, and at the tip its own light against nothing else.
    path = unit + [x for a in (2, 1, 0) for x in unit_nodes(a)]
    for lam in (LAM, 0.5):
        f = split_up(L_APEX, path, lam)
        print(f"- apex's share of the vigour entering the unit, cohorts as buds, lam {lam}: {f:.4f} "
              f"(its light share {L_APEX / (L_APEX + sum(path)):.4f}); cohorts absent: 1.0")
    print("\nPer cohort node, the cycle's factor on the vigour passing it, against an unbiased split:")
    print("(1 + x) / (1 + x (1 - lam) / lam), x = Q_lateral / Q_beyond\n")
    for x in (0.05, 0.1, 0.15, 0.3, 1.0):
        print(f"- x {x}: {(1 + x) / (1 + x * (1 - LAM) / LAM):.4f}")
    # Two branch apexes of one group (ψ 1.0): their size ratio is their
    # vigour-per-presence ratio. A young branch (one unit) against an old
    # one (four units), both lit at L_APEX.
    young = split_up(L_APEX, [x for a in (0,) for x in unit_nodes(a)], LAM) * L_APEX / L_APEX
    old = split_up(L_APEX, path, LAM)
    yl = L_APEX / (L_APEX + sum(unit_nodes(0)))
    ol = L_APEX / (L_APEX + sum(path))
    print("\nTwo branch apexes at the same light, entering vigour in proportion to their subtree's light")
    print("(the unbiased share below them), one on a young branch (one unit), one on an old (four units):")
    print(f"- cycle: young apex keeps {young:.4f} of its branch's vigour, old {old:.4f}; per unit of its own light "
          f"against the unbiased {yl:.4f} and {ol:.4f}: {young / yl:.4f} and {old / ol:.4f}")
    print(f"- the old apex's size over the young's, psi 1: cycle {(old / ol) / (young / yl):.4f}, probe 1.0000")


if __name__ == "__main__":
    main()
