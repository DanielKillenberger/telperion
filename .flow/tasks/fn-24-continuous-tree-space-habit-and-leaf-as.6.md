---
satisfies: [R3, R4, R6]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.6 The evidence: stills, the transition, the oak's frame, and the owner's verdicts

## Description
Produce the evidence the spec closes on: one still per preset at the hero pose, the oak-to-spruce transition, fn-23's oak timing and orbit re-run, a report, and the owner's recorded verdicts on the oak still, the spruce still and the video (R3, R4, R6). Fold the doc updates the enums leave stale into the same task.

**Size:** M
**Files:** `.flow/evidence/fn24/` (stills, timing JSON, transition record, `REPORT.md`), `.flow/specs/fn-24-continuous-tree-space-habit-and-leaf-as.md` (owner verdict section), `docs/species-onboarding.md`, `tests/migration/README.md`
**Touches:** [.flow/evidence/fn24/**, .flow/specs/fn-24-continuous-tree-space-habit-and-leaf-as.md, docs/species-onboarding.md, tests/migration/README.md]

### Approach
- Stills: `npm run species:qa` through the headless target for all five presets; oak and spruce at seed 7, hero pose, 1600 by 1000, the same framing as `.flow/evidence/fn23/oak-hero.png` and `.flow/evidence/fn22/spruce-hero.png`. Ordinary, Telperion and Laurelin are recorded, not gated.
- Transition: one run of the headless command from oak to spruce at seed 7, 240 frames, into `.flow/evidence/fn24/transition/`; the record and the notice go in the report. Do not open the frames or the video.
- Timing: re-run fn-23's oak native timing and browser orbit exactly as fn-23's quick commands do, into `.flow/evidence/fn24/`; the report puts the numbers beside fn-23's with the same verdict vocabulary. An unavailable, disjoint or contended session is recorded and does not count.
- Report: follow `.flow/evidence/fn23/REPORT.md` in structure; protocol first, then per-preset results, then the transition record.
- Verdicts: add an `## Owner verdict` section to the spec in fn-23's shape with three slots, oak still, spruce still, transition video, each answered in the owner's words; the task ends NEEDS_HUMAN until the owner writes them, and a rejecting verdict stops the spec with a one-paragraph blocker.
- Docs: reword the capability column at `docs/species-onboarding.md:13` away from enum-gated anatomy, and re-check the walkthrough at `tests/migration/README.md:91` against the renamed tests.

### Investigation targets
**Required** (read before coding):
- `.flow/evidence/fn23/REPORT.md` — the report shape and verdict vocabulary
- `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md` — the Quick commands and the Owner verdict section shape
- `tests/species.mjs` — the QA still protocol

**Optional** (reference as needed):
- `.flow/evidence/fn23/oak-native-timing.json` — the numbers to sit beside

### Key context
- Budget: at most four images viewed per capture; one transition render; one timing re-run. No forest captures.

## Acceptance
- [ ] Five stills at the hero pose under `.flow/evidence/fn24/`; oak and spruce match fn-23's and fn-22's framing
- [ ] Transition frame sequence and record under `.flow/evidence/fn24/transition/`, video present or a recorded notice
- [ ] Oak native timing and browser orbit recorded beside fn-23's numbers with verdicts; oak p50 at or under 2 ms native and the orbit inside fn-23's R2 bounds, or the spec stops with the number
- [ ] `REPORT.md` in fn-23's shape
- [ ] Owner verdicts on the oak still, the spruce still and the transition video recorded in the spec in the owner's words; the spec closes only on three accepting verdicts
- [ ] Species-onboarding and migration docs no longer describe enum-gated anatomy or stale test names

## Done summary
The evidence fn-24 closes on is on disk under `.flow/evidence/fn24/`: five hero
stills at seed 7 and 1600 by 1000 in fn-23's and fn-22's framing, one
oak-to-spruce transition of 240 frames with its record and its video, the oak's
native and browser clocks re-run beside fn-23's, the frozen species protocol
re-run numerically, and `REPORT.md` in fn-23's shape. The spec carries an
`## Owner verdict` section with three empty slots, and the capability handoff in
`docs/species-onboarding.md` and the fixture prose in `tests/migration/README.md`
no longer describe enum-gated anatomy.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after it integrates this commit)

### The one number that fails

**R6's native half does not hold.** The oak's native frame measures a total p50
of **2.3043 ms** against a 2 ms budget, valid verdict, 0.30 ms over. Nothing was
tuned to chase it, per R6's own error clause. The browser half does hold: 999
frames in ten seconds, wall p95 10.10 ms against 16.7 ms and worst frame 10.20 ms
against 33 ms. The cause is the crown, not the renderer or the browser: fn-23's
oak carried 555,204 retained leaves and this one carries 869,310, fifty-seven per
cent more, while a leaf got sixteen per cent cheaper (2.55 ns against 3.05 ns).
The browser is faster than it was in fn-23 on the same measurement. The oak's
foliage count is a preset row; moving it, accepting 2.3 ms, or opening the
aggregation spec is the owner's call and the report states all three without
taking any.

### Numbers the owner should see

| | fn-23 | fn-24 |
|---|---:|---:|
| oak native together p50 | 1.750 ms | **2.304 ms** |
| oak browser together p50 | 2.275 ms | 2.108 ms |
| oak browser orbit wall p95 / worst | 10.10 / 10.10 ms | 10.10 / 10.20 ms |
| oak retained leaves | 555,204 | 869,310 |
| spruce browser vegetation p50 | 29.434 ms | 13.386 ms |

Fidelity bands, retained leaves at seed 7, all five inside their band: ordinary
41,827 (10^4 to 10^6); oak 869,310, spruce 7,012,326, telperion 118,329,
laurelin 542,357 (10^5 to 10^7). **Ordinary sits a decade below the strategy's
fidelity track** on a twenty-four metre envelope, and telperion's 118,329 is only
just inside; both are properties of the colonizing rows after the one builder and
no preset was touched. Task 5 flagged this for the owner and it is in the report.

Species numeric protocol: 48 of 48 cases pass against the frozen fn19 profiles,
24 seeds per species. The seed-7 trees are ordinary members of their own
distributions (oak median 871,598 leaves, spruce median 7,158,192).

Transition record: oregon-white-oak to norway-spruce, seed 7, 1600 by 1000, 240
frames at 24 fps, encoder present, `transition.mp4` 13.3 MB, one run of 353
seconds. Frames are ignored through `.flow/evidence/fn24/transition/.gitignore`
so only the record and the video are committed.

### What was not run, and why

- `npm run species:qa`'s capture half (57 stills at 960 by 720) was skipped: its
  framing is not the acceptance's, the five hero stills came from the same binary
  at fn-23's framing, and nothing in this task judges the QA rig's images. Its
  numeric half was run in full.
- The spruce's native clock was not re-run - the spec's boundaries say R6 is the
  oak and there is no spruce native re-run in this spec. Its browser numbers came
  free with the oak's and are recorded.
- The frames ignore file lives inside `.flow/evidence/fn24/transition/` rather
  than the repository root, to stay inside this task's declared Touches.

### Images viewed

Two of the four the budget allows: `.flow/evidence/fn24/oregon-white-oak-hero.png`
and `.flow/evidence/fn24/norway-spruce-hero.png`, opened once each to confirm the
stills are trees at the hero pose and not blank or clipped frames. The oak reads
as a broad dome-crowned deciduous tree on a short bole; the spruce as a conical
single-leader conifer whose top fifth is a thin, near-bare leader spike. Neither
observation is a verdict - R3 and R4 are the owner's, and the pixel aids in the
report (oak SSIM 0.680 against fn-23, spruce SSIM 0.884 against fn-22) are aids,
not judgments.

### Terminal state

NEEDS_HUMAN by design. The task stays `in_progress`: three owner verdicts are
unwritten, and R6's native number is a separate stop that no verdict clears.

### Integration and the owner's decisions (conductor, 2026-09-10)

Cherry-picked onto the spec branch as de60008 after amending the video out of git (it stays on disk under `.flow/evidence/fn24/transition/`, ignored, per the project's evidence rules); the workspace commit c3c8bc9 is retired with the worktree. The owner viewed the four stills and the transition on a tailnet page and gave the three verdicts recorded in the spec: accept, accept, accept as the receipt, with the video's flicker and a scenic cut deferred to a separate spec. The owner then accepted the oak's 2.30 ms native frame as meeting R6 by override, with the 2 ms target moving to the later density or aggregation spec. Carried forward for that spec: the ordinary preset's 41,827 retained leaves and Telperion's 118,329, both low in their bands after the one builder.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: de60008
- Tests: worker baseline: cargo test --release --workspace green before any edit, worker: cargo test --release --workspace - green, worker: npm test - 65 passed, worker: npm run test:render - PASS on the hardware adapter (oak browser orbit 999 frames, wall p95 10.10 ms, worst 10.20 ms), worker: headless oak native timing and orbit into .flow/evidence/fn24/ (together p50 2.3043 ms, valid), worker: species numeric protocol 48 of 48 against the frozen fn19 profiles, worker: one transition render, 240 frames at 1600x1000, 353 s, mp4 assembled (video kept on disk, ignored in git per the evidence rules), conductor: owner verdicts recorded in the spec; R6 accepted by owner override at 2.30 ms
- PRs: