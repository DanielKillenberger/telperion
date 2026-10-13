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

## fn-256 task 2 rulings (host, 2026-10-12)

- "Integers exact" means identity integers (lineage keys, canonical order, slot allocation, births, host-read counts). Decisions computed from floats may differ between executors within the lineage agreement tolerances (fn-254 R4 check 4); the device runs plain f32 arithmetic, with no exact mode (it cost 34% GPU time and a 96 s cold compile) and no subnormal emulation.
- Cold compile of the whole cycle is a budget: under 2 s native with the disk cache off (the units module was 37.7 s), then measured in Chrome by the owner's probe.
- Every float reduction, the settle's leaf mean included, is the canonical blocked reduction on both executors.
- The device refuses trees of 2,048 cycles or more by name.

- Storage buffers (host, 2026-10-12, from fn-266): the native test device allows 8 storage buffers a stage (WebGPU's guaranteed minimum), so every stage is designed for 8, not 16.

## Round 4: the device cycle against F4 (2026-10-12)

Both members (`council/astra-4.md`, Fable's round 4) agree:
- The design fails "at or under F4's warm build" today (Telperion 1,909 ms vs 671) and parity on the Two Trees is not reachable by memory and scheduling alone: the cycle grows 18.2M phytomers where F4 grew 4.5M and couples every bud each cycle. Estimate after the fixes below: Telperion about 1.0-1.3 s (unmeasured). Beech and spruce share the cause; birch and ordinary near parity; the oak 2.2x faster.
- Memory is sizing policy, not the tree: retirement runs on the device but rows are let go only at a table move, and each move holds old and new tables at once, which sets the 6.1 GB peak.
- The four hierarchy sweeps are inherent; keep bounded subtrees and the ruled order (no DFS prefix sums: they change the arithmetic and cancel badly).
- A native device budget is required; refusal before any submission.

Ordered steps, host-adopted:
1. **Guard:** a native device budget (configured, conservative default), every allocation reserved before creation including replacement overlap, charges held until GPU completion, every kernel that reads a count clamps it to capacity, a dropped build polled free before the next allocates; tests that a build past budget submits nothing and that ten builds on a card with 2 GB held end in refusals, never a reset.
2. **Labelled peak census** at the peak, then the **live set per cycle** (rows not retired) on Telperion 82 and the oak at 80.
3. **A fixed arena sized once from the measured live-set bound, compacted in place on a cadence** (one column of scratch), refused by name before cycle 1 past it; moves and restarts deleted. Gate: zero moves, peak at the bound, no reset in ten runs on a two-thirds-full card.
4. **Dispatches:** re-fuse the units kernels to a handful per round (cold compile stays under 2 s); collapse the hierarchy's narrow upper levels into one barrier-looped workgroup (fringe and skeleton), words identical across caps; explain light's 46 a cycle. Gate: at most 340 a cycle.
5. Re-measure every preset against F4 and put the remaining ratio to the owner under R0, with the dead-term question (Fable): the Two Trees' shed laterals shade nothing while they carry no leaf area, so skipping them changes no bit today, but fn-264 gives them leaf area, so it would lapse.
6. The owner's Chrome probe, cold and warm, on the result.

## Round 5: the breakthrough against the roofline (2026-10-12)

ROOFLINE.md (fn-256 task 7): every phase but compaction runs 6-270x above its roofline time; unlit Telperion spends 255 ms (19%) on lattice kernels that exit at once; 71% / 82% / 59% of balance, allotment and retirement visits (Telperion / oak / beech) are on subtrees where nothing grew; laterals living under 5 years carry 62-76% of grown phytomers.

Both members agree (`council/astra-5.md`, Fable's round 5):
- Ranked: dormancy at zero (exact), incremental upward sums (exact), kernel shape (exact), reuse (exact; the game's and the retune's lever, not the release bar's), the short-lived statistical record (a model change; not built without the owner authorising a bounded fidelity experiment).
- Incremental propagation keeps the ruled order: a dirty axis recomputes from its own value then every child's cached or recomputed value in canonical order; never subtract-and-add a float. Downward passes (walk with its keyed draw, split by incoming vigour and rank) are not skipped on an unchanged upward sum. PA normalisation's 1,024-bud blocks are invalidated by changed membership or rank.
- Estimate after the engineering levers: Telperion about 1.2-1.5 s warm (only the lattice's 255 ms is directly isolated). F4's 671 ms is the interim executor; 0.1.4's Telperion built in 623 / 85 ms (seeds 1 / 42) as a different, far sparser tree.
- Reuse has a dependency: retirement reads the final target age, so an age-40 build is not an age-41 checkpoint.

Plan (fn-256 tasks 8-11): (8) reproduce and fix the two suspected races (`fault_out` non-atomic OR; `node_laterals` writes in the counted pass); (9) lattice dormant at zero, words and refusals unchanged; (10) instrument real input changes, then cached retirement and upward sums with dirty bits, each stopped if invalidation eats its saving; (11) remaining dominant kernels, then every preset re-measured native and in Chrome, the remainder taken to the owner.

## Round 6: lightning fast, most of the fidelity (2026-10-12)

The owner: "I really don't think we should settle for 1sec generation. I want to figure out how to make it lighning fast while keeping most of the fidelity." Both members (`council/astra-6.md`, Fable's round 6) define fidelity as a contract on the delivered tree, not a process: whole-view silhouette, limbs, shed interior and light gradient are history and need the coupled years; mid-view architecture is the last few years under local light; close-view twig and leaf geometry needs no history.

Agreed:
- **The light kernels first.** The device light code emulates f64 in software and gives each direction one workgroup looping layers; with every species lit (fn-264) Telperion's light is 755 ms of 1,402 stamped, final warm 2.09 s. Native f32, a parallel schedule of the same recurrence, the lattice over the crown's occupied box, the combine fused into the read. Engineering; gate lit Telperion's light under 100 ms.
- **Scale split is the breakthrough candidate:** the scaffold (axes whose expected life passes N years, N a request resolution, the cycle unchanged at N = 0) grows coupled through the cycle; shorter-lived laterals stand in the cycle for light and sums and are then grown alone, from lineage keys and the species tables, against the per-cycle field recorded at their tips (FIELD.md: within a year a subtree grows alone within +1.9% kept, 7.5 cm median tip). F4's deleted `chain.wgsl` is the measured decoupled grower (4.5M phytomers in about 0.3 s).
- Where they differ: Fable stands the short-lived buds in the cycle as their closed-form expectation; Astra requires a compact state carrying a vigour and survival spread and internal shedding, since a mean branch repeated fn-257's failure. The oracle decides.
- Rejected as whole-tree routes: direct synthesis from expectations (no distributions or correlations), runtime twig motifs (visible repetition, bucket pops; refused by STRATEGY's continuous space). Partial evaluation is the split's second half for forests and the site.
- Estimates (unmeasured): Fable about 0.4-0.5 s warm for Telperion with an 80 ms dispatch floor; Astra 30-150 ms if the light is replaced, dispatches cut and the full settle avoided. Under 100 ms for a whole hero tree may need fewer coupled years or axes, which is silhouette.

Experiments, in order: E3 the light kernels (device, fn-256 branch); E1 the scale-split oracle on the CPU (oak 80 seed 1, N = 3/5/8, both stand-in forms, kept within 5% and stills not told apart at whole or mid); E2 the scaffold's measured floor (sub-N laterals skipped, Telperion and the lit oak); then E4 the twig layer on the device.

## The scale split, measured (2026-10-13; fieldspike SPLIT.md)

- Exact stand-in (subtrees regrown alone against the exact run's field): oak within +1.2-1.5%, stills not told apart. The split holds when the field is right.
- Form (a), the closed-form expectation: oak +14-17%, half the sheds, a low limb kept; fails, and slower on the CPU.
- Form (c), grown at birth against the field held at birth: lit Telperion passes (+0.27%), the oak fails (+2.5% to +4.2%, stills told apart); the field goes stale over a twig's life; a grower per tiny subtree costs twice the coupled cost per phytomer.
- **Form (d), yearly against the current field: exact.** Sub-N subtrees stay out of the tree-wide sweeps and advance one year each cycle in one flat pass, reading the current lattice and one vigour at their root; the scaffold's sweeps cover scaffold axes only; the normaliser's sub-N partials are summed in their own pass and added to the scaffold's (host ruling). Totals, splits and shedding equal the one cycle's to the bit; oak N = 3 and 8 the same tree, Telperion N = 5 within 5 of 2.6M phytomers. It is a rescheduling, not an approximation.
- Where the time goes on the CPU: twig growth units and layout (380-507 ns per sub-N phytomer) are the same work either way; what (d) removes is the tree-wide sweep cost of twig axes (a scaffold axis costs 900-2,270 ns a cycle in passes, a sub-N axis 115-347 ns flat). Expected on the device: the level-by-level hierarchy dispatches (the latency-bound part) shrink to the scaffold; twigs become flat parallel passes.
- Host reading: full fidelity means growing every twig, coupled yearly through the field. The lightning floor is then per-phytomer growth work on the device (Telperion about 36 ns a grown phytomer today, 18M grown), so the levers are (1) the light kernels, (2) form (d) on the device, (3) the growth kernels toward their roofline.

## Form (d) on the device, and the order rule lifted (2026-10-13)

fn-256 task 14 (branch fn-256-form-d, unmerged, kept as evidence): the split is exact on the CPU and within tolerance on the device, but slower there (Telperion 655 -> 803 ms GPU, spruce 222 -> 268, oak 135 -> 147): the device's sweep cost is the serial chain of dependent steps down the tree's depth, which the scaffold keeps (levels 29 -> 23, serial depth steps 252 -> 371), and each twig's flat pass is serial within the twig. Not shipped; the production build stays N = 0.

The owner (2026-10-13) lifts the shared summing-order rule: the device may sum floats in any fixed order of its own; repeatability within an executor stays exact (no order-free float atomics); agreement with the CPU stays by lineage within measured tolerances. Next: parallel tree contraction (rake and compress) for the hierarchy sums, about log2(axes) dependent steps instead of the depth path, no subtraction of large partials; to the council first.
