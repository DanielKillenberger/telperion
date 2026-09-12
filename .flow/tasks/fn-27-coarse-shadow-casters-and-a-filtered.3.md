---
satisfies: [R2, R3, R4, R5, R7]
---
# fn-27-coarse-shadow-casters-and-a-filtered.3 The clock on both trees, the caster counts in the record, the report, the docs and the owner's verdicts

## Description
Record the caster counts in the timing record and the browser type, time the oak and the spruce natively and in the browser at the default row beside fn-14's numbers, write the report in fn-14's shape, sweep the docs the casters and the kernel made stale, close fn-14's three stale strings, and write the two owner verdict slots. Ends NEEDS_HUMAN by design until the owner writes the verdicts; R3's number is the stop if it is over.

**Size:** M
**Files:** `crates/telperion-render/src/timing/report.rs` (`with_casters`, JSON fields), `crates/telperion-render/src/timing.rs` (split under 400 if the counts push it), `crates/telperion-render/src/lib.rs` (counts into the report), `crates/telperion-render/tests/timing.rs`, `src/browser/render.ts` (record type with the undeclared fields; the view type gains clay), `crates/telperion-render/examples/headless/walk.rs` (usage line), `crates/telperion-render/src/view.rs` (header), module and shader headers in `shadow.rs`, `shadow/fit.rs`, `scene.rs`, `scene/row.rs`, `shaders/shadow.wgsl`, `common.wgsl`, `foliage.wgsl`, `README.md`, `.flow/evidence/fn27/` (stills, timing and orbit records, REPORT.md), `.flow/specs/fn-27-coarse-shadow-casters-and-a-filtered.md` (verdict slots)
**Touches:** [crates/telperion-render/src/timing.rs, crates/telperion-render/src/timing/**, crates/telperion-render/src/lib.rs, crates/telperion-render/src/view.rs, crates/telperion-render/src/shadow.rs, crates/telperion-render/src/shadow/**, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/**, crates/telperion-render/src/shaders/**, crates/telperion-render/examples/headless/**, crates/telperion-render/tests/timing.rs, src/browser/render.ts, README.md, .flow/evidence/fn27/**, .flow/specs/fn-27-coarse-shadow-casters-and-a-filtered.md]

### Approach
- Record: follow `with_levels` at `timing/report.rs:132-150` for `with_casters(triangles, instances)`; the counts are present on an invalid record like `multisample` (`:95-102`); `to_json` at `:254-297` adds `caster_triangles` and `caster_instances` after the counts it already writes. `timing.rs` is at 397 lines; if the counts push it over, move the report assembly out of `collect` (`:340-397`) into `timing/`. `tests/timing.rs:138-179` lists the full record's fields and the total-equals-sum identity; add the two counts, and the identity is untouched because counts are not times.
- Browser type: `src/browser/render.ts:49-76` declares the record; add the two counts with `shadow_p50_ms`, `shadow_p95_ms` and `multisample`, and add `clay` to the view type at `:78`.
- Clock: oak native at the default row (R3) with the per-pass split; spruce native at the same protocol, the first native spruce clock; oak and spruce browser orbits through the rig with `RENDER_EVIDENCE=.flow/evidence/fn27`, on a port other than 5173. One run each; an invalid session is re-run once and recorded as such. Record every number beside fn-14's: 4.949 total, 1.813 shadow, 10.00/10.10/10.20 oak orbit, 30.00/30.20/30.40 spruce orbit, 13.51 spruce shadow.
- Stills: the lit oak hero and the lit spruce hero at seed 7, the default row; the clay oak still for the pin. Four images viewed at most across the task.
- Report: `.flow/evidence/fn27/REPORT.md` in fn-14's shape, with a section that states what each lever bought against fn-14's numbers, the caster counts beside the full counts, and a follow-ups section that names needle aggregation for the spruce if it is still over 60 fps.
- Docs: rewrite the headers the casters made stale (`shadow.rs:6-9,18-27`, `shadow.wgsl:1-3,12-14`, `common.wgsl:57-60`, `foliage.wgsl:73-75`, `scene.rs:1-5`, `scene/row.rs:1-11`, `timing/report.rs:37-71`, `fit.rs:9-11`), README lines 59, 98 and 130, and fn-14's three stale strings (`examples/headless/walk.rs:14`, `view.rs:1-2`, the browser record type above).
- Verdicts: write the R2 and R5 slots in the spec's Owner verdict section, leave them for the owner, and end with the aids named.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/timing/report.rs:37-150,254-297` — the record and its JSON
- `crates/telperion-render/tests/timing.rs:138-218` — the field list and the identity
- `.flow/evidence/fn14/REPORT.md` — the shape and the numbers to sit beside
- `src/browser/render.ts:49-84` — the record and view types

**Optional** (reference as needed):
- `crates/telperion-render/src/lib.rs:286-289` — why the sun's pass is not in the frame stats
- `.flow/evidence/fn14/spruce-browser-timing.json` — the browser record shape

### Key context
- The spruce's numbers stop nothing; the oak's R3 number stops the spec if it is over the bound, with the number, not another attempt.
- Evidence reports are dated and not rewritten; fn-14's follow-ups close here and in the README link list.
- Budget rules from CLAUDE.md bind: one timing run per capture, at most four images viewed, no forest captures, never open receipts or frames.

## Acceptance
- [ ] The timing record carries caster_triangles and caster_instances on valid and invalid records; every existing field unchanged; the browser record type declares them with shadow_p50_ms, shadow_p95_ms and multisample, and the view type includes clay
- [ ] Oak native total p50 at the default row recorded with the per-pass split beside fn-14's 4.949 ms; at or under 3.8 ms, or the spec stops with the number
- [ ] Spruce native clock and both browser orbits recorded beside fn-14's; the oak orbit holds wall p95 under 16.7 ms with no frame over 33 ms; a spruce still over 60 fps names needle aggregation in the report
- [ ] Lit oak and spruce hero stills and the clay oak still under `.flow/evidence/fn27/` with REPORT.md in fn-14's shape
- [ ] The stale module, shader and README lines rewritten; fn-14's three stale strings closed
- [ ] The R2 and R5 verdict slots written in the spec and left for the owner
- [ ] `cargo test --release --workspace`, fmt, clippy and `npm run typecheck` pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
