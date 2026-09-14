---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-31-growth-rule-sapling-form-thickening-by.1 Implement Growth rule: sapling form, thickening by age, a shedding floor

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## Blocker — NEEDS_HUMAN, 2026-09-14

R1 and R2 stop on their own rules: both need the owner's recorded judgment, so `flowctl done` was not run. The implementation is complete on branch `fn-31-growth-rule-sapling-form-thickening-by`, seven commits `ccb44eaf..a312292` by one codex session (gpt-6-astra, high), and every gate is green on `a312292` as the host re-ran them: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --release --workspace` (322 passed, 0 failed, 7 ignored; the four renderer tests fn-30 left red are green with their assertions, tolerances, cameras and shaders unchanged), `npm test` (77 of 77 including native-to-wasm parity), `npm run typecheck`. The rule changed inside fn-11's slice as ten numeric growth traits every family carries and the blend walks (`vigourFloor`, `juvenileRadius`, `thickeningDelay`, `thickeningShape`, `crownBaseRetention`, `seedlingHeight`, `shootStep`, `juvenileBranching`, `juvenileHeight`, `seedlingRadius`), with no species branch; the authored 0.45 threshold is back on Ordinary and the Two Trees and their mature crowns hold (16,066, 99,128 and 151,220 nodes at 173 years) while a shaded shoot still dies as a stamp at year 174, asserted by `survival_tests.rs`. The measured height-to-diameter ratio falls with age (oak 135 to 131 to 101, spruce 48.9 to 45.9 to 45.3) and trunk diameter lands within 15 percent of fn-30's composed references at all six fit ages (oak -10.9, +12.8, -6.2 percent at 26.7, 56.1 and 112 years; spruce -5.9, +13.6, +0.6 percent at 14.1, 26.6 and 36.9 years). The pins moved once after `.flow/evidence/fn31/CONVERGENCE.md` recorded the populations; the idle mature build cost is 612 ms oak and 717 ms spruce against fn-30's 2,463 and 867 ms ceilings. Decisions the owner must record in `.flow/evidence/fn31/REPORT.md` under `## Owner verdict` (three slots): R1, whether the oak and spruce strips beside fn-30's (`oregon-white-oak-strips-beside-fn30.png`, `norway-spruce-strips-beside-fn30.png`, year one framed to its own size) read as saplings that continue into the same tree, with one named deviation to judge on the mature stills (`*-beside-fn30.png`): the mature oak is far sparser than fn-30's, 19,513 nodes and 132,875 placements against 196,901 and 1,175,265, and Ordinary and Telperion lost 64 and 48 percent of their nodes, which the report attributes to retained structural buds, juvenile steps and the restored threshold; and R2, whether the composed young-age diameter references are the ones the owner accepts (the parked unknown; Stein's mature open-grown figures are the alternative anchor), since the tolerance is met only against that composition. fn-30's rejecting R3 slots are re-judged from this report. Host review of the range: the two fixture-value changes keep their tests' intent (`bark_detail`'s cap 400 to 40,000 only bounds work so the trunk reaches the unchanged 0.16 m relief scale; `conformance` moves four later-added appearance rows off their bounds under the same jitter, the fixture's existing pattern) and every assertion is unchanged; `audit.rs` was already 464 lines at the base. A second codex session for fn-29 shared the GPU during the run; the cost was re-measured idle and both numbers are in the report.

## Round 3 — codex quota exhausted mid-round, 2026-09-14 12:07Z

The owner reviewed round 2's strips and the live harness and did not accept R1: the ordinary tree was a leafed seedling at one year and a bare stick at two, the ten-year oak a stem coated in leaves with no laterals, the young spruce unchanged in kind, and the mature oak had lost ninety percent of its wood. Round 3 went into the same codex session with those four findings and a mature-density bound. Codex landed one checkpoint (8ff0b8c: yearly foliage retained on every preset, woody sapling laterals, primary extension planned before secondary thickening, crown room anchored at the supporting station, a taper inversion in the spruce-to-Telperion blend fixed) and was writing the second, the re-render, the re-pins and the report, when its usage limit hit; the limit resets 2026-09-19 16:27. The host committed that partial unit as it stood. On the new strips the oak reads as a leafed sapling at ten and twenty-six years and its mature crown is back to fn-30's density; the spruce at five and fourteen years is still a bare stem and a bare scaffold. Two blockers stand beside the owner's R1 and R2 verdicts: the oak bark-distance fixture still misses at 4x (mean 3.9764/255 against 3.0), so the workspace test gate is red, and the clean idle mature oak build costs 6,886 ms against fn-30's 2,463 ms ceiling, an R5 miss the rule change caused; the spruce builds in 718 ms. The next codex round, when quota returns, owns the spruce young ages, the distance fixture cause, and the oak build cost. Session id in .flow/tmp/fn-31.1-codex-session.

## Round 4 — NEEDS_HUMAN, 2026-09-14

Claude Opus implemented this round in the conductor's checkout; the Codex bridge
is out until its quota resets on 2026-09-19. Three items were open and all three
were taken to a measured end; one is fixed, one is diagnosed with its fix
measured and withheld, one is cut by 29 percent and still misses. `flowctl done`
was not run: R1 and R2 still need the owner's recorded judgment, R5 still misses
its ceiling and the oak distance fixture is still red, so the workspace test gate
is red. Commits `5556f8f..HEAD` on branch
`fn-31-growth-rule-sapling-form-thickening-by`; the round's evidence is in
`.flow/evidence/fn31/REPORT.md` under `## Round 4`, with the convergence record
in `CONVERGENCE.md`, the gates in `round4/gates.json` and the capture hashes in
`CAPTURE.json`.

**The young spruce is fixed, as two numeric rows.** The five-year spruce was
bare because `visible()` fills a shoot's needle stations over `leafLifetime`
annual cohorts and the spruce authored 6.0, so every shoot on a young tree
showed a fraction of its needles while the mature tree, whose shoots are all
past six years, looked right. Nothing in the rule expires a cohort, so the trait
is a fill schedule and not a retention window; a spruce shoot flushes in one
season and the spruce joins every other preset at 1.0. `shootStep` falls from
the 0.2 default to 0.08 so a juvenile's internode is its own shoot rather than a
fifth of its height. Year five carries 1,095 needle placements against 184 and
172 nodes against 44; year 14.1 carries 252,407 against 78,487. The oak is
untouched by both rows and its identity pin is byte-identical, so 1, 10 and 26.7
years are the same tree the owner has already seen. One deviation is named for
the owner: at five years the spruce is a needled leader with short laterals, not
a whorled cone, because a whorl waits on 0.9 m of leader growth and a lateral
below the authored crown base finds no planning room. Both measured ways to
raise those whorls pull the 14.1-year trunk diameter to -13.0 and -21.3 percent
against the composed reference, the second outside R2, so the diameters were
kept.

**The oak distance fixture has its cause and a measured fix the owner must
weigh.** The planner reads the pipe allocation divided by the year's secondary
scale and writes each radius multiplied by it again. That cancels for a
pipe-derived width but not for `twig.diameter`, an anatomical width in metres,
so every shoot born while the oak's `juvenileRadius` holds the scale at 0.21 is
written at a fifth of its own twig anatomy and the birth ratio carries that fifth
to maturity — the wood in the fixture's mask has a median radius of 0.0035 m
where fn30's is 0.0106 m, which is sub-pixel at 4x. Flooring new wood at its twig
anatomy takes the fixture from 3.976417 mean / 14.50 p95 to 3.074167 / 11.75 at
4x and from 2.019375 / 7.75 to 1.317708 / 4.75 at 2x, p95 inside and mean 2.5
percent over. It was not shipped: wood born five times thicker grows five times
thicker, and the mature oak's radius keyframes go from 7.15 to 18.6 million with
the idle build at 9.46 s, which is R5 three times worse. Two further findings
belong with that choice. The fixture's mask, its own comment says, is a trunk
strip that excludes the silhouette; on fn30's geometry it is solid trunk and on
this rule it holds the trunk's right silhouette edge, open background and
branches crossing in front — the trunk is the same thickness, its path differs.
And with the bark relief zeroed the 4x mean is 2.078 of the 3.074, so two thirds
of what is left after the fix is geometry edges, not the bark field the test
exists to measure.

**R5 is cut but not met.** The keyframe eligibility index paid a B-tree insert
and a fresh allocation for every annual radius frame of every shoot, 5.2 million
of them on the mature oak; it is an append-only vector sorted on the first read
after a run of appends, stable, so a reader sees the order the map gave. The
idle oak median falls from 6,885.527 ms to 4,883.112 ms against the 2,463 ms
ceiling with byte-identical geometry, and the spruce to 653.491 ms inside its
867 ms. The remaining cost is not an implementation choice: the annual keyframe
stage is 3,266 ms of 4,669 and its frame count is total radial growth divided by
the 0.0001 m resize tolerance, which R2's secondary thickening multiplies over
the growth fn30 never had. Raising the tolerance is editing a tolerance to pass a
gate; thickening less fails R2. The third lever is the recommendation and is a
spec of its own: a local shoot's width is already a pure function of its
supporting structural width and its birth ratio and floors, so 6.6 of the 7.15
million frames are derivable and need never be materialized — but moving them to
the read path moves the change-record contract with them.

Decisions for the owner, all in `REPORT.md` under `## Owner verdict`: R1, whether
the two strips beside fn30's read as saplings that continue into the same tree,
with the five-year spruce's missing whorl named above; R2, which young-age
diameter reference is accepted; and the new fork, whether the oak's twig anatomy
is fixed at the cost of R5, or R5 is pursued through the derived-width spec and
the fixture stays red until then.

## Owner verdict on round 4 — R1 not yet accepted, 2026-09-14

The owner judged the round-4 strips on the served page: "It's a big improvement but not 100% happy." The finding is stem form: "the spruce sapling is just a straight stick with needles. Also the second picture. is just a straight pole. having some bend in them or something also for the oak. Saplings are thin and bend in gravity no?" Next round, in the rule: sapling stems carry curvature, a lean off vertical along the stem, a leader tip that nods under its own foliage, and thin laterals that droop, as numeric traits on every family; round 3 scales crookedness down with the growth step, which removes bend at seedling scale and is the first place to look. Judged on the same strips at 1, 5 and 14.1 years for the spruce and 1, 10 and 26.7 for the oak. R2 and the twig-anatomy fork remain open beside it.

## Owner decision — production maturity, 2026-09-14

The derived mature age is the year the last growth quantum lands on an asymptotic curve, 432 years for the oak against an envelope reached at 112. The owner agreed ("that seems very reasonable") to redefine production maturity as the first year the height fraction reaches 99 percent of the envelope, documented and never authored, so the default oak builds near 150 years, looks the same, and pays roughly a third of the slices. Goes into the next round with the stem-form findings, with one re-pin after convergence is recorded.

## Owner verdict on round 5 — two implementers, 2026-09-14

Round 5 ran the same brief on Grok 4.6 high fast (cursor-agent, branch fn-31-round5-grok, 12 commits) and Claude Opus (branch fn-31-round5-opus, 3 commits) from 06111b7. Owner: "the grok spruce is good and opus oak is better but not great." Grok's branch is the base: its spruce (whorls at five years, lean and a nodding top in the tiers), its build cost (mature oak 1,436 ms, under the 2,463 ms ceiling) and its green bark-distance fixture stand. Its oak does not: the mature crown narrowed to a vase about 30 percent narrower than fn-30's, and the ten-year oak is still a straight leaf-coated pole. Opus's oak is the reference for the crown (dense, rounded, fn-30's width, plus 11 percent) but is not accepted either: the ten-year oak is a shrub with no visible stem and nothing leans. Next round, on Grok's branch: the oak's rounded crown at fn-30's width from 26.7 years on, a ten-year oak with a visible leaning stem and woody laterals, and the spruce diameter at 14.1 years back inside tolerance (it misses by 23 percent). Opus's branch is kept until then as the reference and deleted after.

