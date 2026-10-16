# One growth law reads light for every species

## Goal

fn-245's second stage. The owner (2026-10-09): "i'm still not comfortable with only the oak growign with light... seems like it breaks our design principles". It did: `Species::reads_light()` (any of four light rows non-zero) selected a different grower (the stepped CPU executor, `fine/coarse.rs::lit`, `grow.rs::apart` keeping light-reading trees off the GPU's fine roots). Astra's architecture review (fn-245 `ASTRA-ARCH.md` §1-2) proposed one cycle for every species with light read from a frozen summary of the previous cycle's foliage, the light model the open research question. Release item for fn-237. Scope: the CPU executor and the light model; the GPU move is fn-256.

## Decisions

- Council plan (host, 2026-10-11): the owner asked for a council of Fable and Astra; both signed `.flow/evidence/fn-245-a-breakthrough-in-generation-time/COUNCIL.md`, which governs this spec where it differs from the requirements. R3's foliage-volume light candidate is refused on its measurement (R3.md) and the lattice is the light model for every species; R3a is dropped; R3b is the CPU executor as the one cycle with explicit foliage lifetime, phase order, canonical identity, persistent adjacency and reusable scratch; light rows sourced for every species; the council's early tests 1 and 2 first.
- Early tests: the cycle's extra cost is processing, not growth (F4's grower grows the same phytomers in a tenth of the time); a quantised integer deposit is within f32's order noise.
- Cost (CYCLE-COST.md): the whole-tree cycle has a floor of 4.4× F4's grower on Telperion and 2.3× on spruce with no light work at all; host decisions of 2026-10-11 built a working set with frozen upkeep, exact retirement of shed subtrees, and the spike's reduction order.
- R3b: routing production `grow` through the cycle failed the render gate's resident-against-CPU checks, so production routing, the `reads_light()` consumers and the old executors' deletion moved to fn-256 (fn-255 and fn-256 merge into F4 together).

## Acceptance Criteria

- **R1:** every shipped preset grows through one cycle graph; `reads_light()` and every branch it drives deleted; zero light coefficients give neutral terms, never another grower — met on the CPU (`grow_cycle`); the deletions moved with fn-256.
- **R2:** per-PA normalisation unchanged; light and allocation never tested together — met.
- **R3:** the light-volume candidate measured against the lattice as oracle; approval is the pass — measured and refused (council, 2026-10-11).
- **R3a:** degenerate-foliage handling — dropped by the council.
- **R3b:** every build runs through the one CPU cycle; the independent-chain GPU path retires — moved to fn-256 (the production switch fails the render gate without the device executor).
- **R4:** every preset's stills viewed, the oak's light response held, walks cross zero light without a jump, build time per preset, Astra's agreement, gate and npm test green, the owner's verdict — met on the CPU; combined acceptance with fn-256.

## Result

- `grow_cycle` runs every phase for every species with allocation unchanged; an unlit species grows today's tree; the oak through the cycle agrees with today's build within `agree::PATHS`; zero and 1e-30 grow the same tree. Retirement is exact against the every-axis-open reference (oak, spruce, beech, 4 seeds).
- Every preset's stills viewed; cost and peak memory in CYCLE-COST.md.
- Gates on the final commit: workspace without the renderer 1,070 passed; renderer 242 passed; npm test 155. Impl-review SHIP after two fixes.
- Finding (specced as fn-265): shed wood keeps charging its bearer's upkeep; the oak at 80 keeps 10.5 % more if it stops.

## Open

- fn-256: production routing through the one cycle on the device; deletion of `reads_light()`, the lattice consumers and the stepped executor.
- fn-265: shed wood stops costing its bearer.

## Friction

- `flowctl` not on PATH for dispatched workers -> local setup.
- The lattice-scale candidate ran slowly -> the slowness is the finding (R3.md).
- The amplification probe compared axes by lineage -> a documented unique per-axis key for tree diffs (proposed).
- Restoring a throwaway patch reverted the worker's own edits -> apply throwaway patches only on a committed tree, reverse with `git apply -R` (practice).
- No sampling profiler (`perf_event_paranoid`) -> local setup, reported.
- R3b's production switch was tried against the whole gate -> a named list of tests that hold `grow` to another executor (proposed).

## Evidence

- `.flow/evidence/fn-255-one-growth-law-reads-light-for-every/R3.md` (the refused light-volume candidate against the lattice)
- `.flow/evidence/fn-255-one-growth-law-reads-light-for-every/CYCLE-COST.md` (the cycle's cost floor and the levers built)
- `.flow/evidence/fn-255-one-growth-law-reads-light-for-every/DESIGN.md` (the cycle graph and the candidate's design; cited by `grow/cycle.rs`)
- `.flow/evidence/fn-255-one-growth-law-reads-light-for-every/R3B.md` (what moved to the one cycle and what went with fn-256; cited by `capture_grown.rs`)
