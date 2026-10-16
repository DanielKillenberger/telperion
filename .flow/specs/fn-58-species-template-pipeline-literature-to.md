# Species template pipeline: literature to preset

## Goal

The owner wants a template-building pipeline that takes a taxon from literature to a species packet, growth curves and a preset, with TypeSafe judging the evidence, delivered as a runbook and toolset an agent of any model can drive to the same template. The design leaves the driver nothing to judge: every reading step is a Jev question through fn-57's caller, every calculation is code, and every human choice sits in an admitted manifest or a decision a person resolves. A data-quality gate before any fit answers fn30's costliest lesson (a Garry oak curve with no open-grown age-indexed height under 30 years, found out at the end).

## Decisions

- user (turn 3): "so we could use this as part of a tree literature to preset pipeline?"
- user (turn 4): "so we could also use it to determine growth curves from literature and stuff like that?"
- user (turn 5): "we have firecrawl installed for fetching stuff"
- user (turn 7): "ok so in summary how do we use typesafe generally and is there a place for a spec to define a tree template building pipeline with typesafe to judge parameters, growth curves etc."
- user (turn 8): "ideally so that even cheap models could run this pipeline with typesafe with great success"
- user (turn 9): "alright"
- user (turn 11): "why should jev not choose any param value?"
- user (turn 12): "ok"
- user (turn 13): "can you review this with sol? or can jev judge this as well?"
- user (turn 14): "only fold it in if you actually agree"
- user (turn 15): "i think important for the pipeline is to judge the data quality. We lost a lot of time on the garry oak not having enough data about its growth curve. Jev should be able to judge this"
- user (turn 17): "ok i think given spec 34 we should also have a standing rule that each new species gets one spec. Not multiple species per spec. Or is the pipeline structurally different from the spec actually?"
- user (2026-09-17): "jev could probably figure out a new preset's values much more quickly than an llm could. Take the research data and previous templates research data + template values and figure out the new template values."
- user (turn 19): "i wouldnt have choose the cheapest model..? i'd like to point an agent at a runbook and toolset and then it uses that to produce the new tree species template based on the fn-58 work"
- Jev never writes a number. Evidence fields arrive by four routes only: copied span, fit over spans, described level (code proposes candidates from a manifest value table, renders and measures each, ships the in-range nearest-midpoint one), or reference transfer from tuned templates (added 2026-09-17, same render-and-measure admission). Engineering parameters come only from the manifest with a rationale.
- Plan review by GPT-5.6 Sol (2026-09-16, six rounds): agreed findings folded (isolated pinned swap test, manifest as human boundary, typed decisions bound to input checksums, schemas and provenance sidecar, fixed fit objective, R7 bounds). Declined: command names and paths in the spec. The host withdrew its objection to render-measured described levels once the owner's TypeSafe rule stated it.
- Determinism probe: identical calls give identical options with drifting probabilities, so comparisons use options, levels and copied values, never probabilities.
- One species per spec; this spec ships no species. Gap handling is fn-63's; the tuning loop after first render is fn-68's (owner chose a parallel spec after a 2026-09-18 probe showed code sweeps plus Jev per-dial direction beat yes/no and comparison questions).
- Owner, 2026-09-18, R7 dispositions: the nine rounding rows are accepted alternatives (fn30 printed rounded values); the misses row follows the rule that the direct build is the product; the source checksum fills on the first live run over the real PDF (c6c7d655).

## Acceptance Criteria

- **R1:** fixed command sequence, atomic canonical artifacts under versioned schemas, idempotence keys, driver runbook with no reading or deciding, command log — met.
- **R2:** one adapter contract, Firecrawl first; final URL, content type, raw and markdown checksums; coverage-gap against stated row counts; parked checks recorded — met (`.flow/evidence/fn58/parked-checks.md`: Vospernik 2010 in the research index, Gould 2011 and the Ertragstafeln not; CLI raw HTML differs from response bytes, so the raw checksum comes from a plain second request).
- **R3:** evidence fields by the four routes only, provenance sidecar, engineering marked, closed records gain no keys — met.
- **R4:** packet verification: citation check, semantic obligation Nouls, structural checks in code; question sets versioned with labelled sets — met.
- **R5:** curve fit in code with fn30's objective and grid, no extrapolation, tolerance-miss and missing-curve — met (reproduces fn30: oak rate 0.032, shape 2.0, mature age 432; spruce 0.091, 3.4, 158).
- **R6:** model-swap test, two isolated drivers, three trials each, byte-identical canonical artifacts — partial: `species-pipeline swap` tooling and an offline two-run comparison in `tests/validation_fn30.rs`; the live two-driver run did not happen (codex quota).
- **R7:** oak and spruce validation against fn30 with dispositions; held-out accuracy bounds — met for fit and fetch (`.flow/evidence/fn58/validation/`, dispositions recorded by the owner); held-out accuracies are enforced live by `jev cases` but no live run was claimed.
- **R8:** data-quality gate before select, fit and generate, filing data-insufficient below the bar — met (Garry oak young-age height scores proxy only at 0.87, wrong condition at 0.98; an open-grown control scores sufficient at 1.00; both labelled cases).

## Result

Eleven `species-pipeline` commands run from `docs/species-pipeline.md` (`crates/telperion-jev/src/pipeline/`). Not run live in this task: screen, select, verify, gate and generate on the oak or spruce. Gates: core 332, render/wasm/jev 253, `npm test` 85, typecheck, clippy. Follow-ups named for the owner: `serde_json` `float_roundtrip` workspace-wide (canon.rs holds a local fix), ordered Choice criteria without `preserve_order`, `curve.rs` at 495 lines, the `transfer:<dial>` engineering convention.

## Open

- Whether TypeSafe exposes a pinned model version (shared with fn-57).
- Whether two anchors (oak, spruce) carry a first reference transfer for a broadleaf.
- Which source carries an age-indexed table for the oak fixture (the Ertragstafeln oak is Quercus rubra).

## Friction

- Full workspace test run ~10 min for a change the species suite does not exercise -> a quick test command or crate-scoped pre-commit run (later fn-185, fn-271).
- Codex implementer quota exhausted -> reach probe before dispatch, named fallback tier.
- A Choice's criteria cannot carry an order without `preserve_order` (~20 min) -> owner line on ordered-array criteria; object kept meanwhile.
- Parallel module owners broke the crate build for each other (~15 min each, four agents) -> one worktree per owner, or declarations land with their files.
- Float round trips not byte-stable (~20 min) -> fixed locally in `canon.rs`; workspace feature is the owner's call.
- One-line test edit cost a ~25 min release rebuild of twenty jev test targets -> dev-profile tests for jev, fewer integration binaries.

## Evidence

- `.flow/evidence/fn58/parked-checks.md`, `parse-ertragstafeln.json`, `validation/` (fixtures, expected fits, `DISPOSITIONS.md`)
