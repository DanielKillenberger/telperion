# A walk between leaf forms changes the leaves by degree

## Goal

fn-272's film (defects 2 and 5, 2026-10-14): walking from the Oregon white oak to the date palm at 40, the leaves went at once (871,168 at t = 0.0098, 4 at t = 0.0127, then a bare oak-shaped crown on a palm stem), and counts stepped 25-38 % between frames of spruce to birch and birch to oak. Every parameter must change the tree by degree, never switch between ways of building (AGENTS.md design principles). Find the rows behind the jumps and make the leaves change continuously along every walk. Release item (fn-237); walk sizes stay fn-273.

## Decisions

- Host (2026-10-14), ruling 1, fractional counts: `rosetteFronds`, `leafletCount`, `skirtFronds`, `leafBases`, `acanthophylls` and the engine's `cluster_leaves` walk unrounded; a fraction is split by a keyed draw (fn-248's `leaf_years` pattern); whole-valued presets byte-equal.
- Ruling 2, the rosette takeover: a new continuous row, `/canopy/cauline` (the share of leaf-bearing wood that keeps its leaves; dicots and conifers 1, the palm 0, FAO 156), replaces "any rosette frond makes the rosette the tree's only foliage".
- Ruling 3, leaflets: one law, no form switch; `leafletCount` walks on the log scale (fn-251 R1b); peak instances along oak to palm checked against the budget first.
- Ruling 5 (after the first review, replacing the per-bearer oracle): the oracle tests the law and the draw apart — (a) each form's expected count per bearer changes by at most a measured bound per 0.001 step of t over every film walk at seeds 1, 7, 42; (b) drawn counts lie within 4 sigma of their expectation, dormant under 30 organs.
- Ruling 6: the device draws the same law (keyed splits and cauline share in WGSL); no CPU placement route for fractional values.
- Host (2026-10-14): wood steps at oak to palm t 0.25-0.52 are the walk's structure and belong to fn-273.

## Acceptance Criteria

- **R1:** oak-to-palm leaf counts across t in [0.009, 0.013] and the two other jumps, standalone, at seeds 1, 7, 42, failing today — met (`tests/walk_leaves.rs`, red on 850eb039a).
- **R2:** the row or rule behind each jump checked against code — met: `leafletCount` 1 → 2 (t 0.004-0.005), `rosetteFronds` 0 → 1 switching the whole tree to fronds (`rosette::bearing`, t 0.0119), engine `cluster_leaves` 0 ↔ 1 on PAs 12-14 (spruce to birch t 0.167, birch to oak t 0.833).
- **R3:** leaves change continuously along every walk; oracle bound measured and stated; dormant where a form is absent — met (ruling 5).
- **R4:** film walk frames and fn-251's walk tests pass; stills of oak to palm viewed — met; the gate and `npm test` were not admitted in 30 minutes and run by the host on the merged branch.

## Result

Six counts and the cauline share walk unrounded, each bearer drawing the whole counts either side by one keyed 32-bit draw shared by CPU and device. `walk_law` sweeps every film walk at 0.001 steps: steepest step 0.0128 of a form's range against a stated bound of 0.02. `walk_leaves` holds draws within 4 sigma at the film's frames; every film walk passes except Laurelin to Telperion's draws (not admitted; the law shows nothing moving on it). Device `organ_tests` agree on the birch with 12 fronds, the oak with 1.3 leaflets and oak to palm at t 0.01, 0.3, 0.9. Render lib 191 passed. Codex SHIP after rulings 5 and 6.

## Open

- Wood steps on oak to palm at t 0.25-0.52 -> fn-273 (FILM-WALKS.md).
- The window tests take about 300 s of suite time.

## Friction

- First build and core runs waited on or were killed by `telperion-heavy` caps (8 GB) -> declare 3 GB for such builds; gate's own size for partial suites; nextest to resume by test.
- FILM.md cited a frame log path it did not name -> name cited paths.
- Waits for room with no queue position; gate and npm unadmitted in 30 minutes -> `telperion-heavy` prints queue position; host runs the gate on the merged branch.
- Inputs fetched from another worktree -> keep inputs in the worktree's own evidence directory.

## Evidence

- `.flow/evidence/fn-277-a-walk-between-leaf-forms-changes-the/FILM-WALKS.md` (every film walk under the oracle; the wood steps fn-273 takes)
