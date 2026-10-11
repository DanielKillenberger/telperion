# Council: the attack plan for light, unified and fast (2026-10-11)

Convened by the host at the owner's request ("Review the plan you have and decision with fable and astra council on it to find the most elegant solution that works in a unified way and is performant. Discuss and come to agreement on the attack plan"), after fn-255's R3 measured the light-volume candidate. Two rounds; the brief, both members' plans and both round-2 positions are in the session scratchpad (`council/`). Both members signed the plan below; where they still differ, the plan names the measurement that decides.

## Agreed

- **The light-volume candidate is refused** (fn-255 R3.md: 476 s against the lattice's 0.47 s for the oak at 80 at the setting that holds its form; the branching hierarchy visits up to 11% of nodes per bud).
- **The lattice is the one light model for every species**: 0.5 m, trilinear deposit, nine directions, the existing bilinear layer recurrence, read per bud in the one cycle. Species differ only by light-response rows, sourced; a coefficient may stay zero where its source says so. Non-zero rows are not a pass for fn-251's walk: its exact repro is a required test. Light settings that describe the site (extinction) are environment, not species, and are not interpolated by a walk.
- **One cycle, one phase order**: light → balance totals (wood upkeep included, not buds only) → shedding → surviving allocation totals → vigour split at ordered attachments → PA normalisation → fate and births → incremental layout. The executor is chosen per context, never per species.
- **The shed-certain shortcut (`Certain::Skip`) is not restored**: a lateral certain to be shed shades while it lives. If its cost is too high the lever is a species row (horizon, lifespan), never an engine switch.
- **One coarse record kind** for the site: `couple::Aggregate` as it stands (PA and developmental cohorts, a deposit stencil) is the first candidate, cut by world cell size; state it lacks (sleeping buds, internal shedding, light at more than its centroid) is added only where the aggregate-versus-expanded test measures an error. The lattice stays at 0.5 m, independent of the output grid, until that test passes.
- **fn-255 and fn-256 merge together, after combined acceptance.** The site stays on 0.1.4 until fn-257 qualifies; fn-259 keeps the lattice off its deletion list.
- **Deleted when replaced:** `Species::reads_light`, `grow::apart`, `keeps` as a gate, `lasting`/`Certain`, the sketch/balance/allot gates, `grow/alone.rs`, the stepped executor's handover and per-root settle, `settle_with` versus `lit`, `chain.wgsl`, `contract.wgsl`, `wide.wgsl`, the fine-root cut, the 1,024 constant; predictors only once no consumer remains.

## Decided by measurement (still open between the members)

- **GPU hierarchy schedule:** Fable's fixed dispatch count with ancestor walks against Astra's bounded subtrees in workgroups with synchronised boundaries; benchmarked on the busy-cycle replay with the measured depth, attachment counts and root contention.
- **Deposit:** integer atomics on weighted area at q = 2^-20 m² (u32 per node, overflow refused by name) first; ordered binned gathering if the quantum fails accuracy, continuity or capacity. Carbon and allocation sums are not quantised because deposition passed.
- **Sweep:** one workgroup per direction looping layers with barriers, against parallel layer dispatches and tiles with halos; the bilinear recurrence preserved in every form.

## Early tests, in order, with stop conditions

1. **Three-way CPU experiment** (spruce, Telperion; then every preset): F4 with `Certain::Skip`, F4 with `Certain::Grow`, and the cycle; same seeds, threads and profile; grown counts, per-phase times, peak memory, plus active-versus-history records, ancestor depth histograms, attachment counts, lattice dimensions and deposit occupancy. Stop: Telperion over the 20M budget under `Grow` → an owner decision on its ages' horizons before any port. The result sets fn-255's priority (extra growth → transient representation and aggregates; extra processing → traversal, layout, allocation).
2. **Quantised deposit**, frozen oak first (`Field::at` per bud against f32), then through feedback and viewed renders. Stop: overflow, fidelity or continuity fails → ordered gathering.
3. **Chrome dispatch-overhead probe**, then a busy-cycle replay of the oak at 80 (native and cold Chrome) covering both hierarchy schedules and the sweep candidates: compile, encoding, longest dispatch, scratch. Stop the full port if dependency overhead alone exhausts the build budget, correctness fails, or a dispatch causes device loss.
4. **Aggregate-versus-expanded subtree** (a feedback-sensitive oak subtree: light, normalisation, shedding over cycles at tightening cells), then the stepped oak at a 0.32 m cut in single-thread Wasm as a timing probe. Stop: fidelity needs nearly full expansion → the site cannot use this engine at 36³; the owner decides under R0.
5. **Complete small-tree integration**, then every preset and the exact fn-251 frame: canonical repeatability, CPU/GPU agreement, measured memory, completed-build latency, viewed fidelity.

## Decided by the spike (host, 2026-10-11; fn-256 SPIKE.md)

Early test 3 passed its stop condition: dependency overhead is at most about 14% of F4's warm build budget (Telperion 48-163 ms with bounded subtrees against 1.2 s), every integer output matched exactly, no device loss, longest dispatch 1.8 ms, cold compile of all cycle kernels about 0.35 s. On the measurement:

- **Hierarchy schedule: bounded subtrees.** They reproduce the CPU's f32 sums bit for bit at block caps 256-4,096, are faster on the GPU (4.5/5.8 ms against 5.9/6.3 ms a busy cycle), use about 12 B of scratch per axis against 56, and keep every dispatch under 0.5 ms (watchdog margin). Ancestor walks would need integer balance and allocation sums, which this plan does not quantise. The CPU cycle keeps its records in the same subtree order (fn-255), so both executors share one layout.
- **Reduction order is one contract:** every float sum (balance, allocation, PA normalisation) is taken in the canonical block order on both executors; the device's normaliser is that f32 sum, not an integer one.
- **Viability:** both executors read the same f32 table value; the CPU's f64 path goes.
- **Agreement standard:** integers exact; floats by lineage within tolerances measured from real runs (fn-254 R4 check 4), with no bit-forcing guards on fused multiply-adds or division.
- **Not yet replayed:** growth units and incremental layout on the device (Telperion's two largest CPU phases), and keeping blocks and ordered child lists current on the device as the tree grows; both are fn-256's first measurements after fn-255 lands.

## Round 3: the site, after early test 4 (2026-10-11)

Early test 4's stop condition fired (fn-257 SPIKE.md). Both members (`council/astra-3.md`, Fable's round 3) agree:

- `couple::Aggregate` fails as a feedback model: fidelity arrives only with nearly full expansion, and from 0.16 m down it costs more than growing everything (13.2-20.4 s against 10.8 s). Giving it internal shedding, a vigour spread and sleeping buds is a small grower per record; no member recommends building it, and any rescue is a separately bounded oracle experiment first (Astra), not plan.
- Growing everything on the CPU is about ten times the site's budget (oak 3,207 ms cold Wasm), and per-phytomer trimming cannot close it (fn-255 CYCLE-COST.md's floor).
- The closed-form tables are the price of the stands, not of the engine; deletion follows the stands. While they stay: remove the work no consumer reads, count the rows actually read, and pay them once per (species, age) per page. Storing rows in a preset would be baking a derived value; refused.
- The site stays on 0.1.4.

