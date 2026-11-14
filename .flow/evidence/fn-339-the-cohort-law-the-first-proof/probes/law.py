"""fn-339 R1: the cohort law on paper, worked on the oak's, spruce's and
beech's own tables.

Tables transcribed by hand from crates/telperion-space/src/{oak,spruce,beech,
chain,tolerance}.rs at 7194a3efa, canonical form applied as
`Species::canonical` does. Light levels are fn-335's birth-light quintile cuts
(raw/read/<preset>.json, fit seeds 1001-1006, age 40). Nothing here reads the
crates; a transcription error is this script's, and every row it prints names
the setting it read.

Run: python3 -I law.py            (stdlib only, deterministic)
"""
import math
import random
import sys

# The shared reference axis (chain.rs).
(SEEDLING, SAPLING, TRUNK, LOW_FORK, LEADER, TOP_FORK, LIMB, BOUGH, SPRIG,
 BRANCH, BRANCHLET, SPUR, SHOOT, TWIG, SHORT) = range(15)
AGES = 15
NAMES = ["seedling", "sapling", "trunk", "low_fork", "leader", "top_fork",
         "limb", "bough", "sprig", "branch", "branchlet", "spur", "shoot",
         "twig", "short"]


def zone(lo, hi, buds=1, lat=(), dormant=None, delay=0.0, rate=0.0):
    p = [0.0] * AGES
    for pa, q in lat:
        p[pa] = q
    d = [0.0] * AGES
    for pa, q in (dormant or ()):
        d[pa] = q
    return dict(lo=lo, hi=hi, buds=buds, p=p, dormant=d, delay=delay, rate=rate)


def mean_nodes(z):
    """NodeLaw::Uniform::mean: node j stands with chance clamp((max-j)/(max-min+1))."""
    lo, hi = z["lo"], z["hi"]
    span = hi - lo + 1.0
    return sum(min(max((hi - j) / span, 0.0), 1.0) for j in range(math.ceil(hi)))


def state(**kw):
    base = dict(L=0.0, cont=1.0, v=1.0, zones=[zone(0, 0)], shed=math.inf,
                inode=0.04, abort=0.0, relay=0.0, relay_ended=0.0,
                relay_failed=0.0, phi=0.0, psi=0.0, upkeep=0.0, bh=0.0,
                tol=0.0, leaf=0.0, leaf_years=math.inf, cluster=0.0,
                pipe=0.005, exp=2.0)
    base.update(kw)
    return base


def canonical(states):
    """Species::canonical: an unused age carries its nearest used age's
    settings (earlier on a tie), lifespan 0, continuation 1, no laterals."""
    used = [k for k, s in enumerate(states) if s["L"] > 0]
    out = list(states)
    for k, s in enumerate(states):
        if s["L"] > 0:
            continue
        near = min(used, key=lambda u: (abs(u - k), u > k))
        c = dict(states[near])
        c["L"], c["cont"] = 0.0, 1.0
        c["zones"] = [dict(z, p=[0.0] * AGES, dormant=[0.0] * AGES) for z in c["zones"]]
        out[k] = c
    return out


def oak_unit(bare, below, top):
    return [zone(*bare), zone(2, 3, 1, below), zone(1, 1, top[0], top[1])]


def oak():
    # oak.rs `state`: shade_hazard 1, shade_size 1.5, upkeep .35, balance
    # hazard 2, tolerance 0, leaf .00181, leaf_years 1; Form::default pipe .005.
    common = dict(phi=1.0, psi=1.5, upkeep=0.35, bh=2.0, tol=0.0,
                  leaf=0.00181, leaf_years=1.0)
    s = [state(**common) for _ in range(AGES)]
    sleep = lambda z: dict(z, dormant=[0.1 if j == BRANCH else 0.0 for j in range(AGES)], delay=6.0, rate=0.04)
    s[TRUNK] = state(**common, L=12, cont=1, inode=0.045, pipe=0.009, exp=2.6,
                     zones=oak_unit((4, 5), [(SPRIG, .4)], (3, [(SPRIG, .6)])))
    s[LOW_FORK] = state(**common, L=2, cont=1, inode=0.045, pipe=0.009, exp=2.6,
                        zones=oak_unit((3, 4), [(SPRIG, .3)], (3, [(LIMB, .8)])))
    s[LEADER] = state(**common, L=6, cont=1, inode=0.05, pipe=0.009, exp=2.6,
                      zones=oak_unit((3, 4), [(SHORT, .25), (BRANCH, .4)], (3, [(LIMB, .08), (BOUGH, .1)])))
    s[LIMB] = state(**dict(common, psi=0.5), L=500, cont=0, shed=5, inode=0.035,
                    abort=0.15, relay=1, relay_ended=1, relay_failed=1,
                    pipe=0.009, exp=2.8,
                    zones=[sleep(z) for z in oak_unit((2, 3), [(SHORT, .4), (BRANCH, .45)], (3, [(LIMB, .007), (BOUGH, .08)]))])
    s[BOUGH] = state(**dict(common, psi=0.5), L=60, cont=0, v=0.999, shed=4,
                     inode=0.03, abort=0.1, relay=1, relay_ended=1, relay_failed=1,
                     pipe=0.004, exp=2.6,
                     zones=oak_unit((2, 3), [(SHORT, .5), (BRANCH, .3)], (3, [(BRANCH, .3)])))
    s[SPRIG] = state(**common, L=10, cont=1, v=0.95, shed=3, inode=0.031,
                     pipe=0.0012, exp=2.0,
                     zones=oak_unit((1, 2), [(SHORT, .3), (TWIG, .3)], (3, [(TWIG, .4), (BRANCH, .1)])))
    s[BRANCH] = state(**dict(common, psi=1.0), L=12, cont=1, v=0.98, shed=3,
                      inode=0.017, relay=1, relay_ended=1, relay_failed=1,
                      pipe=0.0009, exp=2.0,
                      zones=oak_unit((1, 2), [(SHORT, .6), (TWIG, .35)], (3, [(TWIG, .1), (BRANCH, .02)])))
    s[TWIG] = state(**dict(common, psi=1.0), L=5, cont=0, v=0.98, shed=2,
                    inode=0.012, pipe=0.0008, exp=2.0,
                    zones=oak_unit((1, 1), [(SHORT, .7)], (3, [(TWIG, 0.0), (SHORT, .3)])))
    s[SHOOT] = state(**dict(common, psi=1.0), L=4, cont=0, v=0.93, shed=2,
                     inode=0.015, pipe=0.0009, exp=2.0,
                     zones=oak_unit((1, 1), [(SHORT, .3)], (2, [(SHORT, .35)])))
    s[SHORT] = state(**dict(common, psi=1.0), L=3, cont=0, v=0.9, shed=2,
                     inode=0.006, pipe=0.0006, exp=2.0, zones=[zone(3, 5)])
    for k in (SEEDLING, SAPLING, TOP_FORK, BRANCHLET, SPUR):
        s[k]["L"] = 0.0
    return canonical(s)


# tolerance.rs: k = (5 - T) / (5 - 2.4); hazards times k, upkeep etc. the oak's.
def respond(t):
    k = (5.0 - t) / (5.0 - 2.4)
    return dict(phi=1.0 * k, bh=2.0 * k, upkeep=0.35, tol=0.0)


NEEDLE_AREA = (18.0 + 31.0) / 2.0 * 100.0 * 13.7e-6   # spruce.rs, m2 per m


