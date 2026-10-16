# The test suite under three minutes: one generation per specimen, nextest, shards

## Goal
After fn-74 a run where every suite executes still took about ten minutes, eight of them serial test execution (`species.rs` 214 s on the four-core runner). The owner wanted it brought down by software, not a stronger runner: generate each fixed specimen once, run binaries in one parallel pool, shard across standard runners, with no assertion, seed or test lost.

## Decisions
- > user (2026-09-18): "aren't there tools to increase rust test suites that we can use? surely there must be some software improvement that can be done not using stronger runners."
- > user (2026-09-18): "well we can /flow-next:capture this into one new spec"
- > user (2026-09-18, earlier): "i like ci being below 5min. Ideally much below."
- Owner, 2026-09-18: shards named per crate (`core 1/4` to `core 4/4`, `render`, `wasm and jev`), each crate's receipt saved when its own jobs are green.
- Measured, not inferred: the skeleton is a twentieth of a specimen (foliage placement and cull dominate, 2.8 to 4.2 s), so the cache holds skeletons, not meshes (a whole-mesh cache was 3.4 GB for twelve entries and slower on a miss).
- Rejected: ignoring the species suite on PRs, cross-run specimen caching, a larger runner.

## Acceptance Criteria
- **R1:** Species suite generates each fixed seed once and proves determinism against a committed digest per preset and seed — met (49 digests in `crates/telperion-core/tests/species/digests.json`, matched on the runner).
- **R2:** Fixed specimens come from a per-run cache keyed on wire, seed and generator sources — met for skeletons (`tests/specimens/mod.rs`, under `target/tmp/specimens`, fifteen files).
- **R3:** Rust suites run via nextest as a partitioned matrix; receipts only when every partition is green — met.
- **R4:** No assertion, seed or coverage lost; counts recorded — met (599 -> 604: five added, none lost).
- **R5:** Every-suite run under 3 min on standard runners; single-runner warm under 5 — met warm (2 min 55 s); master's cold first run 4 min 10 s.

## Result
Digest determinism, a skeleton cache, `fixed_species` running four seeds in flight on scoped threads (spruce 209 s -> 41 s on the desk), and six named nextest jobs. Runner (PR #37): first run 6 min 23 s cold (shards bound by serial seeds), after named shards and threaded seeds 2 min 55 s, master's cold merge run 4 min 10 s. A host-review bug (an empty `cache-hit` collapsing `read` fields so no Rust receipt was ever saved) was fixed by publishing literal `true`/`false`; run 35395043461 wrote all four receipts.

## Open
None.

## Friction
- The spec's inferred cost model (generation dominates) was wrong; 30 min of measurement before design -> lesson: a performance spec cites per-stage measurements, or goes to research refine first.
- Local command guard refused five ordinary commands -> local setup, not specced.