## Owner verdict on round 6 — 2026-09-15

Round 6 (Grok on fn-31-round5-grok, cab7655..3e07f0e) restored the mature oak crown at fn-30's width, gave the ten-year oak a leaning stem with woody laterals, and brought the spruce 14.1 y diameter inside tolerance; the spruce stands as accepted in round 5. The owner judged the strips and the live harness and did not accept the oak. Owner: "we probably need the ability to have sprouts and smaller leaves? just having full grown leaves makes it look bad. But then also at 67 the oak still looks like a vase i thought we'd be fixing this?" Round 7, on the same branch: leaf size as a function of shoot and tree age, so a seedling and a sapling carry sprouts and smaller leaves that reach full size with age, as a numeric trait every family walks; the oak crown rounded and broad at every age from 26.7 years to maturity, judged on a strip that adds 40 and 67 years so the middle ages cannot hide, with the 26.7 y umbrella included in that; the bark-distance fixture back to green. Open beside it: the clay look pin re-record, a host action.

## Owner decision after round 6 — 2026-09-15

The owner saw the candidate at seed 1 and 22.75 years in the harness: an inverted-cone crown on a bell-shaped trunk base, and called it "an awful regression." Leaf size and sprouts move to their own spec (captured today) so fn-31 finishes structural integrity first. Round 7 runs on Claude Opus on the candidate branch fn-31-round5-grok: shape invariants as red-first tests across seeds 1, 7 and 42 at every whole year (crown widest around mid-height and never at the top for the decurrent oak; trunk radius never increasing from the ground up; foliage every year; leaf and needle counts within a band of fn-30's mature tree), then the oak crown revised from Opus's round-5 rule so it holds at every age and seed, the trunk-base bell removed, and the bark-distance fixture green; the strip gains a seed-1 row and the ages 40 and 67.
