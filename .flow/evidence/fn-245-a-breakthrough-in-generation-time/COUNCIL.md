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