def spruce():
    common = dict(respond(4.45), psi=0.0)
    s = [state(**common) for _ in range(AGES)]
    trunk_unit = lambda bare, med, whorl: [zone(*bare), zone(1, 2, 1, [(SPRIG, med)]), zone(1, 1, 5, [(SPRIG, whorl)])]
    sl = lambda z: dict(z, dormant=[0.45 if j == BRANCHLET else 0.0 for j in range(AGES)], delay=1.0, rate=0.3)
    s[SEEDLING] = state(**common, L=2, cont=1, inode=0.02, pipe=0.0025, exp=2.4, zones=trunk_unit((2, 3), 0, 0))
    s[SAPLING] = state(**common, L=8, cont=1, inode=0.028, pipe=0.0025, exp=2.4, zones=trunk_unit((4, 5), .25, .85))
    s[TRUNK] = state(**common, L=35, cont=1, inode=0.042, pipe=0.0025, exp=2.4, zones=trunk_unit((5, 6), .3, .85))
    s[LEADER] = state(**common, L=300, cont=0, inode=0.03, pipe=0.0025, exp=2.4, zones=trunk_unit((5, 6), .3, .85))
    bz = [sl(zone(1, 1, 1)), sl(zone(2, 2, 2, [(BRANCHLET, .7)])), sl(zone(1, 1, 2, [(BRANCHLET, .95)]))]
    s[BRANCH] = state(**common, L=300, cont=0, v=0.998, shed=3, inode=0.034, pipe=0.0008, exp=2.2, zones=bz)
    s[SPRIG] = state(**common, L=6, cont=1, v=0.998, shed=math.inf, inode=0.05, pipe=0.0008, exp=2.2, zones=bz)
    s[BRANCHLET] = state(**common, L=10, cont=1, v=0.99, shed=1, inode=0.016, pipe=0.0005, exp=2.0,
                         zones=[zone(0, 0), zone(2, 2, 2, [(SHOOT, .55)]), zone(1, 1, 3, [(SHOOT, .8)])])
    s[SPUR] = state(**common, L=40, cont=0, v=0.99, shed=1, inode=0.006, pipe=0.0005, exp=2.0,
                    zones=[zone(0, 0), zone(0, 0), zone(1, 1, 2, [(SHOOT, .6)])])
    s[SHOOT] = state(**common, L=2, cont=0, v=0.9, shed=3, inode=0.02, pipe=0.0004, exp=2.0, zones=[zone(2, 2)])
    for k in range(AGES):
        s[k]["leaf"] = NEEDLE_AREA * s[k]["inode"]
    return canonical(s)


def beech():
    common = dict(respond(4.56), psi=0.0, leaf=0.0015723, leaf_years=1.0)
    s = [state(**common) for _ in range(AGES)]

    def reiterate(L, top, v=0.995):
        return state(**common, L=L, cont=0, v=v, pipe=0.008, exp=2.6,
                     zones=[zone(2, 3), zone(2, 3, 1, [(SHORT, .3), (BRANCH, .27)]), zone(1, 1, 1, top)])

    def module(r, abort):
        return dict(r, abort=abort, relay=1, relay_ended=1, relay_failed=1)

    s[TRUNK] = state(**common, L=9, cont=1, inode=0.07, abort=0.5, relay=1, relay_ended=1, relay_failed=1,
                     pipe=0.008, exp=2.6,
                     zones=[zone(3, 4), zone(2, 3, 1, [(BRANCH, .35)]), zone(1, 1, 1, [(BRANCH, .8)])])
    s[LEADER] = dict(module(reiterate(40, [(LIMB, .3), (BOUGH, .2)]), 0.3), v=1.0, cont=1.0)
    s[TOP_FORK] = state(**common, L=2, cont=0, pipe=0.008, exp=2.6,
                        zones=[zone(2, 3), zone(2, 2, 1, [(LIMB, .3), (BOUGH, .5)]), zone(1, 1, 1, [(LIMB, .9)])])
    s[LIMB] = module(reiterate(70, [(BOUGH, .16)]), 0.3)
    s[BOUGH] = module(reiterate(30, [(SPRIG, .14)]), 0.35)
    s[SPRIG] = reiterate(25, [(BRANCH, .45)])
    s[BRANCH] = state(**common, L=10, cont=1, v=0.95, shed=1, inode=0.03, pipe=0.0009, exp=2.0,
                      zones=[zone(1, 2), zone(2, 3, 1, [(SHORT, .7)]), zone(1, 1, 1, [(BRANCH, .08)]),
                             zone(1, 2, 1, [(SHOOT, .65)])])
    s[SHOOT] = state(**common, L=5, cont=0, v=0.95, shed=1, inode=0.025, pipe=0.0009, exp=2.0,
                     zones=[zone(1, 1), zone(2, 3, 1, [(SHORT, .6)])])
    s[SHORT] = state(**common, L=4, cont=0, v=0.9, shed=1, inode=0.006, pipe=0.00095, exp=2.0,
                     cluster=4.0, zones=[zone(3, 5)])
    return canonical(s)


# ---------------------------------------------------------------- the rule

def lived(s, pa):
    reach = 1.0
    while pa < AGES:
        if s[pa]["L"] > 0:
            return pa, reach
        reach *= s[pa]["cont"]
        pa += 1
        if reach <= 0:
            return None
    return None


