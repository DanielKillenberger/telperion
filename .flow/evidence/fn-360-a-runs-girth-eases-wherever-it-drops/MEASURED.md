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

## The stateless rule (host, 13 Nov, second decision)

The host replaced the first rule with one that has no state: `g(s) = max over rings s' ≤ s of r(s) + max(0, r(s') − r(s)) · w((s − s')/2r(s'))`. "after2" below is that rule without the 1-in-1 floor, measured to decide the floor; the floor is then kept (below), and the narrow tests ran with it. Same probe, one core, no PAUSE inside the run (01:49–01:52).

| tree | forks at floor | trunk largest fall | pairs > 2% | wood vertices | surface s | steepest girth slope, no floor |
|---|---|---|---|---|---|---|
| oak 120 | 0 of 47 (base 2) | 21.7% (base 21.7%) | 1,126,904 (base 1,126,983) | unchanged | 4.61 (base 4.56) | 0.83 |
| oak 200 | 0 of 33 (base 4) | **5.1%** (base 9.3%) | 365,624 (base 365,557) | unchanged | 1.32 (base 1.33) | **1.57** |
| spruce 40 | 0 | 33.3% (base 33.3%) | 630,677 (base 630,691) | unchanged | 2.12 (base 1.87) | 0.21 |
| beech 40 | 0 of 9 | 27.9% (base 26.9%) | 123,704 (base 123,718) | unchanged | 0.67 (base 0.69) | 0.79 |

**The floor stays.** Without it, oak 200 falls 1.57 in 1 at node 27778 (d 18.471), girth 0.00577 → 0.00336 over 0.0015 m, a twig. The floor is back as a separate step after the rule.

**Oak 200 trunk past the two forks** (node 112 at k 113, d 5.034; target = radius):

| k | d − 5.034 | radius | drawn girth |
|---|---|---|---|
| 113 | 0.00 | 0.4726 | 0.6369 |
| 114 | 0.05 | 0.3408 | 0.6042 |
| 116 | 0.15 | 0.3408 | 0.5729 |
| 118 | 0.25 | 0.3408 | 0.5383 |
| 120 | 0.35 | 0.3408 | 0.5021 |
| 122 | 0.45 | 0.3408 | 0.4659 |
| 124 | 0.55 | 0.3408 | 0.4313 |
| 126 | 0.65 | 0.3408 | 0.4001 |
| 128 | 0.75 | 0.3408 | 0.3738 |
| 130 | 0.85 | 0.3408 | 0.3542 |
| 132 | 0.95 | 0.3408 | 0.3429 |
| 134 | 1.05 | 0.3250 | 0.3406 |
| 140 | 1.35 | 0.3154 | 0.3276 |
| 145 | 1.60 | 0.3154 | 0.3208 |

The largest trunk fall is 5.1%, at k 114 (slope 0.65): the second fork's own step, 0.6369 → 0.6042, with the first fork's wood still holding.

**Oak 200 at k 320 to 325:** 0.21115, 0.20993, 0.20911, 0.20841, 0.20798, 0.20792 against radii 0.20799 to 0.20790, so 1.5% over at k 320, falling to 0.01% at k 325. This is not creep. At k 318 (d 13.45) the radius steps 6.1%, 0.22145 → 0.20799, and k 320 is 0.07 m into that step's ease (the rule's single-step ease; twice the radius is 0.44 m). The creep before it (k 296–317, radius 0.22151 → 0.22145) is drawn within 0.02%.

**The creeping-stem test as specified fails, and is held back.** The test asked for a run thinning 0.05% a ring over 200 rings, 0.05 m apart at 0.3 m, to be drawn within 0.1% of its radii. The rule draws it up to 0.156% over (red: "0.29993 against 0.29955 at 0.15"). On paper, a creep of c per ring held over 2r/Δ rings gives an excess of about c · (2r/Δ) · max t·w(t) ≈ c · (2r/Δ) · 0.30. That is 0.156% here, 0.236% at 0.033 m spacing, and 0.094% at a creep of 0.03% a ring. The upstream ring's surplus counts against every ring within its reach, not just once. Whether the bound or the test changes is the host's call. The test is not committed.

