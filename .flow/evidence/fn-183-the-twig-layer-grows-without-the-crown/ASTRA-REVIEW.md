# Astra review of fn-183, 2026-10-02

`gpt-6-astra` at high effort, read-only, on master a192a407, asked whether the spec is the most efficient and elegant solution under AGENTS.md and docs/principles.md, including "Before work is made faster".

**Change the framing to “twig extent follows branch architecture; outline queries must earn their place.”** Removing repeated admission checks is well supported, but the evidence does not yet establish a replacement falloff as the most efficient or elegant solution. R1 should first compare the existing length rules with the wall removed, then an inherited limb-system allowance, and only then outline-derived room. Keep the scaffold unchanged for that experiment. Reviewed at `a192a407`; the relevant instrumented code is unchanged from `e12a9a28`. Counts were checked against the instrumentation, not rerun; candidate appearance and performance remain **unchecked**.

1. **P1 — The spec chooses another outline constraint before testing whether twigs need one.**  
   [Spec:13](/home/daniel/Projects/telperion/.flow/specs/fn-183-the-twig-layer-grows-without-the-crown.md:13) also gives the wrong general botanical justification. Peripheral shoots do not universally weaken because light runs out there. Research finds greater branch extension with greater light, and substantial effects of branching order and age on peripheral oak shoots. [Sterck and Bongers](https://besjournals.onlinelibrary.wiley.com/doi/10.1046/j.1365-2745.2001.00525.x), [Buck-Sorlin and Bell](https://academic.oup.com/forestry/article-abstract/73/4/331/629092).

   Existing code already supplies radius-based initial length, inherited lateral length ratios, architectural directions, and an inner scaffold envelope reserving space for twigs ([seed.rs:176](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/seed.rs:176), [advance.rs:147](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/advance.rs:147), [branching.rs:240](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching.rs:240)). Attractor consumption supplies a scaffold competition mechanism; it does not currently steer local twigs. Growth-path “exposure” is another outline-depth approximation, while occupancy fields are downstream outputs ([crown.rs:108](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/specimen/crown.rs:108), [stage.rs:64](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/stage.rs:64)).

   **Change:** Add the zero-query baseline to R1. Test inherited system allowance next; remaining distance to the parent’s tip alone would wrongly starve terminal shoots. Describe outline falloff as an authored shape approximation unless botanical evidence supports more.

2. **P1 — Removing `Planner::run` checks does not replace the whole wall.**  
   Terminal and leaf-bearing lateral twigs bypass `run` and receive a fixed `twig.length`, followed by another admission check at [advance.rs:193](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/advance.rs:193). That remains in the direct build. The separate check at line 262 serves the expanding growth envelope.

   **Change:** R2 must specify terminal twig extent as well as planned axes. R3 should explicitly remove mature-build outline rejection from both paths, with the same extent rule serving them. Otherwise the outermost wood still meets a wall. Count terminal admission separately in R1; it currently sits outside the reported planner savings.

3. **P1 — R3 contradicts the curtain’s existing geometry.**  
   “Every axis ends inside the outline” excludes deliberately dropped curtains. Their permitted region includes a band below the crown ([pendant.rs:226](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/pendant.rs:226)). Moreover, the existing curtain already has a descendant-tip-derived floor, pendulous length, sag and clearance. It does **not** obtain that floor from the lower-surface search; the search validates candidate columns ([seed.rs:39](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/seed.rs:39), [pendant.rs:88](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/pendant.rs:88)).

   **Change:** Extend R1’s removal experiment to curtain admission using those existing controls. Plan any required vertical allowance once against the sagged trajectory. If the column-dependent band remains a product requirement, explicitly measure how an axis-level approximation preserves it; caching the starting column is not equivalent when the shoot moves sideways. R3 must distinguish ordinary extent, curtain clearance/band, and shortened limb systems. fn-174 should own only search work demonstrably remaining afterward. Also correct “each check runs the search”: `admits` short-circuits for points already inside.

4. **P1 — Endpoint tolerance cannot validate a bent axis, and the candidate measures answer different questions.**  
   [Planner:120](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/planner.rs:120) changes direction through crookedness, position-dependent bias and sag. An axis can leave and re-enter the envelope; one tip trim cannot repair that.

   Radial depth is the cheapest scalar, but ignores heading and surface slope. A bounded directional probe is the simplest geometric candidate for a nearly straight shoot. True nearest-surface distance would conservatively bound any path’s arc length from an interior start, but would shorten tangential shoots unnecessarily. The existing profile distance is smooth and axisymmetric, not distance to the lobed surface ([envelope.rs:242](/home/daniel/Projects/telperion/crates/telperion-core/src/envelope.rs:242)).

   **Change:** Define room semantics before sharing anything with fn-182. Birth-zone depth and directional travel allowance can share geometry without sharing one scalar. If outline planning survives R1, cap probe distance at useful authored length and assess curved paths with bounded axis-level work. Validate excursion along the whole run, including terminal wood, lobes and mapped limb bounds. Do not promise endpoint trimming as a general solution.

5. **P2 — One falloff row is plausible, but neither necessary nor proven sufficient.**  
   [Spec:40](/home/daniel/Projects/telperion/.flow/specs/fn-183-the-twig-layer-grows-without-the-crown.md:40) commits to a row before establishing its independent purpose. Authored length already supplies a scale; a normalized room/length law could need only one shape parameter. Additional rows should require evidence.

   **Change:** Make the row conditional on R1. If retained, define one finite continuous law across its full range, including its neutral value. No value should restore the old clipping algorithm or select a fallback. Unsupported inputs should produce named errors. R3 must check the resulting geometry and station/terminal transitions, not merely continuity of the scalar formula; shortening currently interacts with internode counts and twig classification ([advance.rs:157](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching/local/advance.rs:157)).

6. **P2 — The query targets are plausible, but tightly constrain replacement cost.**  
   From [QUERY-COUNTS.md:7](/home/daniel/Projects/telperion/.flow/evidence/fn-183-the-twig-layer-grows-without-the-crown/QUERY-COUNTS.md:7), assuming unchanged topology and other work:

   | Preset | Planner queries removable | Additional queries allowed per planned axis to meet R4 |
   |---|---:|---:|
   | Oak | 55.7% | 2.46 |
   | Beech | 59.9% | 3.64 |
   | Birch | 74.0% | 14.02 |

   A scaffold-style multi-sample probe could consume the oak saving. These are conditional budgets, not predictions for changed trees or seed 7.

   The evidence’s residual attribution also needs correction. Terminal admission is uncounted by purpose; direct-build scheduling is inactive; one `profile()` call does not exclude shedding’s per-node radius queries ([branching.rs:299](/home/daniel/Projects/telperion/crates/telperion-core/src/pipeline/branching.rs:299)).

   **Change:** Keep 40%/60% as provisional targets until R1. Report exclusive caller counts, node/axis counts and total work. Measure timing with counters disabled; atomic instrumentation changes the cost being measured. Replace literal “no preset slower” with a stated paired-run noise policy.

7. **P2 — fn-173 remains a candidate, not an established dependency or worthwhile optimization.**  
   Removing only the measured planner work leaves approximately 81,298 oak, 117,646 beech and 329,660 birch radius queries before replacement costs. Consumers include scaffold containment and sampling, terminal admission, shedding, and any retained curtain planning. Scheduling contributes no direct-build benefit and fn-173 explicitly keeps its tangent calculation exact.

   The GPU cull also needs accurate characterization. [place.wgsl:42](/home/daniel/Projects/telperion/crates/telperion-render/src/generation/place.wgsl:42) rejects deep interior foliage; exterior points pass its first test. It will not conceal oversized twigs. Its workload is absent from `growth_profile`.

   **Change:** Re-scope fn-173 from residual **time**, including preparation/upload and GPU measurements, rather than remaining counts alone. Remove the claim that it speeds scheduling. Decouple fn-182’s depth semantics from fn-173’s table mechanism ([fn-182:24](/home/daniel/Projects/telperion/.flow/specs/fn-182-branching-hierarchy-few-strong-limbs.md:24)). Whether the table still earns its complexity is **unchecked**.

8. **P2 — Tighten acceptance around visible outcomes and repeatability.**  
   [R1–R6](/home/daniel/Projects/telperion/.flow/specs/fn-183-the-twig-layer-grows-without-the-crown.md:54) otherwise provide a useful structure.

   **Change:** Let R1 use exploratory code; make R2 precede production implementation. Add independent curved-path/curtain checks, same-revision repeatability, and node/leaf-density measurements so query savings cannot come from accidentally emptying crowns. Include beech in exploratory comparisons and a crisp conifer plus strong-bias specimen in bounded visual validation, using bare and leafy views. Define the crisp-outline distance numerically. Keep R6, naming `cargo test --profile ci --workspace --no-fail-fast` once and `npm test`; retain artifact-size checks and end-to-end build/memory reporting. Do not add historical byte equality or growth-path tuning gates.