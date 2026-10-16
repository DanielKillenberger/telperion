---
satisfies: [R3]
---
# fn-257-the-site-grows-its-trees-on-the-device.11 The site field asks only for what it reads, and its query's time is accounted

## Description
The owner (2026-10-16, spec section "The owner's decisions on reuse and the site field's time"): the spruce's site field (`growField` + a 36³ query, about 0.50 s) against its build (about 381 ms). Measure, then take each cost with a general remedy.

(a) Whether `growField` (`generation/resident/field/make.rs` `grow`: `tree::drawn` → `build::readied` → stations → primitives → index) runs drawing stages whose outputs the field never reads (dress, drawn records, sweep products, bark/normal/colour words, foliage generation...). If so, the field path asks the one pipeline for only what it reads: no second chain, no copied stage; a stage's unread outputs are not requested (a flag or request set the stages already honour, or a narrower entry the build's own path also passes through).

(b) Where the query's ~65 ms goes (`field/ask.rs`, `field_query.wgsl`, `web/field.rs` `query`): dispatch count and launches, readbacks and maps, buffer creation, index traversal against arithmetic. Take each non-arithmetic cost with a general remedy (e.g. one submission, one readback, buffers sized once).

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/field/*.rs`, `crates/telperion-render/src/generation/resident/field.rs`, `crates/telperion-render/src/generation/resident/build.rs`, `crates/telperion-render/src/generation/cycle/tree.rs`, `crates/telperion-render/src/generation/resident/field_query.wgsl`, tests, `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/FIELD-TIME.md`
**Touches:** [crates/telperion-render/src/generation/**, crates/telperion-render/src/web/field.rs, .flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/**]

### Approach
- Profile first with `CYCLE_TRACE` (make.rs already prints drawn / readied / stationed / made / indexed) and the query's own timestamps; list every stage the field path runs and which of its outputs the field reads.
- Measurement: paired warm A/B, 16 runs each interleaved, spruce (and oak, birch) at seed 1, through `Generator::grow_field` + `query_field` (the calls the package's `growField`/`query` make); `the_device_field` style test in `generation/resident/field_tests.rs`; base vs change in one binary via an env toggle only for the measurement, or two binaries run alternately.
- Field answers identical: every preset's answers at 24³, 36³, 48³ byte-equal before and after (the existing fidelity test plus a byte comparison of the answer words).
- The harness's build (`build_time`) not slower and its trees byte-identical where its path changed.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/resident/field/make.rs` — the field's stages
- `crates/telperion-render/src/generation/resident/build.rs` — `readied`
- `crates/telperion-render/src/generation/resident/field/ask.rs`, `field_query.wgsl` — the query
- `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/RESULTS.md`, `SPRUCE.md`, `measure.sh`

## Acceptance
- [ ] FIELD-TIME.md: the field path's stages with their time and which outputs the field reads; the query's 65 ms split into dispatches, readbacks, buffer creation, traversal and arithmetic.
- [ ] Each non-arithmetic cost with a general remedy taken (or named with why none applies); one pipeline, no second chain.
- [ ] Paired warm 16-run medians before and after: the spruce's site field (grow + 36³ query), and the oak's and birch's; field answers byte-identical at 24³, 36³ and 48³ for every preset.
- [ ] Harness build times not worse, its trees byte-identical; focused tests green; codex impl-review SHIP.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
