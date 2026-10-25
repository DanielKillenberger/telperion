# Continuous tree space: habit and leaf as traits

## Goal

The owner asked for a more unified, efficient generator in which every parameter applies across species and the branching model is a dial, not a hidden three-way habit enum (colonizing, spreading, tiered) with its own builder per variant. The standing requirement this spec first proved: a continuous tree space, where a template is a point in one multi-dimensional space and any two trees interpolate smoothly, leaves included. One scaffold builder driven by numeric architectural traits (Hallé and Oldeman: apical dominance, whorl strength, pitch, rise) replaces the three builders; the leaf anatomy and attachment enums dissolve into numeric traits; every growth unit of every axis passes through the one bias field, so the supernatural dials reach the oak and spruce.

## Decisions

- > user (turn 4): "Currently the parameters don't seem to apply across species and the branching model doesn't seem exposed (the enum that allows switching between normal, oak, spruce)." / "Does this need to be an enum or could it be more of a continuos parameter?"
- > user (recorded in fn-22, carried into fn-23): "it should be a preset of parameter numbers. not different way of building/rendering. The generator should be able to generate the trees if you set the right values. The presets should only store those values."
- > user (edit cycle 1): "I don't need to have things to be byte identical. Ideally the generator improves all models over time so they're allowed to look different."
- > user (edit cycle 1): "This should be be made a permanent req that we're stepping through continuous tree space. We want smooth interpoloation between any kind of tree. A template is just a point in that multi dimensional space. This should probably go into strategy."
- > user (edit cycle 1): "I think one cool thing that could come out as a receipt is a rendered video that transitions smoothly between the oak and the spruce, demonstrating the continuous tree space. Incorporate this but keep the spec lean."
- > user (edit cycle 2): "We need continouuity for the leaves also to achieve continuous interpolation between the two trees. I don't see that as part of the spec?"
- > user (plan, 2026-09-10): "fn-24 standard depth no review"
- Continuous traits over an exposed enum; one builder over three; colonization becomes an attractor-pull term in one heading sum (rule + attractor + bias field); per-axis random streams so a trait step reshuffles only its own descendants.
- Rejected: a compatibility shim for old kind tags (closed schema rejects them); a frame-capture module or encoder dependency (the headless target loops its PNG writer and calls the system encoder when present).
- Earlier trees are not preserved; pinned scaffold hashes re-recorded once, stills judged as species, never identity.

## Acceptance Criteria

- **R1:** Habit is a table of numeric traits on every family, no kind field; one scaffold builder, one set of local rules, no habit branch — met (`habit.rs` deleted; fifteen traits).
- **R2:** One step of any habit trait or bias term changes the oak, spruce and ordinary skeleton hashes, asserted without a device — met.
- **R3:** Owner judges oak and spruce hero stills against fn-23's oak and fn-22's spruce as species, not identity — met (accepted 2026-09-10).
- **R4:** Ten linear steps between every preset pair validate and generate; leaf counts in band; oak-to-spruce video judged by the owner — met (accepted 2026-09-10).
- **R5:** Harness shows every habit, element and attachment trait on every preset; renderer untouched; reproducible from seed — met.
- **R6:** fn-23's oak native timing and browser orbit re-run; oak holds 2 ms p50 and 60 fps — partial: 2.3043 ms, accepted by owner override.
- **R7:** Leaf element and attachment are numeric traits with no enum; one-step changes move element/placement hashes; no switch frame in the transition — met.

## Result

- One builder from the trait table; calibrated rows deviated from the starting table where the node ceiling demanded (oak lateral spacing 1.6 m and 3 orders; spruce 0.15 m and 4 orders, 91,331 nodes; Telperion and Laurelin their own row).
- One outline routine: lobe count/depth and section roundness; the oak margin uses a sixth-power sinus term after the owner asked for broader lobes and narrower sinuses. One placement path: forward lean, lean rise, surface contact; oak and spruce placement hashes unchanged.
- `blend::families(a, b, t)` and `tests/sweep.rs`: every pair, ten steps, no switch frame; headless `--to`/`--frames` renders the transition (240 frames, 24 fps).
- Oak native together p50 2.304 ms (fn-23: 1.750), browser orbit wall p95 10.10 ms, worst 10.20 ms; retained leaves oak 869,310 (fn-23: 555,204), spruce 7,012,326, ordinary 41,827 (a decade below the fidelity track, flagged), telperion 118,329, laurelin 542,357. Species protocol 48 of 48 against the frozen fn19 profiles.
- Owner verdicts (2026-09-10):
  - Oak: Accept. "to me it looks good." Given on the four stills and the video together; the pointed leaf-40 blade felt closer to oak than the broad-lobe pass that shipped; further margin work is a later foliage spec.
  - Spruce: Accept. "to me it looks good."
  - Transition video: Accept as the receipt. "to me it looks good. The noise bothers me but that's a separate spec."
  - R6 owner override: Accept 2.30 ms; the 0.30 ms over comes from the denser crown while each leaf got cheaper; the 2 ms target moves to the later density or aggregation spec.

## Open

- Frame-to-frame flicker of a re-grown crown and a scenic cut for X (became fn-25).
- The 2 ms oak target moved to a density or aggregation spec; Ordinary's leaf count below the fidelity track left to the owner.

## Evidence

- `.flow/evidence/fn24/oregon-white-oak-hero.jpg`, `.flow/evidence/fn24/norway-spruce-hero.jpg` (verdict stills, seed 7, 1600x1000)
- `.flow/evidence/fn24/ordinary-hero.png` (loaded by `crates/telperion-render/tests/look.rs`)
- `.flow/evidence/fn24/transition/transition.json` (the transition record; the video is ignored)