def unit_nodes(st):
    return sum(mean_nodes(z) for z in st["zones"])


def unit_max(st):
    return sum(z["hi"] for z in st["zones"])


def laterals_per_unit(st):
    out = [0.0] * AGES
    for z in st["zones"]:
        n = mean_nodes(z) * z["buds"]
        for j in range(AGES):
            out[j] += n * z["p"][j]
    return out


def reaches(s, relays):
    """fn-334's reach(j): sum over the lifespan of survival so far times the
    unit's mean length, plus continuation times reach(j+1). With `relays`,
    a failure or abortion that relays does not end the axis (a relay carries
    its time on), so a year's loss is (1-v)(1-relay_failed) + v a (1-relay)."""
    r = [0.0] * (AGES + 1)
    for j in reversed(range(AGES)):
        st = s[j]
        unit = unit_nodes(st) * st["inode"]
        own, t, alive = 0.0, 0.0, 1.0
        while t < st["L"]:
            share = min(st["L"] - t, 1.0)
            if relays:
                lose = (1 - st["v"]) * (1 - st["relay_failed"])
                alive *= 1 - lose
                own += share * alive * unit
                alive *= 1 - st["v"] * st["abort"] * (1 - st["relay"])
            else:
                own += share * st["v"] ** (t + 1) * unit
            t += 1
        r[j] = own + st["cont"] * r[j + 1]
    return r[:AGES]


def potential(s):
    """The farthest an axis of age j can run: every unit at its most nodes,
    no death, every continuation taken (and a relay's carried time)."""
    r = [0.0] * (AGES + 1)
    for j in reversed(range(AGES)):
        st = s[j]
        r[j] = math.ceil(st["L"]) * unit_max(st) * st["inode"] + (r[j + 1] if st["cont"] > 0 else 0.0)
    return r[:AGES]


def borne_ages(s, j):
    """Ages a lateral of age j's axis bears (its own and its continuations')."""
    out, k = set(), j
    while k is not None:
        for z in s[k]["zones"]:
            for i in range(AGES):
                if z["p"][i] > 0 or z["dormant"][i] > 0:
                    lv = lived(s, i)
                    if lv:
                        out.add(lv[0])
        nxt = lived(s, k + 1) if (s[k]["cont"] > 0 and k + 1 < AGES) else None
        k = nxt[0] if nxt else None
    return out


def extent(s, r, j, seen=()):
    """A subtree's reach: its axis's, plus the farthest of what it bears,
    placed at its tip (an upper bound on the straight-line reach of the
    expected or potential paths; None where it bears its own age)."""
    if j in seen:
        return math.inf
    kids = borne_ages(s, j)
    far = max((extent(s, r, i, seen + (j,)) for i in kids), default=0.0)
    return r[j] + far


def lateral_ages(s):
    out = set()
    for st in s:
        if st["L"] <= 0:
            continue
        for z in st["zones"]:
            for i in range(AGES):
                if z["p"][i] > 0 or z["dormant"][i] > 0:
                    lv = lived(s, i)
                    if lv:
                        out.add(lv[0])
    return sorted(out)


# ------------------------------------------------- the law, one bearing node

class Member:
    __slots__ = ("pa", "t", "born", "alive", "nodes", "kids", "end", "parent")

    def __init__(self, pa, born, parent):
        self.pa, self.t, self.born, self.alive = pa, 0.0, born, True
        self.nodes = []        # (year laid, pa) per phytomer
        self.kids = []
        self.end = None        # last year its apex grew
        self.parent = parent


def survival(st, light):
    """advance.rs: ln survival = ln viability x light^-phi."""
    if st["phi"] > 0 and light < 1:
        return st["v"] ** (light ** -st["phi"])
    return st["v"]


def draw_nodes(z, rnd):
    lo, hi = int(z["lo"]), int(z["hi"])
    return rnd.randint(lo, hi) if hi > 0 else 0


