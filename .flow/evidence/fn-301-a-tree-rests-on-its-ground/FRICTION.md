# fn-301 friction

## 2026-10-21 00:00, conductor (flow --auto), selection

- **Doing:** the pilot snapshot for fn-301 on its branch.
- **Slowed by:** the snapshot marks fn-301 ineligible: "parent branch fn-283-a-frond-draws-its-rachis not on origin". The branch already carries fn-283 (merged at d600fa12c), so the chain predicate's premise does not hold here; the run went ahead on the host's instruction.
- **Cost:** about 2 minutes.
- **Would remove it:** a chain predicate that accepts a parent already merged into the child's branch head.

## 2026-10-21 00:05, conductor, the unknowns

- **Doing:** checking the spec's "Unknown" section against the code.
- **Slowed by:** the spec names the dead skirt as fn-204's leaf bases; it is the rosette's dead fronds (`skirtFronds`, `skirtPitch` 165), drawn as fronds with `dead = true`. `docs/pipeline.md` still says four families have no engine species; all eight have `/space` rows.
- **Cost:** one exploration pass, about 5 minutes.
- **Would remove it:** the spec and the doc corrected (reported to the host).

## 2026-10-21 00:20, conductor, the R1 repro on the base

- **Doing:** the R1 probe (`tests/ground_reach.rs`, 168 builds) through `telperion-heavy 8`.
- **Slowed by:** the first run counted the trunk's ring at the root, which leans with the trunk, as wood below the ground on every preset; a second run left it out. Each run waited 1.5 to 3 minutes for admission and took 2.5 minutes.
- **Cost:** about 12 minutes, one rerun.
- **Would remove it:** a wood bound per run that names the trunk's foot, so a ground probe need not rebuild that rule.

## 2026-10-21 01:00, conductor, the plan

- **Doing:** the plan skill's preflight.
- **Slowed by:** `flowctl init` wrote `review.fanoutExecution` into the tracked `.flow/config.json`. The guard blocks `git restore`, so the line was removed by hand.
- **Cost:** about 2 minutes.
- **Would remove it:** an `init` that leaves an existing config untouched.

## 2026-10-21 01:15, conductor, plan review round 1 (codex)

- **Doing:** the codex plan review.
- **Slowed by:** the review found that the drawn bark is wider than the radius the design rests on (`flareRadius` up to 2 at the ground, lobes 1 + `lobeDepth`), that the renderer enlarges thinned leaves after the pose, and that 10-bit quaternions move a 1 m leaflet tip by about 1.7 mm. The first two are design questions, so the run went back to the host.
- **Cost:** one review round (about 8 minutes) and a stop.
- **Would remove it:** a design note on "radius" that names the drawn reach, and a renderer note on thinning, written before the plan.

## 2026-10-21 02:00, conductor, plan review round 3

- **Doing:** re-reviewing the plan after the round-2 fixes (a per-vertex quantisation bound, the inflorescence posed as one, .4 after .3).
- **Slowed by:** `review.maxIterations` is 2, so flowctl refused the third round with `ESCALATE` before the reviewer saw the fixes. Round 1 spent one of the two rounds on questions only the host could answer.
- **Cost:** one stop and a hand-off. The fixes wait unreviewed.
- **Would remove it:** not counting a round that ends in a host ruling against the cap, or a cap of 3 for plans.

## 2026-10-21 02:30, conductor, plan review round 3 (after the reset)

- **Doing:** the codex re-review of the round-2 fixes.
- **Slowed by:** the inflorescence pose cannot reach the renderer. `shaders/unit.wgsl:61-75` droops an inflorescence's rachis toward world ±Y from its seat and heading alone, so turning the instance does not turn the drawn rachis. No shipped preset's flowers come within 0.44 m of the ground (R1 on the base). How to represent a rested inflorescence is a design question, so the run went back to the host.
- **Cost:** one review round and a stop.
- **Would remove it:** a design note on flowers, written before the plan, saying whether flower contact is in scope for a preset with no low flowers.
