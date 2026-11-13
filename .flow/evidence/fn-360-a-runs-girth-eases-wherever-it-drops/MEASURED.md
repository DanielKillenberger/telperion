# Measured before and after, and the stop

Builder, 13 Nov 2026. Base `b93621b5b`, after = the rule in `surface/samples.rs` (this branch). Read by a scratch probe (`girth` subcommand, the fork probe's crate pointed at this worktree), one process per tree, pinned to one core (`taskset -c 5`), inside `telperion-heavy 16`. No PAUSE line of `yield.log` fell inside either run (base 01:22:24–01:25:20, after 01:27:12–01:29:42), so the seconds count.

Definitions: a "fall" is `1 − r_j / r_(j−1)` between consecutive rings of one run of `Expansion::curve()`, past each run's first pair (the socket, or the trunk's buried root); radii include the flare and the collar swell. "Trunk run" is the widest trunk run, root to tip. "Forks at floor" counts forks over 5 cm with a codominant child whose run has a ring-to-ring slope ≥ 0.995 within two fork diameters above (the fork probe's STEP reading). Surface seconds: the least of three `Expansion::wood()` builds.

| tree | forks at floor | trunk largest fall | largest fall, all runs | pairs > 2% (of all pairs) | wood vertices | surface s |
|---|---|---|---|---|---|---|
| oak 120 s1001, base | 2 of 47 | 21.7% | 88.8% | 1,126,983 of 2,451,235 | 59,036,786 | 4.56 |
| oak 120 s1001, after | **0 of 47** | 21.7% | 88.7% | 1,129,398 | 59,036,786 | 4.29 |
| oak 200 s1001, base | 4 of 33 | 9.3% | 92.6% | 365,557 of 786,795 | 18,884,766 | 1.33 |
| oak 200 s1001, after | **0 of 33** | **15.6%** | 84.7% | 367,179 | 18,884,766 | 1.17 |
| spruce 40 s1, base | 0 of 0 | 33.3% | 68.8% | 630,691 of 886,785 | 26,373,798 | 1.87 |
| spruce 40 s1, after | 0 of 0 | 33.3% | 68.8% | 630,674 | 26,373,798 | 1.85 |
| beech 40 s1, base | 0 of 9 | 26.9% | 85.7% | 123,718 of 416,671 | 9,414,878 | 0.69 |
| beech 40 s1, after | 0 of 9 | 27.6% | 85.7% | 123,726 | 9,414,878 | 0.65 |

Wood vertices are unchanged on all four trees (0%). The large falls and the half of all pairs over 2% are dominated by the collar swell's decay and twig tips on both sides; the reading as defined does not separate them, so it does not show the bands. Where on the oak's trunk run at 200 the largest fall rose from 9.3% to 15.6%, and on the beech's from 26.9% to 27.6%, was not located: unknown.

## Stop: children newly start inside the bearer's drawn wood

For each lateral run, its first own ring (k 1) was set against the bearer run's drawn surface at the attach node: inside when the ring's distance from the bearer's axis plus its radius is under the bearer's drawn radius there. The socket reads the bearer's node radius (READING.md, 4), and the rule draws the bearer above its node radius after every drop, so a lateral can only move inward.

| tree | laterals | inside, base | inside, after | newly inside | no longer inside |
|---|---|---|---|---|---|
| oak 120 | 238,382 | 8,949 | 9,063 | **114** | 0 |
| oak 200 | 74,972 | 3,279 | 3,302 | **23** | 0 |
| spruce 40 | 205,668 | 9,353 | 9,354 | **1** | 0 |
| beech 40 | 25,748 | 737 | 744 | **7** | 0 |

Sites, by how far inside the first ring sits (margin = bearer drawn r − (axis distance + child ring r)):

| tree | child node | margin m | bearer drawn r at attach | bearer node r |
|---|---|---|---|---|
| oak 200 | 609572 | 0.00251 | 0.00744 | 0.00313 |
| oak 200 | 420570 | 0.00059 | 0.03543 | 0.03475 |
| oak 200 | 494429 | 0.00036 | 0.00398 | 0.00398 |
| oak 120 | 1052181 | 0.00260 | 0.03863 | 0.03771 |
| oak 120 | 604239 | 0.00244 | 0.00489 | 0.00158 |
| oak 120 | 1296456 | 0.00217 | 0.01144 | 0.00614 |

The newly inside children are on twigs and small limbs (bearer 1 mm to 4 cm), margins at most 2.6 mm. None floats off: no lateral left the inside set. This was the spec's stop ("a child that starts inside or off the bearer's drawn wood"), and the run returned with it. The full lists are in the scratch directory (`inside-{base,after}-<tree>.txt`), not committed.

**Host decision, 13 Nov 2026:** a lateral that starts a few millimetres inside its bearer's drawn wood is accepted, and the build goes on. The stop guarded against a visible fault: a child starting inside the bearer is hidden by the bearer's surface (0 float off, at most 2.6 mm inside, 145 of about 313,000 laterals on the two oaks), while a child floating off would show. Sockets keep reading the node radius.

## Pins

- `pipeline::cycle_cost_tests::every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` (upstream, `telperion-space`'s grown structure, every preset at 16, seed 7) fails on the base commit in a separate worktree with the same values it fails with after: pre-existing, not moved by this change. Upstream unchanged: checked.
- `suite::rachis::the_axes_move_no_leaf` (the palm at 40, every leaf's transform): passes on base and after.
- `suite/species/digests.json` is read only by tests ignored under fn-198; not run.

## The trunk falls that grew (host's question 1, 13 Nov)

Read by `probe trunkfall` (the widest trunk run's rings; target = the node's radius, girth = ring radius over the flare), base and after built from separate checkouts.

**Oak 200, k 325 (y 12.87, 8 m above the trunk forks).** The ring before lies 0.232 m back, against 0.033 m between the other rings here.

| k | d | target | base girth | after girth | after fall |
|---|---|---|---|---|---|
| 320 | 13.519 | 0.20799 | 0.20799 | 0.30242 | 0.3% |
| 322 | 13.577 | 0.20789 | 0.20789 | 0.30114 | 0.2% |
| 323 | 13.610 | 0.20797 | 0.20797 | 0.29916 | 0.7% |
| 324 | 13.644 | 0.20794 | 0.20794 | 0.29579 | 1.1% |
| 325 | 13.876 | 0.20790 | 0.20790 | 0.24967 | **15.6%** |
| 326 | 13.909 | 0.20790 | 0.20790 | 0.24204 | 3.1% |
| 329 | 14.009 | 0.20790 | 0.20790 | 0.22179 | 2.7% |

Base draws the target here (falls under 0.05%). After, the drawn girth is 0.302 m against a radius of 0.208 m, 45% over it, 8.5 m above the forks, and the 15.6% is that excess going down over one long segment (slope 0.20 per metre).

Why the excess lasts so long: the targets along this stretch do not fall once and hold; they wobble by 0.01 to 0.05% (0.20799, 0.20789, 0.20797, 0.20794). As built, each new low below the ease's target counts as "a further drop met inside an ease" and starts a new ease from the girth drawn there, over twice that girth, at t = 0. `(1 − t)²(1 + 2t)` starts flat, so every restart holds the drawn girth near where it was, and a trunk whose radius creeps down in tiny steps never comes back to its radius. At k 111–120 (the two trunk forks), after: 0.660 → 0.649 m over 0.44 m against a target of 0.341 (base: 0.650 → 0.432 with the 9.3% ledge).

**Is this the spec's rule?** The text says a further drop "does not restart it from the node's own radius: the ease carries on from the girth drawn there, toward the new target, over twice that drawn girth". I built that as a fresh ease from the drawn girth at each new low (a new start, at t = 0). Read the other way, "carries on" keeps the ease's progress and only moves its target. The two readings differ exactly here, and with the fresh-start reading a tiny drop stalls the ease. That is a design question, so the run stops here, as the host asked, with no fix.

**Beech 40, k 156 (y 7.96).** The target falls 27.8% in one ring (0.02243 → 0.01620) over 0.040 m, while twice the girth is 0.045 m, so no ease can spread it. After, the ring before is still easing from the drop at k 152 (0.02275 against a target of 0.02243), so the same fall is taken from a higher girth: 0.02275 → 0.01646 is 27.6%, against base 0.02243 → 0.01640, 26.9%.

## The bands on sibling stem 120, oak 200 (host's question 2)

`probe stem`, run 120 from its first ring, rings as FORK-PROBE.md §7 numbers them:

| ring pair | what steps | node r | base fall | after fall |
|---|---|---|---|---|
| k 7 → 8 (d 0.578, band at row ~254) | relay join over a 0.40 m segment | 0.3853 → 0.3853 | 9.3% | 9.2% |
| k 25 → 26 (d 1.197, band at row ~81) | growth unit 19 → 20 | 0.3851 → 0.3706 | 4.0% | **0.3%** |
| k 38 → 39 (d 1.632) | growth unit 21 → 22 | 0.3706 → 0.3565 | 3.9% | **0.2%** |

The growth-unit steps no longer draw as a fall. At the relay join the node radius does not drop. The 9.3% there is the collar swell decaying over one long segment, which this rule does not touch, so that band is unchanged.

## The upstream pin failing on base

`every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` expects `[(9819307049801252693, 9819307049801252693), (15182297322291910030, 7299178540230679106), (12665486350558508414, 9318247602647309830), (16361327149142543059, 16361327149142543059), (2054489571488262802, 18014430067925384448), (4799502948138528708, 4799502948138528708), (3392261673279751331, 3392261673279751331), (16456616023369512333, 16456616023369512333)]` and gets `[(13986120611573680668, 13986120611573680668), (14944671683796001899, 475426537269313102), (12665486350558508414, 8154765959244887056), (13924919106548380920, 13924919106548380920), (2054489571488262802, 9663260454690218178), (4799502948138528708, 4799502948138528708), (3392261673279751331, 3392261673279751331), (16456616023369512333, 16456616023369512333)]`, the same on base and after.

## Not done because of the stop

R5's stills were not drawn, and `flowctl done` was not run: the run stopped at the question above. The full narrow lists of `pipeline` tests beyond `surface`, `fork`, `bark` and the two pins were not run.
