# Fork probe: the rim at the old oak's stem forks

Oregon white oak (`oregon-white-oak`), seed 1001, at ages 120 and 200. Crates of worktree `.worktrees/fork-probe` at `5033ed05a` (branch `fn-357-a-retired-root-keeps-only-what-is-read`, detached). These are measurements only. No product change is committed, and `git status` in the worktree is clean after the trial was reverted.

All radii and distances are in drawn metres. Ring numbers are read from `Expansion::curve()`, the points the sweep actually draws its rings on (centre, radius, distance along). They are not a re-derivation. Each ring is matched to a tree node by exact position. Surface params for the oak: `fork_socket 0.5`, `socket_containment 0.9`, `fork_swell 1.35`, `lobe_depth 0`, 5 lobes, 12 radial segments, no sections.

## 1. The forks

A fork here is a stand node with a radius over 5 cm that has a child marked `codominant`. "Followed" is the child the run carries on into, read off the curve. Note on width: `paths.rs` compares children by `start_radius`. The followed child's `start_radius` always equals the parent's radius, because it is the continuation node, while its own `radius` can be much smaller.

| age | forks | a marked sibling wider than the followed child by `start_radius` | the same by own `radius` | followed child itself marked (no unmarked child) |
|---|---|---|---|---|
| 120 | 47 | 0 | 3 | 0 |
| 200 | 33 (30 with a followed child found) | 0 | 4 | 3 |

Distribution of the ratio widest marked sibling / followed child:

| ratio | 120 by start_r | 120 by radius | 200 by start_r | 200 by radius |
|---|---|---|---|---|
| 0–0.5 | 29 | 18 | 20 | 11 |
| 0.5–0.75 | 16 | 21 | 6 | 13 |
| 0.75–0.9 | 2 | 3 | 2 | 1 |
| 0.9–1.0 | 0 | 2 | 2 | 1 |
| 1.0–1.1 | 0 | 1 | 0 | 0 |
| 1.1–1.5 | 0 | 2 | 0 | 2 |
| 1.5–2.0 | 0 | 0 | 0 | 1 |
| > 2 | 0 | 0 | 0 | 1 |

The largest forks at 200 (full tables for both ages are in `forks-u-120.txt` and `forks-u-200.txt`, summarised by `summ.py`). Each child is given as (start_r, r); a sibling also carries its weight and its angle to the followed child in degrees.

| rank | node | y m | parent r | followed (start_r, r) | marked siblings (start_r, r, weight, angle) |
|---|---|---|---|---|---|
| 0 | 105 | 4.72 | 0.664 | 106 (0.664, 0.473) | 120 (0.385, 0.385, 0.601, 24.7); 168 (0.273, 0.273, 0.036, 24.9) |
| 1 | 112 | 5.03 | 0.473 | 127 (0.473, 0.341) | 113 (0.333, 0.333, 1.0, 24.8) |
| 2 | 6352 | 12.80 | 0.236 | 6353 (0.236, 0.138) | 7679 (0.193, 0.193, 1.0, 27.9) |
| 3 | 11244 | 15.13 | 0.185 | 11245 (0.185, 0.160) | 12836 (0.079, 0.079, 0.09, 34.7) |
| 4 | 6357 | 12.89 | 0.138 | 6358 (0.138, 0.117) | 7660 (0.063, 0.063, 0.186, 34.8) |
| 7 | 10561 | 14.39 | 0.117 | 12045 (0.117, 0.056) | 12053 (0.105, 0.105, 1.0, 31.0) |
| 10 | 17683 | 15.79 | 0.103 | 17684 (0.103, 0.016) | 19334 (0.103, 0.103, 1.0, 31.0) |

At 120 the same two trunk forks lead: node 105 (parent 0.534; siblings 0.274 and 0.248) and node 112 (parent 0.386; sibling 0.235). The 0.616/0.31 sibling-wider-than-primary case the survey reported at 300 does not occur at the big forks at 120 or 200.