**Bands on sibling stem 120 (after2):** not re-read on the stateless rule; the first rule's reading stands above.

**Laterals starting inside their bearer (after2 against base):** oak 120: 48 newly inside, deepest 1.7 mm (child 1052181); oak 200: 7, deepest 0.5 mm (420570); spruce 40: 1, 0.3 mm (20284); beech 40: 2, 1.1 mm (17983). None is no longer inside, so none floats off. All are under 1 cm, within the earlier acceptance.

## R5: the sheet

`sheet.jpg`, base left and after right (the stateless rule with the floor, `c526b1531`), leafless. Rows: the beech's trunk at 40, the oak's first fork at 200 from az 0 and az 90 (the probe's camera, `FORK_CAM=-0.07,4.9,0.016,4.0`), the oak's stem 120 at 200 (eye 2.214, 5.6, −1.0015 toward −0.07, 5.6, 0.016), the spruce's trunk at 40. One GPU job, 01:57–02:01.

The builder's description of what the stills show, for the host to judge:
- Oak fork, both azimuths: the base's flat, square-edged ledge is gone. The trunk's top is now a rounded shoulder, with the bark running from the trunk up into the stems on the far side. The rounded top edge is still visible on the near side, more of it than in the probe's trial A (`fork/fork-sheet2.jpg`), which drew a straight taper there. The numbers agree: trial A kept 0.618 m at 0.2 m past node 112, and this rule draws 0.573 m at 0.15 m. Under the rule as specified, the first fork's wood holds the second fork's drop over its own remaining ease and does not start a new one. A thin dark band runs round the trunk at the shoulder, as in trial A.
- Stem 120: the base's dark band across the stem at mid-frame (a growth-unit step) is gone. The dark line low in the frame, where the stem meets the trunk's shoulder, remains.
- The beech and spruce trunks: no difference visible at this distance.

## The additive rule checked on paper before building (host's third form, 13 Nov)

The rule is `g(s) = r(s) + Σ drops j ≤ s of δ_j · w((s − s_j)/2a_j)`. Its paper claims were checked in Python (scratch) on the creeping stem and on oak 200's real trunk radii and distances (`probe trunkfall … 0 391`, all 391 rings), with the 1-in-1 floor. The product code was not changed this round.

**The creeping stem does not come out.** For 0.05% a ring, 0.05 m rings and r 0.3 m, the excess reaches **0.325%** of r, against the stated "at most 0.3%". At ring 150 the excess is 0.000849 m, while half the thinning within the upstream window 2r(s) (11 rings back) is 0.000768 m, so the test's bound "half the thinning within the upstream window, plus 1e-6" fails by about 10%. The reason is that the holds are a sum at discrete rings: Σ_{n=0}^{N−1} w(n/N) = N/2 + 1/2. The newest drop counts at w(0) = 1 in full, so the excess is half the window's thinning plus half of one ring's drop (6.5 δ against 6 δ here).

**Oak 200's trunk under the additive rule (paper):**

| k | m past node 112 | radius | g |
|---|---|---|---|
| 106 | −0.315 (fork 1, node 105) | 0.66406 | 0.6643 |
| 107 | −0.270 | 0.47261 | 0.6643 |
| 113 | 0.000 (fork 2, node 112) | 0.47261 | 0.6437 |
| 114 | 0.050 | 0.34079 | 0.6362 |
| 115 | 0.100 | 0.34079 | 0.6268 |
| 121 | 0.400 | 0.34079 | 0.5262 |
| 128 | 0.750 | 0.34079 | 0.3889 |
| 131 | 0.900 | 0.34079 | 0.3520 |
| 135 | 1.100 | 0.32504 | 0.3405 |
| 140 | 1.350 | 0.31544 | 0.3343 |

- **Continuity at the drops: comes out.** At k 107, g holds at 0.6643 across the drop. At the second fork (k 113 → 114) g falls 1.2%, from the first fork's hold fading, against the stateless rule's 5.1%.
- The trunk run's largest fall moves off the second fork, to k 126 (d 5.684, 0.65 m past it): 4.4%, with the flare included.
- "About 0.512 at 0.35 m further" comes out: 0.526 at 0.40 m.
- "0.615 just past the second fork" does not: g is 0.636 there. The paper took the first drop at 0.3 m back; on the tree it is 0.315 m back, at a radius of 0.664, not 0.637.
- "Equal to 0.341 from 1.27 m past the first fork" does not: the trunk keeps thinning past the forks (0.341 → 0.325 → 0.315), and each of those drops lays its own hold. At 1.35 m past node 112, g is 0.334 against r 0.315.

The run stops here, as the host asked, before building: the creeping-stem arithmetic does not come out, and the bound is the host's to set.

## The additive rule built: the drop ring is held at the radius above it (stop)

Built as written (branch `fn-360-additive-wip`, commit `91e6a65ea`, not on the task branch), the narrow run gives 77 passed and 6 failed.

The six failures come from one property. The formula anchors each hold at s_j, the ring the wood falls to, with w(0) = 1. So that ring is always drawn at the girth above it, however far apart the rings are, and a drop shows only from the next ring on. The earlier rules measured from the ring the wood fell from, s_(j−1), where a drop over a segment longer than 2a completes inside the segment. Worked examples, rings as (distance, radius):

| case | rings | anchored at s_j (as specified) | anchored at s_(j−1) |
|---|---|---|---|
| suite taper, 4 m apart | 1, 0.5, 0.333 | 1, **1.0**, 0.5 | 1, 0.5, 0.333 |
| tip below float32, 4 m apart | 1, 1, 0.001 | 1, 1, **1.0** | 1, 1, 0.001 |
| twig, 3 cm apart | 4, 4, 2 mm | 4, 4, **4 mm** | 4, 4, 2 mm |

As specified, every tip ring is drawn at the ring below it, so every shoot's last segment is drawn untapered with a blunt cap. A ring can no longer collapse to a point after a drop. Every taper on rings spaced wider than twice their radius (most twigs) lags one segment.

Failures:
- `suite::surface::tapered_closed_surface_has_normals_and_bounds` checks drawn ring = node radius on a 3-ring taper.
- `suite::surface_collapse::a_ring_collapsed_to_one_point_drops_the_strips_that_meet_it` and `a_tip_below_float32_resolution_far_out_drops_its_cap` check the sweep's handling of degenerate rings, which can no longer occur after a drop.
- `fork_tests::the_trunks_girth_eases_into_the_stem_over_the_forks_diameter` and `shoulder_tests::a_run_never_thins_faster_than_one_in_one` expect the ease to start at the wider ring; under the s_j anchor it starts one ring later.
- The new creeping test fails near the run's base. With fewer than a full window of rings upstream, the holds have not faded: ring 3, 0.15 m from the root, is 0.000436 over against a bound of 0.000376. Rings with a full window behave as the paper says.

The spec's own lines disagree here. "One step from a to b draws b + (a − b) w(Δ/2a)", with Δ the step's length, is the s_(j−1) anchor; "g does not move at a drop" is the s_j anchor. Which anchor the rule uses is the host's call. The run stops here: the task branch holds the max-form rule at `c526b1531`, green on its tests.

## The final rule built: holds measured from the ring the wood fell from (host, 13 Nov)

`g(s) = r(s) + Σ δ_j · w((s − s_(j−1))/2a_j)` with the 1-in-1 floor, on the task branch at `e7444be0f`. Narrow run: 83 passed, 0 failed (surface, fork, bark, the palm's leaf pin). The five tests that failed on the s_j anchor pass again unchanged. The creeping test passes with the host's two-part bound.

**R4, final rule** (one core, no PAUSE inside 02:15–02:18; base from the first table):

| tree | forks at floor | trunk largest fall | pairs > 2% | wood vertices | surface s | steepest girth slope (floor on) | largest girth / radius |
|---|---|---|---|---|---|---|---|
| oak 120 | 0 of 47 (base 2) | 21.7% (21.7%) | 1,126,900 (1,126,983) | unchanged | 4.68 (4.56) | 0.89 | 23.7 at node 77471 (0.0153 against 0.00064) |
| oak 200 | 0 of 33 (base 4) | **4.4%** at k 125 (9.3% at the second fork) | 365,615 (365,557) | unchanged | 1.31 (1.33) | 0.92 | 77.9 at node 20660 (0.1172 against 0.0015) |
| spruce 40 | 0 | 33.3% (33.3%) | 630,679 (630,691) | unchanged | 2.05 (1.87) | 0.21 | 1.69 at node 360558 |
| beech 40 | 0 of 9 | 27.9% (26.9%) | 123,704 (123,718) | unchanged | 0.73 (0.69) | 0.83 | 82.6 at node 126 (0.0273 against 0.00033) |

The largest girth/radius ratios sit where a run carries on into a child whose own radius is a small fraction of its start (the followed child's `start_radius` equals the parent's, FORK-PROBE §1). The rule holds the fall over twice the radius it fell from, so a 2.7 cm limb running on into a 0.3 mm shoot is drawn wide for about 5 cm. Not compared against base (the base probe has no such reading).

**Oak 200's trunk, built against paper:** the girth at k 106, 107, 113, 114, 115, 121, 128, 131, 135 and 140 is 0.66432, 0.66365, 0.63697, 0.62765, 0.61562, 0.50652, 0.37490, 0.34559, 0.33987 and 0.33243. The paper (s_(j−1) anchor, `paper-final.txt`) gave 0.6643, 0.6636, 0.637, 0.6276, 0.6156, 0.5065, 0.3749, 0.3456, 0.3399 and 0.3324, so they agree to rounding. Across the second fork (k 113 → 114) the girth falls 1.5%. The largest trunk fall is 4.4% at k 125 (d 5.634, 0.6 m past the second fork), as on paper.

**Bands on stem 120, final rule:**

| ring pair | base | first rule | final |
|---|---|---|---|
| k 7 → 8, relay join (swell over one 0.40 m segment) | 9.3% | 9.2% | 9.3% |
| k 25 → 26, growth unit 19 → 20 | 4.0% | 0.3% | 0.3% |
| k 38 → 39, growth unit 21 → 22 | 3.9% | 0.2% | 0.4% |

**Laterals against their bearer (final against base):** newly inside: oak 120 51 (deepest 2.3 mm, child 1052181), oak 200 7 (0.5 mm), spruce 1 (0.3 mm), beech 2 (1.1 mm). None left the inside set, so none floats off.

**Stop: laterals already inside at base now sit more than 1 cm deeper.** Measured as the change in each lateral's margin (bearer drawn radius − its first ring's reach) against base:

| tree | laterals over 1 cm deeper | deepest | its depth, base → final | bearer drawn / node radius at attach |
|---|---|---|---|---|
| oak 120 | 12 | child 113 (the second trunk fork's sibling), +2.06 cm | 15.3 → 17.4 cm | 0.503 / 0.386 |
| oak 200 | 14 | child 4927, +3.20 cm | 15.3 → 18.5 cm | 0.304 / 0.279 |
| oak 200 | | child 113 (the second trunk fork's sibling), +2.10 cm | 15.7 → 17.8 cm | 0.638 / 0.473 |
| beech 40 | 1 | child 115, +3.50 cm | 5.0 → 8.5 cm | 0.087 / 0.0014 |
| spruce 40 | 0 | child 453, +0.20 cm | | |

These children were inside already at base: fork siblings and laterals sunk into their socket by design. They sit deeper because the bearer no longer drops where they leave; at a fork that is the ledge being removed. The host's acceptance was "none floating off, none over 1 cm in". These are over 1 cm deeper, though none floats off and none was outside before. The run stopped here with the sheet drawn and without `flowctl done`.

**Host decision, 13 Nov 2026:** accepted. The laterals already sunk into their bearer that now sit up to 3.5 cm deeper were inside by design, none floats off, and at a fork the extra depth is the ledge being removed. The host viewed the final sheet: no ledge, no shoulder, no band round the trunk, the stem's growth-unit band gone, spruce and beech unchanged; better than the base and than trial A.

**R5, final sheet** (`sheet.jpg`, redrawn 02:26 on `e7444be0f`; replaces the max-form sheet). Rows: oak 200 fork az 0 and az 90 (base | after | the probe's trial A), then beech 40, oak 200 stem 120, spruce 40 (base | after). The builder's description, for the host to judge:
- Oak fork, both azimuths: no ledge and no rounded shoulder. The trunk tapers continuously into the stems, and no dark band runs round the trunk; trial A, beside it, still has one.
- Stem 120: the growth-unit band across the stem is gone.
- Beech and spruce trunks: no visible difference.

## Not judged: thick wood carrying on into thin

The two largest drawn-girth-to-radius sites, read with `probe around` on base `b93621b5b` and the final rule `e7444be0f` (girth = ring radius over the collar swell; radius = the node's own). In both, a thick axis ends and its run carries on into a thin lateral that takes over. A run follows the widest child by `start_radius` (`paths.rs`), so a 2 mm shoot is the run's continuation when it is the only child left.

**Oak 200, node 20660** (the limb of branch 168 ends at node 485, k 100, and the run carries on into the shoot of branch 10990):

| k | d | node | radius | base girth | final girth |
|---|---|---|---|---|---|
| 100 | 8.4456 | 485 (limb, last node) | 0.22735 | 0.22735 | 0.22740 |
| 101 | 8.6038 | 10990 (shoot, first node) | 0.00308 | 0.06917 | 0.16481 |
| 149 | 8.6628 | 20655 | 0.00157 | 0.01011 | 0.12153 |
| 150 | 8.6639 | 20656 | 0.00157 | 0.00902 | 0.12056 |
| 151 | 8.6650 | 20657 | 0.00156 | 0.00797 | 0.11964 |
| 152 | 8.6660 | 20658 | 0.00156 | 0.00691 | 0.11881 |
| 153 | 8.6671 | 20659 | 0.00155 | 0.00582 | 0.11801 |
| 154 | 8.6682 | **20660** | 0.00150 | 0.00477 | 0.11721 |
| 155 | 8.6692 | 20661 (run's last ring) | 0.00150 | 0.00373 | 0.11641 |

(k 154 has only one ring after it; the five before it are listed.) A limb of 0.227 m radius ends, and its run carries on into a 3 mm shoot 0.158 m beyond it. Base drew the fall with the 1-in-1 floor: 0.069 m at the shoot's first ring, down to 4 mm at its tip 6.5 cm on. The final rule holds the limb's fall over twice its radius, 0.45 m, so the whole 6.5 cm shoot is drawn as the tail of a cone 0.165 to 0.116 m wide, about 78 times its own radius.

**Beech 40, node 126** (the axis of branch 27, r 0.100 m, ends at node 30, k 4; the run carries on into branch 39, a 2.2 mm shoot, 2 mm beyond it):

| k | d | node | radius | base girth | final girth |
|---|---|---|---|---|---|
| 4 | 1.8837 | 30 (axis, last node) | 0.10029 | 0.10030 | 0.10045 |
| 5 | 1.8857 | 39 (shoot, first node) | 0.00220 | 0.09832 | 0.10042 |
| 66 | 2.0068 | 100 | 0.00046 | 0.00046 | 0.03307 |
| 67 | 2.0084 | 122 | 0.00046 | 0.00046 | 0.03192 |
| 68 | 2.0101 | 123 | 0.00046 | 0.00046 | 0.03078 |
| 69 | 2.0117 | 124 | 0.00041 | 0.00041 | 0.02960 |
| 70 | 2.0134 | 125 | 0.00035 | 0.00035 | 0.02843 |
| 71 | 2.0150 | **126** | 0.00033 | 0.00033 | 0.02731 |
| 72 | 2.0167 | 127 | 0.00033 | 0.00033 | 0.02621 |
| 73 | 2.0184 | 128 (run's last ring) | 0.00033 | 0.00033 | 0.02514 |

A 0.10 m axis ends, and its run carries on into a 2.2 mm shoot 2 mm beyond it. Base drew the fall 1 in 1 and was back at the shoot's radius 0.12 m on (by k 66). The final rule holds it over 0.20 m, so the shoot's 0.13 m tip is drawn 25 to 33 mm wide against 0.33 to 0.46 mm.

## Closed

The task closed on the final rule (`e7444be0f`) after the host's acceptance above. Not run: the `pipeline` tests outside the surface, fork, bark and pin filters; the full gate is the phase's, at its landing.

## Follow-up: a run ends at its own radius (held, 13 Nov; superseded below)

The host asked for `ℓ_j = min(2a_j, S_end − s_(j−1))`, so that every hold is zero at a run's last ring, with the 1-in-1 floor kept, and for a test: a 0.2 m run carrying on into a 3 mm shoot 0.2 m long ends at 3 mm and falls monotonically along the shoot.

- On the old rule the test was red: the run ended at 0.1015 m (`100ddba9b`).
- On the change it is still red: the run ends at **0.0192 m**. The rule alone draws 0.1692, 0.1015, 0.0338 and 0.0030 m at the shoot's four rings (0.05 m apart). The floor, max(girth, below − 0.05), lifts them to 0.1692, 0.1192, 0.0692 and **0.0192**. A 0.197 m fall over the shoot's 0.2 m cannot reach the radius under a 1-in-1 floor once the ease front-loads the first rings.
- The change also breaks `a_rings_girth_reads_only_the_wood_up_to_it`. Each girth now reads the run's end (S_end), so a run cut at a ring ends at that ring's radius: at ring 21 the cut run draws 0.59 against the whole run's 0.6392. That test checked locality, which the change gives up by design. The girth is still a pure function of the run's radii and distances.

The fix (code, spec formula, docs) is held on branch `fn-360-run-end-wip` at `7bcd0e27c`. The red test is reverted on the task branch (`7c1621a89`), so the task branch is green as before. Not re-read: the two sites, oak 200's trunk, g/r and the laterals; they wait on the host's choice of the floor or the test.

## Follow-up landed: a run ends with no hold (host, 13 Nov)

Host decision: the floor stays and outranks the run's end. A run ending where the floor draws it is no worse than the base, which fell 1 in 1 there too. The fix, `ℓ_j = min(2a_j, S_end − s_(j−1))`, is on the task branch (`6d336051d`; docs and spec `7a1cd78fb`), with the tests restated:
- `a_run_ends_with_no_hold`: below a 0.2 m shoot, the last ring is drawn with no hold, at max(radius, the ring before − distance) = 0.0192 m, under the old rule's 0.1015 m. Below a 0.6 m shoot it ends at 3 mm. Both narrow all along the shoot.
- `a_rings_girth_reads_only_the_wood_up_to_it_and_the_runs_end`: extending a run moves no ring at least twice the widest upstream radius before the old end, to the bit; a run cut at a ring draws that ring with no hold.
- The host's wording called the end "the floor-only girth". With no holds anywhere the run's end in that fixture would be 3 mm. The 0.0192 m is the floor from the ring before, which still carries its holds, so the test asserts "no hold at the last ring".

Narrow run: 84 passed, 0 failed. No PAUSE inside the reading run, 02:39–02:42.

**Oak 200's trunk at k 106 to 140: unchanged to the bit** (0.66432, 0.66365, 0.63697, 0.62765, 0.61562, 0.50652, 0.37490, 0.34559, 0.33987, 0.33243). The fork stills would not change, so the sheet was not redrawn.

**The two sites, girth against base:**

| oak 200 k | node | radius | base | before this fix | now |
|---|---|---|---|---|---|
| 101 | 10990 | 0.00308 | 0.06917 | 0.16481 | 0.06922 |
| 149 | 20655 | 0.00157 | 0.01011 | 0.12153 | 0.01016 |
| 154 | **20660** | 0.00150 | 0.00477 | 0.11721 | 0.00482 |
| 155 | 20661 (end) | 0.00150 | 0.00373 | 0.11641 | 0.00378 |

| beech 40 k | node | radius | base | before this fix | now |
|---|---|---|---|---|---|
| 5 | 39 | 0.00220 | 0.09832 | 0.10042 | 0.10039 |
| 30 | 64 | 0.00129 | 0.04867 | | 0.06751 |
| 50 | 84 | 0.00111 | 0.00896 | | 0.02780 |
| 66 | 100 | 0.00046 | 0.00046 | 0.03307 | 0.00252 |
| 71 | **126** | 0.00033 | 0.00033 | 0.02731 | 0.00051 |
| 73 | 128 (end) | 0.00033 | 0.00033 | 0.02514 | 0.00033 |

The oak's shoot is now drawn as the base drew it, the floor's 1-in-1 fall, within 0.05 mm. The beech's shoot (0.17 m from the 0.10 m axis to its tip) ends at its radius. Along the way it is wider than the base drew it (0.068 against 0.049 m at k 30, 0.028 against 0.009 m at k 50), since the hold now fades over the shoot's own length.

**Largest girth / radius per tree:** oak 120 23.7 (node 77471, unchanged); oak 200 31.0 (node 26062, 6.5 mm on 0.21 mm; was 77.9 at node 20660); spruce 40 1.69 (node 360558, unchanged); beech 40 52.5 (node 64, 0.068 m on 1.3 mm, mid-shoot on the same run; was 82.6 at node 126).

**R4 rows, this fix:** forks at the floor 0 of 47 and 0 of 33; trunk largest fall 21.7%, 4.4% (k 125), 33.3%, 27.9%; wood vertices unchanged on all four trees; surface stage 4.63, 1.18, 1.87 and 0.66 s.

**Laterals against their bearer, against base:** none floats off anywhere. Newly inside: oak 120 51 (deepest 2.3 mm), oak 200 7 (0.5 mm), spruce 1 (0.3 mm), beech 2 (1.2 mm). Over 1 cm deeper: oak 120 12 (most 2.1 cm, child 113), oak 200 14 (most 3.2 cm, child 4927), beech 1 (1.9 cm, child 115, was 3.5 cm), spruce 0. These are within the host's acceptance above.

**Generation-limit guard (13 Nov):** the ease left five unclassified sites in `samples.rs`. Two are stale: one `.max(0.0)` and one `.max(1e-9)` belonged to the removed fork ease and are dropped from the inventory. Three are new, each classified `algorithm` as a bound of the formula, none a cap on growth: `.max(2.0 * own)` (the scan reach), `.max(below - (at(k) - at(k - 1)))` (the 1-in-1 floor) and `.min(end - at(j - 1))` (the hold length). Both sub-second guards pass; the 84 narrow tests pass unchanged.
