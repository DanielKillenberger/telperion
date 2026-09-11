---
satisfies: [R1, R2, R3, R11, R12]
---
# fn-14-bark-and-foliage-appearance.6 The evidence: references fetched, lit hero stills, the clock, the report, docs, and the owner's verdicts

## Description
Produce the evidence the spec closes on: fn-9's reference photographs re-fetched into the ignored references directory, the lit oak and spruce hero stills at seed 7, the oak's timing and browser orbit beside fn-24's, the clay view still for R3, the report, the docs that went stale, and the two owner verdict slots (R11, R12, the evidence halves of R1 to R3). Ends NEEDS_HUMAN by design until the owner writes the verdicts.

**Size:** M
**Files:** `.refs/fn9/` (ignored), `.flow/evidence/fn14/` (stills, timing JSON, REPORT.md), `.flow/specs/fn-14-bark-and-foliage-appearance.md` (owner verdict section), `README.md`, `STRATEGY.md` (one wording line), module headers in `crates/telperion-render/src/{lib,scene,wood,foliage,timing}.rs` and the three shader headers, `docs/species-onboarding.md`
**Touches:** [.refs/**, .flow/evidence/fn14/**, .flow/specs/fn-14-bark-and-foliage-appearance.md, README.md, STRATEGY.md, docs/species-onboarding.md, crates/telperion-render/src/lib.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/wood.rs, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/timing.rs, crates/telperion-render/src/shaders/**]

### Approach
- References: run fn-9's retrieval command from `.flow/evidence/fn9/REFERENCES.md` for O-WHOLE and S-WHOLE into `.refs/fn9/`; record source URL and SHA-256 in the report; never copy a pixel into the repo. Not a GPU capture.
- Stills: oak and spruce at seed 7, hero pose, 1600 by 1000, the default scene row, into `.flow/evidence/fn14/`; one clay-view oak still beside them for R3. View at most four images.
- Clock: the oak native timing and orbit, the browser orbit through `npm run test:render`, all with the shadow pass and 4x; the numbers beside fn-24's with the verdict vocabulary; R12's bound is 3.8 ms native p50 and the orbit's 16.7 and 33 ms.
- Report in fn-24's shape, plus a materials section: the shadow and resolve numbers, the sample count, and the memory the new buffers and the shadow map take.
- Docs: README lines that describe the clay room, the viewer, the headless flags and the timing record; the module and shader headers that say clay; the strategy's owner's-eye metric line drops "in clay"; species-onboarding gains one line on the appearance ranges the reference supplies.
- Verdicts: an `## Owner verdict` section in fn-23's shape with two slots, oak and spruce, each with the photograph's source and checksum beside it; leave the quoted lines empty for the conductor.

### Investigation targets
**Required** (read before coding):
- `.flow/evidence/fn9/REFERENCES.md` — the manifest and retrieval command
- `.flow/evidence/fn24/REPORT.md` — the report shape and verdict vocabulary
- `README.md:55-150` — the stale prose

**Optional** (reference as needed):
- `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md` — the owner verdict section shape

### Key context
- Budget rules bind: one timing re-run per target, no forest captures, four images viewed at most, never open receipts.
- If R12's number fails, write it into the report and return; no tuning.

## Acceptance
- [ ] O-WHOLE and S-WHOLE fetched under `.refs/fn9/` with source and checksum in the report; nothing from them committed
- [ ] Lit oak and spruce hero stills and one clay oak still under `.flow/evidence/fn14/`
- [ ] Oak native timing and browser orbit recorded beside fn-24's with verdicts; native p50 at or under 3.8 ms and the orbit inside its bounds, or the spec stops with the number
- [ ] `REPORT.md` in fn-24's shape with the materials section
- [ ] README, strategy wording, module and shader headers, and species-onboarding no longer describe the clay room as the only light
- [ ] Owner verdict slots for oak and spruce written into the spec, ready for the owner's words

## Done summary
fn-14's evidence is on disk and the spec now has the owner's two verdict slots.
fn-9's two whole-tree photographs were re-fetched into the ignored `.refs/fn9/`
with their sources and SHA-256s recorded, the lit oak and spruce hero stills and
one clay oak still were rendered at seed 7 and 1600x1000 into
`.flow/evidence/fn14/`, the oak was put on the clock natively and in the browser
at four samples with the shadow pass timed on its own, `REPORT.md` was written
in fn-24's shape with a materials section, and the README, STRATEGY.md, the
`lib.rs` and `scene.rs` headers and `docs/species-onboarding.md` stopped
describing the clay room as the only light the renderer has.

The task ends NEEDS_HUMAN by design: the two verdict slots and R12's stop are
the owner's.

**R12 is over its bound and the spec stops with the number.** Oak, seed 7,
1600x1000, RTX 3080, valid session, four samples a pixel: vegetation p50
3.050 ms, selection 0.087 ms, shadow 1.813 ms, **total p50 4.9487 ms** against
the 3.8 ms bound, 1.15 ms over. Nothing was tuned to chase it, per R12's error
clause and the owner's standing instruction. The browser orbit holds its half of
R12 exactly where fn-24 left it: wall p50 10.00 ms, p95 10.10 ms, worst 10.20 ms
over 999 frames. The lever named in the report is the sun's depth pass, which
draws the full-resolution wood for 1.813 ms because only the crown has a level
ladder; the same pass is 13.51 ms of the spruce's 28.49 ms browser frame.

**Two deliberate choices a reader may want to reverse.** The browser rig was
pointed at this spec's own directory with `RENDER_EVIDENCE=.flow/evidence/fn14`,
so it wrote its four records there instead of rewriting fn-23's closed numbers,
which is what it does by default and what fn-14.5 had to stash. And three stale
strings outside this task's declared Touches were left alone and recorded
instead: the headless usage line still offers `--view whole|bare|leaf` though
`clay` works, `src/view.rs`'s header still says "the same three the harness
offers" though `View::NAMES` has four, and `src/browser/render.ts` still does
not type `shadow_p50_ms`, `shadow_p95_ms` or `multisample`, which every record
in `.flow/evidence/fn14/` carries.

Baseline: green via handoff (verified at 4fc5027 by fn-14.5). Gates after the
commit: `cargo test --release --workspace` 169 passed, 0 failed, 7 ignored;
`cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets --
-D warnings` clean; `npm run wasm:build`, `npm test` (65) and `npm run
typecheck` clean; `npm run test:render` PASS. Green receipt
`.flow/tmp/green-receipts/8519ba29-unittest.json`. Budget: three images viewed
of four allowed, four native runs with at most one timing session each, one
browser run, no forest capture, no receipt or frame opened.

stage: impl-review - skipped(config: REVIEW_MODE=none)

Owner verdicts written 2026-09-11: oak accepted, spruce accepted with a note on branches showing through the needles, R12 number accepted with fn-27 as the fix.

stage: wave-join - ran (fast-forward 3ba6e68..8519ba2, no collision; report images embedded in c1a8caa)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8519ba29417ac0023d10359bde4c125bc7c59ea7
- Tests: cargo test --release --workspace (169 passed, 0 failed, 7 ignored), cargo fmt --all -- --check (clean), cargo clippy --workspace --all-targets -- -D warnings (clean), npm run wasm:build && npm test (65 passed) && npm run typecheck (clean), RENDER_EVIDENCE=.flow/evidence/fn14 npm run test:render (PASS: 5 presets, dial, views, 2 timing sessions, 2 orbit sessions), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --out .flow/evidence/fn14/oregon-white-oak-hero.png --timing .flow/evidence/fn14/oak-native-timing.json (valid, total p50 4.9487 ms), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --view clay --out .flow/evidence/fn14/oregon-white-oak-clay.png
- PRs: