---
satisfies: [R1]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.1 The fine-root record and its decision contract, on the CPU reference executor

## Description
Astra changes 1 and 2 (spec Design; ASTRA-REVIEW-FINE.md sections 2-4). (a) Audit every continuous-to-discrete decision a fine root's growth, shedding, lay and conversion take (lineage.rs draws and Poisson recurrence, grow/advance.rs survival and abortion thresholds, schedule.rs fractional cycle times and its 1e-12 boundary, P2's shedding in src/alone.rs, the converter's merge and cull tests), file:line, into FINE-CONTRACT.md. (b) Give each a specified deterministic representation both executors can evaluate identically (integer draws against canonical integer thresholds, integer cycle ticks, or values supplied in the record), and say for each whether it moves today's trees. (c) The canonical fine-root record: everything P2's Bud carries plus option 4's committed section, mass and first moment, attachment frame, ancestor survival and termination, dormant and relay history, retained-pipe transfer; light-reading species' aggregate state is named but its content is F2b and DESIGN-LIGHT.md's. (d) The CPU grows the structure above the cut, chosen before growth by the closed form's expected kept phytomers (N = 1,024), and the reference executor grows each fine root from its record alone through the pipeline's expansion (executor module, docs/pipeline.md updated). Stop and report any design question beyond the spec.

## Acceptance
- FINE-CONTRACT.md lists every decision with file:line, its representation and whether it moves trees.
- Fine roots expand from records alone on the CPU reference executor; a tree built through it equals itself across item order, chunking and N in {256, 1024, 4096} for the partition (botanical inputs fixed).
- Adversarial unit cases each have a test: fractional lifespans, threshold-adjacent draws, relay and dormant buds, ancestor shedding, retained pipes, a loaded junction.
- The predictor's expected against realized fine-root sizes, empty roots and item work (organs, chains, cycles) at ages 40 and 80 for beech, spruce, oak, recorded.
- Workspace gate green.

## Done summary
Fine roots grow from their records alone on the CPU reference executor (`crates/telperion-space/src/fine/`, `grow_fine`).

- **The partition.** The CPU grows the structure above a cut, chosen before growth by a light-neutral predictor: the closed form's expected kept phytomers, a new `count` column of the kept tables, held against grown trees in `kept/tests.rs`. Each bud at or below the cut becomes a `FineRoot` record. The executor grows, sheds, scales, girths and lays each root from its record alone, using P2's shared chain loop and option 4.
- **The partition is exact.** A tree is the same to the bit at N in {0, 256, 1,024, 4,096, ∞}, in every item order and chunking, and on 1 or 4 cores. It equals the tree P2 grows whole.
- **Adversarial tests**, each the same at every cut: fractional lifespans, threshold-adjacent draws (including a fine root's viability set exactly at its draw and one bit above), relay and dormant buds, ancestors' shedding, retained pipes, and a loaded junction where every root stands exactly on its bearer's laid node. A species that reads light is refused (F2b).
- **Integer draws.** Every draw decides as a 53-bit integer against `ceil(p · 2^53)`, which is exact. No tree moved: 28 trees dumped against the base matched.
- **FINE-CONTRACT.md** lists 24 decisions with file:line, representation and whether each moves trees. The record and the predictor tables are in it too.
- **Open for the host:** the arithmetic of per-bud thresholds.
  - A (pinned `libm`, IEEE f64): measured topology unchanged, tips within 1e-12 m. Recommended.
  - B (2^-20 ticks with fixed point): measured topology unchanged, tips within 2.5e-5 m.
  - The converter's merges (D22).
  - The ground refusal inside a root (D19).
  - The executor's home before F4.
- `docs/pipeline.md` gains a "Fine growth" section.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ffeaec59, 43f0dbce
- Tests: cargo test --profile ci --workspace --no-fail-fast (exit 0, 12m21s, under the GPU lock), cargo test --profile ci -p telperion-space --lib fine:: (12 passed)
- PRs: