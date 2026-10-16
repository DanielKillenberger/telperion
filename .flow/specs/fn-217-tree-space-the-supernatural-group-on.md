# Tree space: the supernatural group on the engine path

## Goal

fn-10's supernatural group (off by default, on in legendary presets) on the engine path, with what the Two Trees need: scale, two-sided leaf colour with an edge term, emitted light from flowers with a time cycle (two trees in counterphase, mingling at the crossover) and a falling dew of light. Owner, 2026-10-07, lifts 2026-09-14's "leave luminosity for later" for the Two Trees (full creative freedom). Also carries fn-216's blossom follow-ups (host, 2026-10-08): florets read as flat white scraps, and blossom vanishes at whole-tree distance.

## Decisions

Full record in `DESIGN.md` sections 6-10. Host, 2026-10-08:
- Release lens: the old Telperion and Laurelin never glowed, so light is not release parity. Split into (a) scale and leaves and (b) floret form and blossom at distance, for the release; (c) emitted light, cycle, dew and two trees in one frame, after the release. fn-218 and fn-219 depend on (a) and (b).
- **X2 extended** (Astra P1, all of Astra's findings taken): every legendary visual (flowering share, `/bloom`, leaf front/back/margin colours, reflectances, scale) is a `/supernatural` override applied only while `enabled`; off is the base species by hash and sheet.
- **S1:** scale every position and length once, after the natural conversion and before folds, through the common input; orientations unchanged; tested at 0.1, 1, 100.
- **E2** back and margin reflectance, neutral at zero, none on the organ; **F1+F2+F3** floret eye, stamens, organ lit as a surface; **M0 first** then M1 or M2 by measurement; writhe (W1) not ported.
- After task 2: seated-leaf trees over 64 m on the device stay a named refusal; the rim becomes a leaf-space fraction with area-weighted sub-pixel mix; bark metre rows scale with S1; contact precision left to fn-218/fn-219.
- Horn gap: add `/bloom/petalCurl` (positive flares, 0 byte-identical); raceme by values first.
- Blossom at whole-tree distance moved to **fn-234** (M0 rules out sampling, M1 values reach only 2.9 % of the frame).

## Acceptance Criteria

- **R1:** The group's design, every trait dormant at neutral, host decision and Astra review before building — met (DESIGN.md).
- **R2:** Each trait on a test tree, stills (and a time strip for the light cycle), viewed — met for scale, leaves, florets, horn form and racemes; blossom at distance moved to fn-234; the light cycle moved to the post-release light spec.
- **R3:** Every passed species unchanged with the group off; frame cost of the light measured — met for off-identity; light cost moves with the light spec.

## Result

- Off-identity: mesh hash equals the base species'; `space_hash` 16/16 identical at 33bc84cf and HEAD; neutral stills byte-identical to pre-E2.
- S1: device matches CPU at scales 0.1 and 100 (3,546 nodes, 700 runs). Contact precision worst error at scale 1: 34 um (0.04 % of the smallest leaf); at 100: 3.4 mm (4.0 %).
- Rim at distance (area-weighted): width 0.1 near 0.115 / far 0.117; width 0.5 near 0.709 / far 0.712. Laurelin's test tree reads green with gold edges at whole-tree view.
- M0: sampling keeps 1.004-1.005 of the reference; organ levels keep 92-96 % of the finest coverage, so M1 (values).
- Stills viewed by the host: `petalCurl` 0.6 gives a throat opening into a flared mouth; racemes hang as strands at middle distance (close up at share 1 a dense curtain); blossom now a five-petal flower with eye and stamens; margin change on beech and oak invisible (0.03-0.15 % pixels), accepted under the generator-evolution policy.

## Open

- Blossom at whole-tree distance: fn-234. Emitted light, cycle, dew, two trees in one frame: post-release light spec.
- Raceme colour is ochre where it should be golden: a values question for fn-219.
- The birch's margin is unchecked until the birch has an engine species.
- A local-origin ring position step when a tall seated species needs it (GPU ring domain 64 m).
- A debug-profile overflow in `telperion-space/src/shed.rs` predates this spec.

## Friction

- No helper names where a drawn floret stands, and a low flowering share shrinks florets rather than thinning them (five still rounds) -> proposed a drawn-floret position helper; the share behaviour raised as a product question.
- The GPU lock held by a deliberate 45-minute sleep -> proposed an owner and end time noted beside a deliberate hold, or a second lock for short GPU tests.

## Evidence

- `.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md` (the design and host decisions; cited by code and presets)