## 2. Rings at the three largest forks at 200 (unchanged code)

**Fork 0 (node 105, y 4.72, r 0.664) and fork 1 (node 112, 0.315 m above it, r 0.473): one trunk run passes through both.**

| d from node 105 (m) | ring r | change from ring below | node |
|---|---|---|---|
| −2.65 | 0.706 | | 46 |
| −0.45 | 0.666 | | 95 |
| 0.000 | 0.665 | 0.000 | 105 (fork 0) |
| +0.180 | 0.655 | −0.006 | 109 |
| +0.315 | 0.6375 | −0.010 | 112 (fork 1) |
| +0.365 | 0.5874 | **−0.079** | 127 |
| +0.415 | 0.5373 | **−0.085** | 128 |
| +0.465 | 0.4872 | **−0.093** | 129 |
| +0.515 | 0.4577 | −0.061 | 130 |
| +0.565 | 0.4501 | −0.017 | 131 |
| +0.665 | 0.432 | | 133 |
| +1.265 | 0.341 | | 145 |
| +2.165 | 0.315 | | 163 |

Between d 0.315 and 0.515 the ring radius falls by 0.180 m over 0.200 m of length: a slope of exactly 1.00 (0.050 m per 0.050 m ring). That is the 1:1 floor in `girths` (`under − distance`). No single ring drops by more than 10% (the largest fall is 9.3%). The step is a 45° cone, 18 cm deep, on a trunk of radius 0.64.

What the numbers show (a reading of `samples.rs::girths`, not tested separately): the easing from fork 0 holds the girth near 0.64 up to node 112. Node 112 is itself a fork (`forks[112]`), so the easing restarts from `nodes[112].radius` = 0.473, not from the drawn 0.6375. The eased girth at 127 is then below `0.6375 − 0.05`, and the floor holds it to the 1:1 slope until it meets the new easing. The next fork lies 0.315 m up, inside the first fork's 1.33 m diameter.

At 120 the same thing happens at the same nodes: 0.503 at node 112 falls to 0.453, 0.403 and then 0.378 at d +0.05, +0.10 and +0.15 above 112 (falls of 9.9%, **11.0%** and 6.2%). That is 12.5 cm over 15 cm, slope 1.0 at its first two rings.

