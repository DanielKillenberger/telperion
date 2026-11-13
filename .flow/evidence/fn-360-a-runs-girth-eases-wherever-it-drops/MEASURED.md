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

The newly inside children are on twigs and small limbs (bearer 1 mm to 4 cm), margins at most 2.6 mm. None floats off: no lateral left the inside set. This is the spec's stop ("a child that starts inside or off the bearer's drawn wood"); the run returns here. The full lists are in the scratch directory (`inside-{base,after}-<tree>.txt`), not committed.

## Pins

- `pipeline::cycle_cost_tests::every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` (upstream, `telperion-space`'s grown structure, every preset at 16, seed 7) fails on the base commit in a separate worktree with the same values it fails with after: pre-existing, not moved by this change. Upstream unchanged: checked.
- `suite::rachis::the_axes_move_no_leaf` (the palm at 40, every leaf's transform): passes on base and after.
- `suite/species/digests.json` is read only by tests ignored under fn-198; not run.

## Not done because of the stop

R5's stills were not drawn. The full narrow lists of `pipeline` tests beyond `surface`, `fork`, `bark` and the two pins were not run.