def grow(s, m, year, rnd, light):
    """One year of one living member: survival, its unit, its laterals,
    its move on or stop. Returns the members born (grow from next year)."""
    st = s[m.pa]
    if rnd.random() >= survival(st, light):
        m.alive = False
        return []
    born = []
    for z in st["zones"]:
        for _ in range(draw_nodes(z, rnd)):
            m.nodes.append((year, m.pa))
            for _slot in range(int(z["buds"])):
                u, acc = rnd.random(), 0.0
                for i in range(AGES):
                    acc += z["p"][i]
                    if u < acc:
                        lv = lived(s, i)
                        if lv:
                            k = Member(lv[0], year, m)
                            m.kids.append(k)
                            born.append(k)
                        break
    m.end = year
    m.t += 1
    if m.t >= st["L"]:
        nxt = lived(s, m.pa + 1) if m.pa + 1 < AGES else None
        if nxt and rnd.random() < st["cont"] * nxt[1]:
            m.pa, m.t = nxt[0], 0.0
        else:
            m.alive = False
    return born


def last_growth(m):
    ends = [m.end if m.end is not None else m.born]
    for k in m.kids:
        ends.append(last_growth(k))
    return max(ends)


def standing(s, m, year, keep_parent=True):
    """shed.rs: a lateral is dropped once its subtree has held no living apex
    for more than its delay, faded over the cycle past (whole delays: gone in
    year last + delay + 1). The delay read is the age it ended in."""
    if m.parent is not None and not standing(s, m.parent, year):
        return False
    if m.alive:
        return True
    return year <= last_growth(m) + s[m.pa]["shed"]


def bearer_node(s, bearer_pa, zone_index, years, light, seed, reps):
    """Every lateral one node of zone `zone_index` of a unit of `bearer_pa`
    bears, and all they bear, run by the law for `years` with the cell's
    light fixed. Returns per-year means over `reps` repeats."""
    rnd = random.Random(seed)
    z = s[bearer_pa]["zones"][zone_index]
    rows = [dict(alive=0.0, standing=0.0, units=0.0, nodes_new=0.0,
                 leaf_light=0.0, leaf_drawn=0.0, wood=0.0, section=0.0,
                 by_age={}, balance=[]) for _ in range(years + 1)]
    for _ in range(reps):
        members = []
        # Birth: the node's bud places, as today; and its sleeping buds,
        # which wake at their hazard after their delay (dormant.rs).
        for _slot in range(int(z["buds"])):
            u, acc = rnd.random(), 0.0
            for i in range(AGES):
                acc += z["p"][i]
                if u < acc:
                    lv = lived(s, i)
                    if lv:
                        members.append(Member(lv[0], 0, None))
                    break
        sleepers = []
        for _slot in range(int(z["buds"])):
            for i in range(AGES):
                if z["dormant"][i] > 0 and rnd.random() < z["dormant"][i]:
                    sleepers.append(i)
        for year in range(1, years + 1):
            woke = []
            for i in list(sleepers):
                if year > z["delay"] and rnd.random() < z["rate"]:
                    sleepers.remove(i)
                    lv = lived(s, i)
                    if lv:
                        woke.append(Member(lv[0], year - 1, None))
            members.extend(woke)
            born = []
            for m in members:
                if m.alive:
                    born.extend(grow(s, m, year, rnd, light))
            members.extend(born)
            row = rows[year]
            for m in members:
                if not standing(s, m, year):
                    continue
                row["standing"] += 1
                age = year - m.born
                key = (NAMES[m.pa], age)
                row["by_age"][key] = row["by_age"].get(key, 0) + 1
                if m.alive:
                    row["alive"] += 1
                for (laid, pa) in m.nodes:
                    st = s[pa]
                    row["wood"] += st["inode"]
                    row["section"] += st["pipe"] ** 2
                    if laid == year:
                        row["nodes_new"] += 1
                        row["leaf_light"] += st["leaf"]
                    # The drawn leaf: a station on wood `year - laid` old keeps
                    # its leaf by clamp(leaf_years - age, 0, 1); a short
                    # shoot's cluster bears while its shoot lives (fn-248).
                    if st["cluster"] == 0 and year - laid < st["leaf_years"]:
                        row["leaf_drawn"] += st["leaf"]
                if s[m.pa]["cluster"] > 0 and m.end == year:
                    row["leaf_drawn"] += s[m.pa]["cluster"] * s[m.pa]["leaf"]
                if m.end == year:
                    row["units"] += 1
    for row in rows:
        for k in ("alive", "standing", "units", "nodes_new", "leaf_light",
                  "leaf_drawn", "wood", "section"):
            row[k] /= reps
        row["by_age"] = {k: v / reps for k, v in row["by_age"].items()}
    return rows


def aggregation(s, bearer_pa, cohort, years, light, seed, reps):
    """What a law over classes steps against one over members, on one whole
    unit of `bearer_pa` (every zone's nodes drawn), its laterals of the ages
    in `cohort` only: member-years standing, units grown, and classes
    standing (a class: age now, years in it, years since birth, living)
    counted per node and per unit, summed over `years`."""
    rnd = random.Random(seed)
    tot = dict(members=0.0, units=0.0, node_classes=0.0, unit_classes=0.0, nodes=0.0)
    for _ in range(reps):
        nodes = []
        for z in s[bearer_pa]["zones"]:
            for _n in range(draw_nodes(z, rnd)):
                kids = []
                for _slot in range(int(z["buds"])):
                    u, acc = rnd.random(), 0.0
                    for i in range(AGES):
                        acc += z["p"][i]
                        if u < acc:
                            lv = lived(s, i)
                            if lv and lv[0] in cohort:
                                kids.append(Member(lv[0], 0, None))
                            break
                nodes.append(kids)
        tot["nodes"] += len(nodes)
        for year in range(1, years + 1):
            unit_keys = set()
            for members in nodes:
                born = []
                for m in members:
                    if m.alive:
                        born.extend(grow(s, m, year, rnd, light))
                members.extend(born)
                keys = set()
                for m in members:
                    if not standing(s, m, year):
                        continue
                    tot["members"] += 1
                    tot["units"] += m.end == year
                    keys.add((m.pa, m.t, year - m.born, m.alive))
                tot["node_classes"] += len(keys)
                unit_keys |= keys
            tot["unit_classes"] += len(unit_keys)
    return {k: v / reps for k, v in tot.items()}


def expected_units(s, bearer_pa, zone_index, years, light):
    """The same node's expected units grown a year, by linear recursion with
    no shedding: a check on the repeats (the closed form's own arithmetic)."""
    z = s[bearer_pa]["zones"][zone_index]
    # pop[(pa, t)] expected living members entering year with t years in pa
    pop = {}
    for i in range(AGES):
        q = z["buds"] * z["p"][i]
        lv = lived(s, i)
        if q > 0 and lv:
            pop[(lv[0], 0)] = pop.get((lv[0], 0), 0) + q * (lv[1] if False else 1.0)
    sleeping = {i: z["buds"] * z["dormant"][i] for i in range(AGES) if z["dormant"][i] > 0}
    out = [0.0]
    for year in range(1, years + 1):
        if year > z["delay"]:
            for i, n in sleeping.items():
                lv = lived(s, i)
                wake = n * z["rate"]
                sleeping[i] = n - wake
                if lv:
                    pop[(lv[0], 0)] = pop.get((lv[0], 0), 0) + wake
        grown, nxt = 0.0, {}
        for (pa, t), n in pop.items():
            st = s[pa]
            g = n * survival(st, light)
            grown += g
            for j, q in enumerate(laterals_per_unit(st)):
                lv = lived(s, j)
                if q > 0 and lv:
                    nxt[(lv[0], 0)] = nxt.get((lv[0], 0), 0) + g * q
            if t + 1 >= st["L"]:
                lv = lived(s, pa + 1) if pa + 1 < AGES else None
                if lv and st["cont"] > 0:
                    nxt[(lv[0], 0)] = nxt.get((lv[0], 0), 0) + g * st["cont"] * lv[1]
            else:
                nxt[(pa, t + 1)] = nxt.get((pa, t + 1), 0) + g
        out.append(grown)
        pop = nxt
    return out


def balance_of(s, rows, light):
    """D16's balance on the whole cohort as one subtree: q = the apexes that
    grew this year x light, c = upkeep x standing wood; memory halves toward
    it each year."""
    out, m = [], 1.0
    up = max(st["upkeep"] for st in s)
    for row in rows[1:]:
        q, c = row["units"] * light, up * row["wood"]
        b = (q - c) / (q + c) if q + c > 0 else 1.0
        m = (m + b) / 2
        out.append((b, m))
    return out


# ------------------------------------------------------------------- print

