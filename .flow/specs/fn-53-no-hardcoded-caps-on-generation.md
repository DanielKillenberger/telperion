# No hardcoded caps on generation

## Goal

While fn-31 was judged, the owner asked why the ten-year oak test carried a node cap; limits unrelated to a tree's form (test count gates, a build-time ceiling, generator constants) had been steering the growth rule and stalling form fixes. The owner set the rule on 2026-09-15: a count or limit that changes what the generator grows is a configurable parameter with a validated range, never a constant hardcoded apart from those ranges. This spec takes the whole project to that rule (fn-31 handled its own gates) and adds a guard that keeps it.

## Decisions

- user (2026-09-15): "why do we have a node cap?"
- user (2026-09-15): "shouldn't we avoid having arbitrary caps? but just make sure the generator produces expected results without assertions of any kind by keeping parameter ranges clean and accepting that some trees may need some more performance. We rather improve performance than arbitrarily cap things that have impact on generation."
- user (2026-09-15): "if the node count is a parameter that can be set i guess that's fine but not hardcoded separately from ranges for params that are configurable. Makes sense?"
- user (2026-09-15): "ok can you capture another spec that cleans up those caps/constants and makes sure the project follows this."
- Numeric tolerances (epsilons, convergence thresholds) are not caps and stay constants; history retention, age range and existing budget settings already comply. Renderer/GPU limits are out of scope. Tests check form, not counts.
- Owner requested direct merge without a PR.

## Acceptance Criteria

- **R1:** every constant and limit in the generation core and Wasm bindings inventoried and classified (range bound, tolerance, or cap); no cap remains — met (inventory committed under docs/).
- **R2:** node budget is one parameter whose range carries its upper bound; no separate ceiling; reaching a caller budget gives the capped diagnostic — met.
- **R3:** scaffold step limit, twig depth limit and bend-resolution floor become ranged parameters or are removed — met (eleven former limits now named validated parameters; snapshot schema 3).
- **R4:** every preset builds the same tree or a documented change; determinism and native/Wasm parity hold — met (all 21 matched sample pairs identical hashes and counts; no repins).
- **R5:** build cost per preset measured before and after with load recorded; regressions optimized, not capped — met (median shifts −3.2% to +4.3%, within load noise).
- **R6:** core tests assert form and parameter contracts, not counts — met.
- **R7:** a guard test fails on a new limit that is neither a range bound nor a declared tolerance — met.

## Result

Removed the hidden 250,000-node ceiling, the 4,096-step scaffold axis limit, the twelve-generation twig limit and the bend-resolution floor; the source guard also surfaced form limits in foliage station placement, packed placement keys, rejection sampling, reach probing and limb clumping, all carried as parameters through wire, blend and harness. Beech seed 1 with three laterals per station and a 1,000,000 budget grows 254,252 nodes uncapped. Mature direct builds (seed 42 etc., median ms before/after): oak 1,147/1,131, spruce 5,789/5,763, beech 4,071/3,942, birch 3,662/3,819, ordinary 145/147. Review (Fable medium): NEEDS_WORK on five findings (notably a sparse persistent foliage index), fixed in 08c5357d, then SHIP; nested birth/station maps cut index pages 25,105 to 4,229 on the bounded fixture. 569 Rust tests and 99 Vitest tests green.

## Open

None.

## Friction

- Safety hook refused a recoverable move of three duplicate untracked evidence copies; merge blocked until moved by hand -> local setup issue, reported, not specced.
- Review helper ran the whole Wasm lib in debug, a minute of inconclusive output -> use exact filters or the ci profile.
- Old-contract fixtures stopped successive full runs (~5 min) -> run `--no-fail-fast` for broad contract migrations.
- Cap inventory grew beyond the initial four (~15 min) -> a classified inventory at capture; the checked inventory now exists.
- Baseline timings ran beside the core suite (load 5 to 11) -> serialize performance samples with test suites.
- make-pr/land workflow instruction volume (~23k tokens to read) -> proposed shorter common path (later `docs/pr-format.md`).