Siblings at forks 0 and 1. `k` is the ring index, `d` is metres along from the fork node, `h` is height along the trunk axis from the fork node, and `out` is ring centre to run axis + ring r − run r there (positive means the ring pokes out of the run's surface).

| sibling | sink | contained | k0 (socket) h / r | k1 h / r | run r at k1 | out k0..k5 |
|---|---|---|---|---|---|---|
| 120 at 105 (w 0.60) | 0.332 | 0.566 | −0.302 / 0.521 | +0.030 / 0.514 | 0.665 | −0.004, −0.136, −0.128, −0.118, −0.108, −0.104 |
| 168 at 105 (w 0.04) | 0.332 | 0.566 | −0.301 / 0.369 | +0.029 / 0.364 | 0.665 | −0.160 … −0.245 |
| 113 at 112 (w 1.0) | 0.236 | 0.403 | −0.215 / 0.403 | +0.026 / 0.443 | 0.609 | −0.161, −0.157, −0.126, −0.094, −0.062, −0.030 |

At these two forks every sibling ring stays inside the trunk run. The siblings leave nearly coaxially (lateral offset 1.4 cm at k1, angle 25°) and are narrower than the trunk (0.51 against 0.665). Sibling 113's margin closes at 3 cm per ring, so it meets the run's surface about 0.15 m above node 112, which is where the 1:1 shoulder runs.

**Fork 2 (node 6352, y 12.80, r 0.236), with fork 4 (node 6357) 0.117 m up the same run:**

| d | ring r | change |
|---|---|---|
| 0.000 | 0.2365 | 0 |
| +0.117 | 0.2213 | −0.022 (node 6357, fork 4) |
| +0.140 | 0.1984 | **−0.103** |
| +0.163 | 0.1755 | **−0.115** |
| +0.186 | 0.1526 | **−0.130** |
| +0.209 | 0.1329 | **−0.130** |
| +0.232 | 0.1304 | −0.018 |

This is the same pattern: 8.8 cm lost over 9.2 cm at slope 1.00, with four rings each dropping more than 10%.

Sibling 7679 (weight 1.0): sink 0.118, contained 0.201, socket ring r 0.201. Its first ring on its own node is **0.2536**, while the run at the same distance is **0.2356**: the sibling's ring is wider than the run's. `out` over k0..k5 is +0.020, +0.030, +0.038, +0.047, +0.059, +0.071, so the sibling's tube stands proud of the run surface from its socket ring on. That ring is 0.193 × swell (about 1.31) = 0.254, which exceeds the parent's 0.2365.

Across all forks: the steepest ring-to-ring fall slope in the run within two fork diameters is 1.00 at 6 of the 80 forks (both ages). Every one is a fork with a second fork within the first's easing window (node 105 at both ages; 6352 at 200), or the second fork itself, whose run is the same run. The other forks have slopes ≤ 0.75 (0.75 at 120 node 67329, with a fork 0.117 up), and most are ≤ 0.3. `STEP` lines in the `forks-*.txt` files list these.

**Gaps between surfaces:** none. Every socket ring found sits inside its parent run (`out` at k0 ≤ 0 everywhere except sibling 7679's +0.020, and that ring is still centred inside the run).

## 3. Stills (unchanged), largest fork at 200

The camera looks level at (−0.07, 4.9, 0.016), between nodes 105 and 112, from 4.0 m (three diameters of the 1.33 m fork). It is leafless (`View::Bare`), with field of view 38°. Az 0 is on the hero camera's side and az 90 is 90° round from it. Files: `u-200-fork0.png`, `u-200-fork90.png`.

- **Az 0:** the trunk rises as a plain cylinder and ends in a flat, rounded-edged top, a ledge that runs right round it, about a quarter of the trunk's width deep on each side. Above it several narrower stems rise out of the top surface. The ledge is the strongest feature in the frame and matches the description of a flat rim.
- **Az 90:** the same flat-topped trunk with the same ledge all the way round. Three or four stems fan out of the top at about 25°.
- In both views the stems above the ledge carry regularly spaced dark bands across them, like seams or rings. Their cause was not measured and is unknown. They are not the subject of this probe.

The ledge sits at the height of nodes 105 to 130 (y 4.7 to 5.2). By the ring numbers this is the 1:1 shoulder from r 0.6375 to 0.4577 above node 112. The siblings rise from inside it, narrower than the trunk (0.51 and 0.36 against 0.64), so they do not cover the annulus the shoulder leaves.

## 4. Trial: the run carries on into the widest child whatever its mark

The edit in `surface/paths.rs` was `} else if wider(leaders[at]) {` changed to `}` followed by `if wider(leaders[at]) {` (forks are still flagged). It has been reverted by hand, and `git status --short` is empty.

- The fork tables and every ring printed for ages 120 and 200 are **byte-identical** to the unchanged code (`cmp forks-u-*.txt forks-t-*.txt`).
- The stills `t-200-fork0.png` and `t-200-fork90.png` are **byte-identical** to the unchanged stills.

The reason: by `start_radius`, which `paths.rs` compares, the followed continuation node always equals the parent radius. No marked sibling is wider at either age, so the trial never changes a leader on these trees. By own `radius` a marked sibling is wider at 3 (120) and 4 (200) forks, but none of them is among the three largest. A trial comparing by own radius was not run, because it would be a different rule.

- `telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --lib surface` under the trial: 54 passed, **2 failed**, 1 ignored. The failures are both hand-built trees in `fork_tests.rs`:
  - `the_trunk_carries_on_into_the_primary_not_the_wider_sibling` (line 102)
  - `the_trunks_girth_eases_into_the_stem_over_the_forks_diameter` (line 146)

## 5. Verdict on the rim

The unchanged geometry explains a ledge at the position seen in the stills. The numbers point to the 1:1 shoulder that forms where a second fork lies inside the first fork's easing window (0.315 m above at the trunk fork; the window is 1.33 m), not to the codominant-primary choice. The trial leaves the geometry unchanged, so the leader rule is not the cause on this tree. Whether the rim's look comes only from that shoulder, or also from the siblings being narrower than the trunk and leaving the annulus uncovered, is not separated by these measurements: unknown. Whether the integration branch `goal/fn-321-…` shows the same rings was not measured: unknown. Sibling 7679 at node 6352 is wider than its parent run from its first ring (out +0.02 to +0.07 m). That is a second, smaller junction artefact, and it was not rendered.


## 6. Trial A: the ease past a fork carries on from the run's drawn girth

The edit was in `surface/samples.rs` `girths`. The ease past a fork starts from the run's drawn girth at the fork node (the value `below` holds for it, after any earlier ease and the 1-in-1 floor), not from `nodes[f].radius`, and it runs over twice that drawn girth. The floor was not changed. The trial has been reverted by hand.

**Trunk run at 200** (d from node 105):

| d | unchanged r | trial A r |
|---|---|---|
| 0.000 (node 105) | 0.6651 | 0.6651 |
| +0.315 (node 112) | 0.6375 | 0.6375 |
| +0.365 | 0.5874 (−7.9%) | 0.6361 (−0.2%) |
| +0.515 | 0.4577 | 0.6177 |
| +0.665 | 0.4320 | 0.5824 |
| +0.965 | 0.3715 | 0.4844 |
| +1.265 | 0.3409 | 0.3885 |
| +1.565 | 0.3251 | 0.3254 |
| +1.865 | 0.3155 | 0.3155 |

Under trial A the trunk run eases from 0.6375 to 0.325 over about 1.25 m. The largest ring-to-ring fall is 3.7% and the steepest slope is 0.51, where the unchanged run has a 1.00 slope over 0.20 m.

**Trunk run at 120** (d from node 112): unchanged 0.503 → 0.453 → 0.403 → 0.378 at +0.05, +0.10 and +0.15 m. Trial A: 0.503 → 0.502 → 0.498 → 0.491 → 0.483 → 0.473 (+0.25) → 0.448 (+0.35) → 0.362 (+0.65) → 0.307 (+0.95). The largest fall is 3.7% and the steepest slope 0.29.

**Third fork (node 6352, with fork 6357 at +0.117 m) at 200:**

| d | unchanged r | trial A r |
|---|---|---|
| +0.117 | 0.2213 | 0.2213 |
| +0.140 | 0.1984 (−10.3%) | 0.2205 (−0.4%) |
| +0.186 | 0.1526 (−13.0%) | 0.2146 |
| +0.232 | 0.1304 | 0.2040 |
| +0.342 | 0.1191 | 0.1680 (−4.4%) |
| +0.386 | 0.1172 | 0.1528 (−4.7%) |
| +0.508 | 0.1172 | 0.1212 |
| +0.629 | 0.1172 | 0.1172 |

**All forks** (within two fork diameters above each fork):

| | unchanged 120 | unchanged 200 | trial A 120 | trial A 200 |
|---|---|---|---|---|
| forks whose steepest slope is 1.00 | 2 | 4 | **0** | **0** |
| largest ring-to-ring fall | 23.4% (node 67334) | 38.2% (node 17683) | 12.7% (node 125737) | 38.2% (node 17683) |
| steepest slope | 1.00 | 1.00 | 0.60 (node 252) | 0.63 (node 17683) |

So the count at slope 1.00 goes from 6 of 80 to 0 of 80. The 38.2% fall at node 17683 is the same under both rules. That fork's followed child has an own radius of 0.016 against a sibling of 0.103, and it is not a second fork inside an ease. Every other fork outside the two trunk forks and 6352/6357 is unchanged, apart from 120 node 67329/67334 (0.75 slope, 23.4% fall unchanged; 0.32 slope, 12.5% fall under A).

**Sibling 7679 under trial A:** byte-identical to the unchanged code. Its socket ring is 0.2014, its first node ring is **0.2536** against the run's 0.2356 at the same d, and `out` over k0..k5 is +0.020, +0.030, +0.038, +0.047, +0.059, +0.071. A sibling's run starts at its own base (`starts`), so trial A does not touch it.

**Stills** (`a-200-fork0.png` and `a-200-fork90.png`; sheet `fork-sheet2.jpg`, unchanged left and trial A right):

- **Trial A, az 0:** the flat ledge is gone. The trunk narrows smoothly into the stem above in a long taper. The left sibling (120) now shows as a separate cylinder whose rounded lower end sits against the trunk's surface; that end was hidden behind the ledge before.
- **Trial A, az 90:** a smooth taper with the stems fanning out of it, and no flat top.
- **Both trial A views:** a thin dark band runs round the trunk at image row about 325. That is about y 5.03, node 112, where the drawn radius no longer steps (0.6375 to 0.6361). See section 7.

**Surface tests under trial A** (`TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --lib surface`): 56 passed, **0 failed**, 1 ignored.

## 7. The dark bands round the stems

**Method:** a probe subcommand (`probe stem`) printed every swept ring of the trunk run from node 95 and of the three sibling runs (120, 113 and 168) for 4 m. For each ring it prints the tree node's `radius`, `start_radius` and `base_radius`, and its engine phytomer, found by its lineage key through a second `telperion_space::grow` of the same request: axis, origin kind, axis birth, PA, growth unit (the phytomer's cycle) and engine radius. It also prints whether a run starts at the ring and whether a curve cluster starts there, and the ring centre projected into the az 0 still (eye 3.584, 4.9, −1.612, target −0.07, 4.9, 0.016, fov 38°, 960×720). Raw output is in `bands-200.txt`.

**Band positions:** the left stem in the unchanged az 0 still is sibling 120. One cropped view (`crop-u0.png`, marks at projected rows 81, 254 and 312) shows two bands on it, centred at image rows about 75–100 and 250–285, tilted with the stem.

**What coincides with each band:**

| band (az 0 row) | ring | drawn r step | node radius step | engine | other |
|---|---|---|---|---|---|
| ~254 | 120 k8, d 0.578, y 5.28 | 0.4841 → 0.4393, **−9.3%** over a 0.40 m segment | none (0.3853 both) | new axis: axis 3 (lateral) → axis 7 (**relay**, born cycle 14), unit 14 → 17 | not a run start, not a cluster start |
| ~81 | 120 k26, d 1.197, y 5.86 | 0.4074 → 0.3911, **−4.0%** in one 3.3 cm segment | 0.3851 → 0.3706 (−3.8%) | same relay axis 7; **growth unit 19 → 20** | not a run start, not a cluster start (the cluster starts at k31, row 31, where there is no band) |

The same pattern holds on every stem printed. Within an axis the node radius is constant (to 4 decimals) over 10 to 13 rings, then steps down 3 to 6.5% in one ring at a growth-unit boundary, about every two units:

- run 120: steps at units 20, 22, 27 and 29, by −4.0%, −3.9%, −4.5% and −4.4%
- run 113: units 20, 22, 25, 30 and 32, by −4.3%, −3.8%, −3.2%, −2.9% and −4.8%
- run 168: units 18, 27 and 29, by −5.7%, −5.6% and −6.5%
- trunk above the fork: units 18, 19, 21, 22, 25 and 26, by −4.6%, −2.9%, −4.0%, −3.4%, −5.4% and −3.4%

**Where the step comes from:** at every step the tree node's radius equals the engine phytomer's radius times the scale (`eng_r` = `n_r` in every row). So the stepped radii are the ones the pipeline is handed from `telperion-space`'s girth, not anything the surface stage made. The surface adds only the branch run's fork swell, which decays smoothly (sibling 120's drawn r runs 0.514 → 0.407 between steps while the node r stays 0.385). The 1-in-1 floor does not smooth a step of 0.016 m over 0.033 m (slope 0.49). Which rule in `girth.rs` produces radii that are constant across a unit and then step was not read: unknown.

**Not a seam:** none of the band rings is a run start, so the bark coordinate `along` is continuous across them. Curve cluster boundaries (k31 on run 120, row 31) do not coincide with a band. Mesh chunk boundaries were not measured: unknown.

**Not explained:** the band trial A leaves on the trunk at node 112 (az 0 row ~325). There the drawn radius changes by only −0.2% and −0.6% per ring. The ring at node 127 is an engine axis boundary: axis 1 (continuation, PA 3, unit 13) to axis 4 (continuation, PA 4, unit 15). Whether that band comes from a change of direction of the run's centreline there, from shading, or from something else was not measured: unknown.

Node radii along sibling 120 (all rings to 3.1 m; px row is the projected ring centre in the az 0 still):

| k | d m | y m | drawn r | change | node | node r | start_r | base_r | engine axis | origin | unit | engine r | px row |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 0.000 | 4.749 | 0.5143 | -0.013 | 120 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 400 |
| 2 | 0.033 | 4.779 | 0.5080 | -0.012 | 121 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 392 |
| 3 | 0.066 | 4.809 | 0.5020 | -0.012 | 122 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 384 |
| 4 | 0.100 | 4.839 | 0.4964 | -0.011 | 123 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 376 |
| 5 | 0.111 | 4.849 | 0.4944 | -0.004 | 124 | 0.3852 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3852 | 373 |
| 6 | 0.144 | 4.880 | 0.4892 | -0.011 | 125 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 365 |
| 7 | 0.177 | 4.910 | 0.4841 | -0.010 | 126 | 0.3853 | 0.3853 | 0.3853 | 3 | lateral | 14 | 0.3853 | 357 |
| 8 | 0.578 | 5.284 | 0.4393 | -0.093 | 228 | 0.3853 | 0.3853 | 0.3853 | 7 new | relay | 17 | 0.3853 | 254 |
| 9 | 0.612 | 5.316 | 0.4366 | -0.006 | 229 | 0.3853 | 0.3853 | 0.3853 | 7 | relay | 17 | 0.3853 | 245 |
| 10 | 0.646 | 5.348 | 0.4340 | -0.006 | 230 | 0.3853 | 0.3853 | 0.3853 | 7 | relay | 17 | 0.3853 | 236 |
| 11 | 0.681 | 5.379 | 0.4316 | -0.006 | 231 | 0.3853 | 0.3853 | 0.3853 | 7 | relay | 17 | 0.3853 | 227 |
| 12 | 0.715 | 5.411 | 0.4292 | -0.005 | 232 | 0.3852 | 0.3853 | 0.3853 | 7 | relay | 17 | 0.3852 | 217 |
| 13 | 0.749 | 5.443 | 0.4269 | -0.005 | 233 | 0.3851 | 0.3852 | 0.3853 | 7 | relay | 17 | 0.3851 | 208 |
| 14 | 0.784 | 5.475 | 0.4247 | -0.005 | 234 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 18 | 0.3851 | 199 |
| 15 | 0.819 | 5.507 | 0.4227 | -0.005 | 235 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 18 | 0.3851 | 189 |
| 16 | 0.854 | 5.539 | 0.4208 | -0.005 | 236 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 18 | 0.3851 | 179 |
| 17 | 0.888 | 5.572 | 0.4190 | -0.004 | 237 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 18 | 0.3851 | 170 |
| 18 | 0.923 | 5.604 | 0.4173 | -0.004 | 238 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 18 | 0.3851 | 160 |
| 19 | 0.957 | 5.636 | 0.4156 | -0.004 | 239 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 150 |
| 20 | 0.992 | 5.667 | 0.4141 | -0.004 | 240 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 140 |
| 21 | 1.026 | 5.699 | 0.4126 | -0.004 | 241 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 131 |
| 22 | 1.061 | 5.731 | 0.4112 | -0.003 | 242 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 121 |
| 23 | 1.095 | 5.763 | 0.4099 | -0.003 | 243 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 111 |
| 24 | 1.130 | 5.795 | 0.4087 | -0.003 | 244 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 101 |
| 25 | 1.164 | 5.826 | 0.4074 | -0.003 | 245 | 0.3851 | 0.3851 | 0.3853 | 7 | relay | 19 | 0.3851 | 91 |
| 26 | 1.197 | 5.857 | 0.3911 | -0.040 | 246 | 0.3706 | 0.3851 | 0.3853 | 7 | relay | 20 | 0.3706 | 81 |
| 27 | 1.230 | 5.888 | 0.3901 | -0.003 | 247 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 71 |
| 28 | 1.264 | 5.918 | 0.3892 | -0.002 | 248 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 61 |
| 29 | 1.297 | 5.949 | 0.3883 | -0.002 | 249 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 51 |
| 30 | 1.330 | 5.980 | 0.3874 | -0.002 | 250 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 41 |
| 31 | 1.363 | 6.010 | 0.3866 | -0.002 | 251 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 31 |
| 32 | 1.396 | 6.041 | 0.3858 | -0.002 | 252 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 20 | 0.3706 | 21 |
| 33 | 1.430 | 6.072 | 0.3850 | -0.002 | 253 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | 10 |
| 34 | 1.464 | 6.103 | 0.3843 | -0.002 | 254 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | 0 |
| 35 | 1.497 | 6.133 | 0.3837 | -0.002 | 255 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | -10 |
| 36 | 1.531 | 6.164 | 0.3830 | -0.002 | 256 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | -21 |
| 37 | 1.565 | 6.195 | 0.3824 | -0.002 | 257 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | -32 |
| 38 | 1.598 | 6.226 | 0.3818 | -0.002 | 258 | 0.3706 | 0.3706 | 0.3853 | 7 | relay | 21 | 0.3706 | -42 |
| 39 | 1.632 | 6.257 | 0.3668 | -0.039 | 259 | 0.3565 | 0.3706 | 0.3853 | 7 | relay | 22 | 0.3565 | -53 |
| 40 | 1.665 | 6.287 | 0.3663 | -0.001 | 260 | 0.3565 | 0.3565 | 0.3853 | 7 | relay | 22 | 0.3565 | -64 |
| 41 | 1.699 | 6.318 | 0.3658 | -0.001 | 261 | 0.3565 | 0.3565 | 0.3853 | 7 | relay | 22 | 0.3565 | -75 |
| 42 | 1.732 | 6.348 | 0.3653 | -0.001 | 262 | 0.3565 | 0.3565 | 0.3853 | 7 | relay | 22 | 0.3565 | -86 |
| 43 | 1.766 | 6.379 | 0.3649 | -0.001 | 263 | 0.3565 | 0.3565 | 0.3853 | 7 | relay | 22 | 0.3565 | -96 |
| 44 | 1.799 | 6.409 | 0.3642 | -0.002 | 264 | 0.3562 | 0.3565 | 0.3853 | 7 | relay | 22 | 0.3562 | -108 |
| 45 | 1.834 | 6.441 | 0.3638 | -0.001 | 265 | 0.3562 | 0.3562 | 0.3853 | 7 | relay | 23 | 0.3562 | -119 |
| 46 | 1.868 | 6.473 | 0.3634 | -0.001 | 266 | 0.3562 | 0.3562 | 0.3853 | 7 | relay | 23 | 0.3562 | -131 |
| 47 | 1.903 | 6.504 | 0.3630 | -0.001 | 267 | 0.3562 | 0.3562 | 0.3853 | 7 | relay | 23 | 0.3562 | -142 |
| 48 | 1.937 | 6.536 | 0.3625 | -0.001 | 268 | 0.3561 | 0.3562 | 0.3853 | 7 | relay | 23 | 0.3561 | -154 |
| 49 | 1.972 | 6.568 | 0.3622 | -0.001 | 269 | 0.3561 | 0.3561 | 0.3853 | 7 | relay | 23 | 0.3561 | -166 |
| 50 | 2.350 | 6.889 | 0.3595 | -0.008 | 1145 | 0.3560 | 0.3561 | 0.3853 | 30 new | relay | 25 | 0.3560 | -305 |
| 51 | 2.384 | 6.918 | 0.3593 | -0.000 | 1146 | 0.3560 | 0.3560 | 0.3853 | 30 | relay | 25 | 0.3560 | -319 |
| 52 | 2.758 | 7.289 | 0.3433 | -0.045 | 1656 | 0.3415 | 0.3560 | 0.3853 | 45 new | relay | 27 | 0.3415 | -455 |
| 53 | 3.128 | 7.655 | 0.3283 | -0.044 | 2008 | 0.3273 | 0.3415 | 0.3853 | 61 new | relay | 29 | 0.3273 | -563 |

## Commands

```
S=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fork
# probe: the fn-352 scoreboard probe copied to $S/probe, crates pointed at .worktrees/fork-probe,
# plus src/forks.rs (`probe forks <preset> <age> <seed> <detail>`) and FORK_CAM shots fork0/fork90 in stills.rs
telperion-heavy 8 --threads 16 -- cargo build --profile ci --manifest-path $S/probe/Cargo.toml --target-dir $S/target
telperion-heavy 16 --threads 8 -- bash $S/forks.sh u 120 200          # t for the trial
python3 -I $S/summ.py $S/forks-u-200.txt
telperion-heavy 16 --gpu 2 --threads 8 -- env FORK_CAM=-0.070,4.9,0.016,4.0 \
  $S/target/ci/scoreboard-probe tree oregon-white-oak 200 1001 $S/u-200 fork0,fork90
montage ... -tile 2x2 $S/fork-sheet.jpg      # and fork-sheet2.jpg
TELPERION_HEAVY_RESERVE=8 telperion-heavy 16 --threads 8 -- bash $S/forks.sh a 120 200   # trial A (trialA.sh)
TELPERION_HEAVY_RESERVE=8 telperion-heavy 16 --threads 8 -- env FORK_EYE=3.583922,4.9,-1.611530,-0.07,4.9,0.016 \
  $S/target/ci/scoreboard-probe stem oregon-white-oak 200 1001 4.0 95 120 113 168      # bands.sh
```

## Friction

- 12 Nov 2026: the shell hook refused a `sed > file` into the scratch probe (dynamic redirect path), and `sleep` waiting was blocked. Each cost about a minute. Nothing else.
- 12 Nov 2026, evening: the first band run (`telperion-heavy 20`) waited over an hour for room on the shared machine and was then killed before it started. It was rerun on 13 Nov with `TELPERION_HEAVY_RESERVE=8` at 16 GB and finished. About 70 minutes of wall time were lost, with no work done in them.
- The trial A forks/stills script ran before the reserve rule existed. Trial A's stills and fork runs came from it, and finished.

## End state

The trial in `surface/samples.rs` was reverted by hand. `git -C .worktrees/fork-probe status --short` prints nothing: no product change is kept, and nothing was committed, pushed or stashed.