def main():
    species = dict(oak=oak(), spruce=spruce(), beech=beech())
    print("## Reach and the cohort rule\n")
    for name, s in species.items():
        r_plain, r_relay, r_pot = reaches(s, False), reaches(s, True), potential(s)
        print(f"### {name}\n")
        print("| lateral age | reach (fn-334) | reach, relays carry | subtree extent (relays) | potential subtree | cohort at 0.25 / 0.5 / 1.0 m |")
        print("|---|---|---|---|---|---|")
        for j in lateral_ages(s):
            ext = extent(s, r_relay, j)
            pot = extent(s, r_pot, j)
            marks = " / ".join("yes" if ext < c else "no" for c in (0.25, 0.5, 1.0))
            print(f"| {j} {NAMES[j]} | {r_plain[j]:.3f} | {r_relay[j]:.3f} | {ext:.3f} | {pot:.3f} | {marks} |")
        print()

    cases = [
        ("oak", BRANCH, 1, [0.26, 1.0, 0.125], "branch medial node (SHORT .6, TWIG .35)"),
        ("spruce", BRANCHLET, 1, [0.49, 1.0], "branchlet medial node (2 buds, SHOOT .55)"),
        ("spruce", BRANCH, 1, [0.49, 1.0], "branch medial node (2 buds, BRANCHLET .7, sleeping .45)"),
        ("beech", BRANCH, 1, [0.73, 1.0], "branch medial node (SHORT .7)"),
        ("beech", BRANCH, 3, [0.73, 1.0], "branch spare node (SHOOT .65)"),
    ]
    reps = int(sys.argv[1]) if len(sys.argv) > 1 else 20000
    print(f"## One bearing node over ten years ({reps} repeats, seed 339)\n")
    for name, pa, zi, lights, label in cases:
        s = species[name]
        for light in lights:
            rows = bearer_node(s, pa, zi, 10, light, 339, reps)
            check = expected_units(s, pa, zi, 10, light)
            bal = balance_of(s, rows, light)
            print(f"### {name}, {label}, cell light {light}\n")
            print("| year | standing | living | units grown (law / closed form) | new nodes | light leaf m2 | drawn leaf m2 | wood m | section (sum pipe^2) | balance / memory |")
            print("|---|---|---|---|---|---|---|---|---|---|")
            for y in range(1, 11):
                r = rows[y]
                b, m = bal[y - 1]
                print(f"| {y} | {r['standing']:.3f} | {r['alive']:.3f} | {r['units']:.3f} / {check[y]:.3f} | "
                      f"{r['nodes_new']:.2f} | {r['leaf_light']:.5f} | {r['leaf_drawn']:.5f} | {r['wood']:.4f} | "
                      f"{r['section']:.3e} | {b:+.2f} / {m:+.2f} |")
            tot_units = sum(rows[y]["units"] for y in range(1, 11))
            tot_leaf = sum(rows[y]["leaf_light"] for y in range(1, 11))
            print(f"\nTen-year sums: units grown {tot_units:.2f}, light leaf deposited {tot_leaf:.5f} m2.")
            ages = sorted(rows[10]["by_age"].items(), key=lambda kv: (kv[0][0], kv[0][1]))
            print("Standing at year 10 by (age now, years since birth): "
                  + ", ".join(f"{k[0]} {k[1]}y {v:.3f}" for k, v in ages if v >= 0.0005) + "\n")

    print("## Steps a law over classes takes against one over members, one bearer unit, ten years\n")
    print("| species, bearer unit, cohort ages, light | nodes | member-years standing | units grown | class-years per node | class-years per unit |")
    print("|---|---|---|---|---|---|")
    agg = [("oak", BRANCH, {TWIG, SHORT}, 0.26), ("oak", BOUGH, {TWIG, SHORT}, 0.26),
           ("spruce", BRANCHLET, {SHOOT}, 0.49), ("spruce", BRANCH, {BRANCHLET, SHOOT, SPUR}, 0.49),
           ("beech", BRANCH, {SHOOT, SHORT}, 0.73)]
    for name, pa, cohort, light in agg:
        a = aggregation(species[name], pa, cohort, 10, light, 339, max(reps // 10, 200))
        ages = ", ".join(NAMES[c] for c in sorted(cohort))
        print(f"| {name}, {NAMES[pa]}, {ages}, {light} | {a['nodes']:.2f} | {a['members']:.1f} | {a['units']:.1f} | "
              f"{a['node_classes']:.1f} | {a['unit_classes']:.1f} |")


if __name__ == "__main__":
    main()