Where they differ, the host takes both as ordered next steps:

- **Early test 5 (Fable):** the stop condition fired on lattice deposits, which the site does not draw. Measure what the site draws: oak and birch at 40, seeds 1/1234/5678, cuts 1.0/0.5/0.32 m, the site's voxel field from the stands against the expanded tree's in `voxelize`, viewed stills of the voxel grove beside the expanded tree's and beside 0.1.4's, cold Wasm end to end at each cut. Stop: the owner cannot accept the 0.32 m grove beside the expanded tree's, or the acceptable cut costs more than the owner's "slightly less" against 0.1.4.
- **Table audit (Astra):** behaviour-preserving removal of table work no site consumer reads (the copied extinction curves, the reach bound the coarse settle disables), the rows actually read counted per seed, demand-driven rows measured against eager on cold oak, birch and ordinary; outputs unchanged. Stop optimising tables when the savings cannot close the budget; cheaper tables are never fidelity acceptance.

To the owner: whether the site may grow on the device (WebGPU), the only route on which it draws the same tree as the harness (fn-256's spike: cycle kernels compile cold in about 0.35 s; units and layout on the device unreplayed); whether the oak's 1.1-1.7× of 0.1.4 is "slightly less"; whether the ordinary ships on the site (its rows alone are 11.5 s).

## The owner's answers (2026-10-11) and early test 5

Early test 5 (fn-257 TEST5.md): what the site draws from the stands costs 1.7-3.4× 0.1.4 for the oak at a 0.32 m cut and 2.2-3.8× for the birch at every cut; the expanded CPU tree 2.3-3.7 s. The table audit (TABLES.md): demand-driven rows would bring the birch's rows near budget and the ordinary under 0.1.4, but cannot close the oak's.

The owner: "Yes, WebGPU on the site"; the oak "Must match 0.1.4"; the ordinary "Yes, it must ship". So the coarse-record route is closed and fn-257 is re-scoped: the site grows the full tree on the device through fn-256's executor, voxelised at its cell; stands, tables and predictor go where no other consumer remains; a browser without WebGPU gets a named error, never a CPU fallback. Its first measurement is a probe page the owner opens (device creation and cold compile against 0.1.4's 273 ms).

## The probe in Chrome (owner's machine, 2026-10-11)

Chrome 154, RTX 3080, Linux (fn-257 probe, runs 1 and 2):
- Device creation 134-181 ms; cold compile of the spike's 24 pipelines 65-367 ms, cached 14-16 ms. The owner accepts these as a once-per-page cost (fn-257 R3-0).
- Per dispatch over 640: indirect 429-432 µs (one pass or a pass each); direct 5.3-6.7 µs; 8 command buffers 5.5 µs; one looping dispatch 3.4 µs per step. The cost is Dawn's indirect-argument validation.
- Chrome rejected what naga accepted: a NaN constant from `bitcast`, a barrier under a loop bound Tint calls non-uniform, and 17 storage buffers in one stage against 16.

Host decisions: the cycle uses direct dispatches only, with host-known workgroup counts and in-shader guards; at most 16 storage buffers a stage (8 where cheap); Tint's constant and uniformity rules are design rules.
