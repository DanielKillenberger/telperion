# Council round 8: continuable growth and a smaller peak

Read-only; ground claims in code or measurement; mark the unmeasured. Read F4's COUNCIL.md (all rounds) and AGENTS.md's "Botanical growth is the product". F4 (`/home/daniel/Projects/telperion/.worktrees/f4`, branch fn-198) now holds the merged engine (fn-255/256/264/266); its evidence for the device cycle is under `.flow/evidence/fn-256-the-whole-grower-runs-on-the-gpu/` (GUARD.md, PROOF.md, DESIGN.md, RETIRE.md, ROOFLINE.md); fn-255's CPU cycle evidence under `.flow/evidence/fn-255-one-growth-law-reads-light-for-every/` (CYCLE-COST.md, R3B.md).

## The owner, 2026-10-13

"this seems like way to much peak memory" (Telperion's build peaks at 4,209 MB of device memory, 85 MB under the 4 GiB native default; its warm build is 1.24 s). "If we could just ask for a tree at an age and pass that tree back into the generator as input to grow from => the consumer can render the previous one while the next one is generated, would that add lots of overhead? or how would you do this?" Then: "alright add this to release, but make sure to test/fail early if something doesn't hold up. Maybe council first?" Growth is locked in as the product.

## The host's sketch (critique it)

- API: `begin(params, seed) -> GrowthState` (year 0, resident), `grow(state, years) -> GrowthState`, `settle(state) -> Tree` (the drawable tree at the state's age). The consumer draws the last settled tree while the next stretch grows. Exactness test: grow 0->82 equals 0->40->82 byte for byte on the device, and on the CPU.
- Retirement today frees subtrees the settle would drop at the final requested age; open-ended growth needs a rule that does not read the final age (free what is already dead now, keep the rest), and the settle must not depend on knowing the final age beyond the state's own age.
- Memory: Telperion's census: axis-indexed tables 854 B a row for every axis ever grown (about 1.5M), compacted only for species without records; phytomer rows 76 B for the live set; records 412 MB (every species now keeps records because leaf girth is shared); units' job counts about 0.4 GB; light scratch 179 MB; tables sized once from a hint at the worst ratio over all presets x 1.5. Levers: slimmer rows and scratch shared across phases, compaction of axes for every species, per-species hints that can grow on continuation, growth scratch freed before the settle. Target 1.5-2 GB for Telperion (unmeasured).

## Answer together, under 1,000 words

1. Is the API shape right for the harness, the site (fn-257: a field query per tree, WebGPU), and a game growing trees over years? What does `GrowthState` hold, and what can be dropped between calls?
2. Retirement and the settle without a final age: the exact rule, what it costs in memory, and whether 0->82 == 0->40->82 can hold byte for byte (keyed draws, reductions, compaction order, arena moves on continuation).
3. The memory plan: which levers, in what order, with what target per preset; whether continuable growth raises or lowers the peak.
4. The fail-early plan: the cheapest tests, in order, that would stop the work if a premise fails (for example the split-equality test on the CPU cycle first, before any device work; a census after each lever), each with a stop condition.
5. The spec contents (requirements in order) you would sign.
